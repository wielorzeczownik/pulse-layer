use anyhow::{Result, anyhow};
use btleplug::api::{
  Central, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Manager, Peripheral};
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio::time::{Instant, interval_at};

use super::protocol::{build_cmd, parse_cmd};
use crate::constants::{
  CMD_START_HEART_RATE, CMD_STOP_HEART_RATE, KEEP_ALIVE_SECS, MAX_VALID_BPM, MIN_RR_INTERVAL_MS,
  MIN_VALID_BPM, RR_TO_BPM_MS, SCAN_REFRESH_SECS, UUID_READ, UUID_WRITE,
};
use crate::types::{BleCmd, BleEvent, DeviceInfo};

pub async fn ble_worker(mut cmd_rx: UnboundedReceiver<BleCmd>, evt_tx: UnboundedSender<BleEvent>) {
  let adapter = match init_adapter(&evt_tx).await {
    Some(a) => a,
    None => return,
  };

  let mut scanning = false;
  let mut scan_interval = tokio::time::interval(Duration::from_secs(SCAN_REFRESH_SECS));
  let mut connected: Option<Peripheral> = None;
  let mut write_char: Option<Characteristic> = None;
  let mut notify_task: Option<JoinHandle<()>> = None;
  let mut realtime_task: Option<JoinHandle<()>> = None;

  loop {
    tokio::select! {
        cmd = cmd_rx.recv() => {
            match cmd {
                Some(BleCmd::StartScan) => {
                    match adapter.start_scan(ScanFilter::default()).await {
                        Ok(_) => {
                            scanning = true;
                            let _ = evt_tx.send(BleEvent::Status("Scanning...".to_string()));
                        }
                        Err(e) => {
                            let _ = evt_tx.send(BleEvent::Error(format!("Scan failed: {e}")));
                        }
                    }
                }
                Some(BleCmd::StopScan) => {
                    scanning = false;
                    let _ = adapter.stop_scan().await;
                    let _ = evt_tx.send(BleEvent::Status("Scan stopped".to_string()));
                }
                Some(BleCmd::Connect(id)) => {
                    // clean up previous connection in case user reconnects without disconnecting first
                    do_disconnect(&mut connected, &mut write_char, &mut notify_task, &mut realtime_task).await;
                    scanning = false;
                    let _ = adapter.stop_scan().await;
                    let _ = evt_tx.send(BleEvent::Status("Connecting...".to_string()));

                    match connect_device(&adapter, &id, &evt_tx).await {
                        Ok((p, name, wc, nh, rth)) => {
                            let device_id = p.id().to_string();
                            write_char     = Some(wc);
                            connected      = Some(p);
                            notify_task    = Some(nh);
                            realtime_task  = Some(rth);
                            let _ = evt_tx.send(BleEvent::Connected(device_id, name));
                        }
                        Err(e) => {
                            let _ = evt_tx.send(BleEvent::Error(format!("Connect failed: {e}")));
                        }
                    }
                }
                Some(BleCmd::Disconnect) => {
                    do_disconnect(&mut connected, &mut write_char, &mut notify_task, &mut realtime_task).await;
                    let _ = evt_tx.send(BleEvent::Disconnected);
                }
                None => break,
            }
        }
        _ = scan_interval.tick(), if scanning => {
            let list = collect_devices(&adapter).await;
            let _ = evt_tx.send(BleEvent::ScanUpdate(list));
        }
    }
  }
}

// Send STOP before disconnecting — without it the ring keeps streaming and drains the battery.
async fn do_disconnect(
  connected: &mut Option<Peripheral>,
  write_char: &mut Option<Characteristic>,
  notify_task: &mut Option<JoinHandle<()>>,
  realtime_task: &mut Option<JoinHandle<()>>,
) {
  if let (Some(p), Some(wc)) = (connected.as_ref(), write_char.as_ref()) {
    let stop = build_cmd(CMD_STOP_HEART_RATE, &[0]);
    let _ = p.write(wc, &stop, WriteType::WithoutResponse).await;
    let _ = p.disconnect().await;
  }
  *connected = None;
  *write_char = None;
  if let Some(h) = notify_task.take() {
    h.abort();
  }
  if let Some(h) = realtime_task.take() {
    h.abort();
  }
}

