pub mod protocol;
pub mod worker;

use futures::{SinkExt, StreamExt, channel::mpsc, future::pending, stream::BoxStream};
use iced::Subscription;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::constants::BLE_CHANNEL_BUFFER;
use crate::types::{BleCmd, BleEvent, Message};
use worker::ble_worker;

struct BleData {
  cmd_rx: Arc<Mutex<Option<UnboundedReceiver<BleCmd>>>>,
  evt_rx: Arc<Mutex<Option<UnboundedReceiver<BleEvent>>>>,
  evt_tx: UnboundedSender<BleEvent>,
}

// Hash by Arc pointer address so iced sees the same subscription identity every frame
// and doesn't restart the BLE worker on each render.
impl Hash for BleData {
  fn hash<H: Hasher>(&self, state: &mut H) {
    (Arc::as_ptr(&self.cmd_rx) as usize).hash(state);
  }
}

fn build_stream(data: &BleData) -> BoxStream<'static, Message> {
  let cmd_rx = data.cmd_rx.clone();
  let evt_rx = data.evt_rx.clone();
  let evt_tx = data.evt_tx.clone();

  iced::stream::channel(
    BLE_CHANNEL_BUFFER,
    move |mut output: mpsc::Sender<Message>| async move {
      // take() only succeeds on the first call
      // subsequent runs (iced retrying the stream)
      // get None and skip spawning a duplicate worker.
      let cmd_rx = { cmd_rx.lock().unwrap().take() };
      if let Some(rx) = cmd_rx {
        let tx = evt_tx.clone();
        tokio::spawn(async move {
          ble_worker(rx, tx).await;
        });
      }

      let evt_rx_taken = { evt_rx.lock().unwrap().take() };
      let mut evt_rx = match evt_rx_taken {
        Some(rx) => rx,
        None => {
          pending::<()>().await;
          unreachable!();
        }
      };

      loop {
        match evt_rx.recv().await {
          Some(event) => {
            let _ = output.send(Message::BleEvent(event)).await;
          }
          None => break,
        }
      }

      // Never return
      // iced would treat a finished stream as "subscription ended" and restart it.
      pending::<()>().await
    },
  )
  .boxed()
}

pub fn ble_subscription(
  cmd_rx: Arc<Mutex<Option<UnboundedReceiver<BleCmd>>>>,
  evt_rx: Arc<Mutex<Option<UnboundedReceiver<BleEvent>>>>,
  evt_tx: UnboundedSender<BleEvent>,
) -> Subscription<Message> {
  Subscription::run_with(
    BleData {
      cmd_rx,
      evt_rx,
      evt_tx,
    },
    build_stream,
  )
}
