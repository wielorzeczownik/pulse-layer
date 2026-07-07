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
use uuid::Uuid;

use super::protocol::{build_cmd, parse_cmd};
use crate::types::{BleCmd, BleEvent, DeviceInfo};

// Nordic UART Service
const UUID_WRITE: Uuid = Uuid::from_u128(0x6e40_0002_b5a3_f393_e0a9_e50e_24dc_ca9e);
const UUID_READ: Uuid = Uuid::from_u128(0x6e40_0003_b5a3_f393_e0a9_e50e_24dc_ca9e);

const CMD_START_HEART_RATE: u8 = 0x69;
const CMD_STOP_HEART_RATE: u8 = 0x6A;
const CMD_REALTIME_HEART_RATE: u8 = 0x1E;

const SCAN_REFRESH_SECS: u64 = 3;
const KEEP_ALIVE_SECS: u64 = 60; // some models stop streaming without a periodic re-trigger
const BPM_RETRIGGER_SECS: u64 = 5; // re-send START if no valid BPM received for this long

// Ring reports RR interval in ms; BPM = 60000 / rr_ms
const RR_TO_BPM_MS: u32 = 60_000;
const MIN_RR_INTERVAL_MS: u32 = 300; // below this → BPM > 200, discard

// Ring sends 0xEE when it loses skin contact — filter it out along with other non-human values
const MIN_VALID_BPM: u8 = 30;
const MAX_VALID_BPM: u8 = 200;