async fn connect_device(
  adapter: &btleplug::platform::Adapter,
  id: &str,
  evt_tx: &UnboundedSender<BleEvent>,
) -> Result<(
  Peripheral,
  String,
  Characteristic,
  JoinHandle<()>,
  JoinHandle<()>,
)> {
  let peripherals = adapter.peripherals().await?;
  let needle = id.to_lowercase();

  let peripheral = peripherals
    .into_iter()
    .find(|p| p.id().to_string().to_lowercase() == needle)
    .ok_or_else(|| anyhow!("Device not found: {id}"))?;

  let name = peripheral
    .properties()
    .await
    .ok()
    .flatten()
    .and_then(|p| p.local_name)
    .unwrap_or_else(|| id.to_string());

  if !peripheral.is_connected().await? {
    peripheral.connect().await?;
  }
  peripheral.discover_services().await?;

  let chars = peripheral.characteristics();
  let write_char = chars
    .iter()
    .find(|c| c.uuid == UUID_WRITE)
    .cloned()
    .ok_or_else(|| anyhow!("write characteristic not found"))?;
  let read_char = chars
    .iter()
    .find(|c| c.uuid == UUID_READ)
    .cloned()
    .ok_or_else(|| anyhow!("notify characteristic not found"))?;

  peripheral.subscribe(&read_char).await?;
  peripheral
    .write(
      &write_char,
      &build_cmd(CMD_START_HEART_RATE, &[1, 0]),
      WriteType::WithResponse,
    )
    .await?;

  // Keep-alive: re-send START every KEEP_ALIVE_SECS so the ring doesn't stop streaming.
  let p_rt = peripheral.clone();
  let wc_rt = write_char.clone();
  let realtime_handle = tokio::spawn(async move {
    let mut interval = interval_at(
      Instant::now() + Duration::from_secs(KEEP_ALIVE_SECS),
      Duration::from_secs(KEEP_ALIVE_SECS),
    );
    loop {
      interval.tick().await;
      let _ = p_rt
        .write(
          &wc_rt,
          &build_cmd(CMD_START_HEART_RATE, &[1, 0]),
          WriteType::WithResponse,
        )
        .await;
    }
  });

  let mut notifications = peripheral.notifications().await?;
  let tx = evt_tx.clone();
  let notify_handle = tokio::spawn(async move {
    while let Some(data) = notifications.next().await {
      let Some((cmd, payload)) = parse_cmd(&data.value) else {
        continue;
      };

      if cmd == CMD_START_HEART_RATE && payload.len() >= 7 && payload[0] == 1 && payload[1] == 0 {
        // payload[0] == 1: streaming active, payload[1] == 0: measurement type is RR interval
        // payload[5..7]: RR interval in ms (LE u16) — time between heartbeats
        let rr_ms = u16::from_le_bytes([payload[5], payload[6]]) as u32;
        if rr_ms >= MIN_RR_INTERVAL_MS {
          let bpm = (RR_TO_BPM_MS / rr_ms) as u8;
          if is_valid_hr(bpm) {
            let _ = tx.send(BleEvent::HeartRate(bpm));
          }
        }
      }
    }
    // Stream ended naturally (ring out of range / off). When do_disconnect() aborts this task,
    // it's cancelled at the .await above, so the line below never runs — no double Disconnected event.
    let _ = tx.send(BleEvent::Disconnected);
  });

  Ok((peripheral, name, write_char, notify_handle, realtime_handle))
}

// Ring sends 0xEE (238) when it loses skin contact — discard that and anything out of human range.
fn is_valid_hr(bpm: u8) -> bool {
  (MIN_VALID_BPM..=MAX_VALID_BPM).contains(&bpm)
}

async fn init_adapter(evt_tx: &UnboundedSender<BleEvent>) -> Option<btleplug::platform::Adapter> {
  let manager = match Manager::new().await {
    Ok(m) => m,
    Err(e) => {
      let _ = evt_tx.send(BleEvent::Error(format!("Bluetooth manager error: {e}")));
      return None;
    }
  };

  let adapters = match manager.adapters().await {
    Ok(a) => a,
    Err(e) => {
      let msg = e.to_string();
      let err = if msg.to_lowercase().contains("permission") {
        "Bluetooth permission denied.".to_string()
      } else {
        format!("Adapter error: {e}")
      };
      let _ = evt_tx.send(BleEvent::Error(err));
      return None;
    }
  };

  match adapters.into_iter().next() {
    Some(a) => Some(a),
    None => {
      let _ = evt_tx.send(BleEvent::Error("No Bluetooth adapters found".to_string()));
      None
    }
  }
}

// Sorted by RSSI descending (closest device first), unnamed devices skipped.
pub async fn collect_devices(adapter: &btleplug::platform::Adapter) -> Vec<DeviceInfo> {
  let Ok(peripherals) = adapter.peripherals().await else {
    return Vec::new();
  };
  let mut list = Vec::new();
  for p in peripherals {
    let Some(props) = p.properties().await.ok().flatten() else {
      continue;
    };
    let name = props.local_name.unwrap_or_default();
    if name.is_empty() {
      continue;
    }
    list.push(DeviceInfo {
      id: p.id().to_string(),
      name,
      rssi: props.rssi,
    });
  }
  list.sort_by(|a, b| match (b.rssi, a.rssi) {
    (Some(br), Some(ar)) => br.cmp(&ar),
    (Some(_), None) => std::cmp::Ordering::Less,
    (None, Some(_)) => std::cmp::Ordering::Greater,
    (None, None) => a.name.cmp(&b.name),
  });
  list
}