pub async fn ble_worker(mut cmd_rx: UnboundedReceiver<BleCmd>, evt_tx: UnboundedSender<BleEvent>) {
  let Some(adapter) = init_adapter(&evt_tx).await else {
    return;
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
                        Ok(()) => {
                            scanning = true;
                            let _ = evt_tx.send(BleEvent::Status("Scanning...".to_string()));
                        }
                        Err(err) => {
                            let _ = evt_tx.send(BleEvent::Error(format!("Scan failed: {err}")));
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
                        Ok((peripheral, name, write, notify, realtime)) => {
                            let device_id = peripheral.id().to_string();
                            write_char     = Some(write);
                            connected      = Some(peripheral);
                            notify_task    = Some(notify);
                            realtime_task  = Some(realtime);
                            let _ = evt_tx.send(BleEvent::Connected(device_id, name));
                        }
                        Err(err) => {
                            let _ = evt_tx.send(BleEvent::Error(format!("Connect failed: {err}")));
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
  if let (Some(peripheral), Some(write)) = (connected.as_ref(), write_char.as_ref()) {
    let stop = build_cmd(CMD_STOP_HEART_RATE, &[0]);
    let _ = peripheral
      .write(write, &stop, WriteType::WithoutResponse)
      .await;
    let _ = peripheral.disconnect().await;
  }
  *connected = None;
  *write_char = None;
  if let Some(handle) = notify_task.take() {
    handle.abort();
  }
  if let Some(handle) = realtime_task.take() {
    handle.abort();
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
    .find(|peripheral| peripheral.id().to_string().to_lowercase() == needle)
    .ok_or_else(|| anyhow!("Device not found: {id}"))?;

  let name = peripheral
    .properties()
    .await
    .ok()
    .flatten()
    .and_then(|props| props.local_name)
    .unwrap_or_else(|| id.to_string());

  if !peripheral.is_connected().await? {
    peripheral.connect().await?;
  }
  peripheral.discover_services().await?;

  let chars = peripheral.characteristics();
  let write_char = chars
    .iter()
    .find(|characteristic| characteristic.uuid == UUID_WRITE)
    .cloned()
    .ok_or_else(|| anyhow!("write characteristic not found"))?;
  let read_char = chars
    .iter()
    .find(|characteristic| characteristic.uuid == UUID_READ)
    .cloned()
    .ok_or_else(|| anyhow!("notify characteristic not found"))?;

  peripheral.subscribe(&read_char).await?;
  // Step 1: open the measurement session (StartHeartRateReq).
  peripheral
    .write(
      &write_char,
      &build_cmd(CMD_START_HEART_RATE, &[1, 0]),
      WriteType::WithResponse,
    )
    .await?;
  // Step 2: request first real-time BPM reading.
  peripheral
    .write(
      &write_char,
      &build_cmd(CMD_REALTIME_HEART_RATE, &[3]),
      WriteType::WithoutResponse,
    )
    .await?;

  let realtime_handle = spawn_keep_alive(peripheral.clone(), write_char.clone());
  let notify_handle = spawn_notifier(&peripheral, write_char.clone(), evt_tx.clone()).await?;

  Ok((peripheral, name, write_char, notify_handle, realtime_handle))
}

// Keep-alive: re-send RealTimeHeartRate(3) every KEEP_ALIVE_SECS so the ring doesn't stop streaming.
fn spawn_keep_alive(peripheral: Peripheral, write_char: Characteristic) -> JoinHandle<()> {
  tokio::spawn(async move {
    let mut interval = interval_at(
      Instant::now() + Duration::from_secs(KEEP_ALIVE_SECS),
      Duration::from_secs(KEEP_ALIVE_SECS),
    );
    loop {
      interval.tick().await;
      let _ = peripheral
        .write(
          &write_char,
          &build_cmd(CMD_REALTIME_HEART_RATE, &[3]),
          WriteType::WithResponse,
        )
        .await;
    }
  })
}

// Parse incoming notifications into BPM events, re-triggering measurement when no valid BPM
// arrives for BPM_RETRIGGER_SECS (ring finishes its measurement cycle).
async fn spawn_notifier(
  peripheral: &Peripheral,
  write_char: Characteristic,
  evt_tx: UnboundedSender<BleEvent>,
) -> Result<JoinHandle<()>> {
  let mut notifications = peripheral.notifications().await?;
  let peripheral = peripheral.clone();
  Ok(tokio::spawn(async move {
    let retrigger_after = Duration::from_secs(BPM_RETRIGGER_SECS);
    let mut last_bpm_at = tokio::time::Instant::now();

    loop {
      tokio::select! {
        data = notifications.next() => {
          let Some(data) = data else {
            // Stream ended naturally (ring out of range / off). When do_disconnect() aborts this task,
            // it's cancelled at the .await above, so the line below never runs — no double Disconnected event.
            let _ = evt_tx.send(BleEvent::Disconnected);
            break;
          };
          let Some((cmd, payload)) = parse_cmd(&data.value) else {
            continue;
          };
          if cmd == CMD_START_HEART_RATE && payload.len() >= 7 && payload[0] == 1 && payload[1] == 0 {
            // payload[5..7]: RR interval in ms (LE u16) — time between heartbeats
            let rr_ms = u32::from(u16::from_le_bytes([payload[5], payload[6]]));
            if rr_ms >= MIN_RR_INTERVAL_MS
              && let Ok(bpm) = u8::try_from(RR_TO_BPM_MS / rr_ms)
              && is_valid_hr(bpm)
            {
              last_bpm_at = tokio::time::Instant::now();
              let _ = evt_tx.send(BleEvent::HeartRate(bpm));
            }
          }
        }
        () = tokio::time::sleep_until(last_bpm_at + retrigger_after) => {
          // No valid BPM for BPM_RETRIGGER_SECS — reopen session then trigger a new measurement.
          let _ = peripheral
            .write(&write_char, &build_cmd(CMD_START_HEART_RATE, &[1, 0]), WriteType::WithoutResponse)
            .await;
          let _ = peripheral
            .write(&write_char, &build_cmd(CMD_REALTIME_HEART_RATE, &[3]), WriteType::WithoutResponse)
            .await;
          last_bpm_at = tokio::time::Instant::now();
        }
      }
    }
  }))
}

// Ring sends 0xEE (238) when it loses skin contact — discard that and anything out of human range.
fn is_valid_hr(bpm: u8) -> bool {
  (MIN_VALID_BPM..=MAX_VALID_BPM).contains(&bpm)
}

async fn init_adapter(evt_tx: &UnboundedSender<BleEvent>) -> Option<btleplug::platform::Adapter> {
  let manager = match Manager::new().await {
    Ok(manager) => manager,
    Err(err) => {
      let _ = evt_tx.send(BleEvent::Error(format!("Bluetooth manager error: {err}")));
      return None;
    }
  };

  let adapters = match manager.adapters().await {
    Ok(adapters) => adapters,
    Err(error) => {
      let msg = error.to_string();
      let err = if msg.to_lowercase().contains("permission") {
        "Bluetooth permission denied.".to_string()
      } else {
        format!("Adapter error: {error}")
      };
      let _ = evt_tx.send(BleEvent::Error(err));
      return None;
    }
  };

  if let Some(adapter) = adapters.into_iter().next() {
    Some(adapter)
  } else {
    let _ = evt_tx.send(BleEvent::Error("No Bluetooth adapters found".to_string()));
    None
  }
}

// Sorted by RSSI descending (closest device first), unnamed devices skipped.
pub async fn collect_devices(adapter: &btleplug::platform::Adapter) -> Vec<DeviceInfo> {
  let Ok(peripherals) = adapter.peripherals().await else {
    return Vec::new();
  };
  let mut list = Vec::new();
  for peripheral in peripherals {
    let Some(props) = peripheral.properties().await.ok().flatten() else {
      continue;
    };
    let name = props.local_name.unwrap_or_default();
    if name.is_empty() {
      continue;
    }
    list.push(DeviceInfo {
      id: peripheral.id().to_string(),
      name,
      rssi: props.rssi,
    });
  }
  list.sort_by(|left, right| match (right.rssi, left.rssi) {
    (Some(right_rssi), Some(left_rssi)) => right_rssi.cmp(&left_rssi),
    (Some(_), None) => std::cmp::Ordering::Less,
    (None, Some(_)) => std::cmp::Ordering::Greater,
    (None, None) => left.name.cmp(&right.name),
  });
  list
}
