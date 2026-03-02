use std::sync::{Arc, Mutex};
use tokio::sync::{
  mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
  watch,
};

use iced::{Element, Subscription, Task, Theme};

use crate::ble;
use crate::config;
use crate::constants::PORT;
use crate::i18n;
use crate::i18n::Strings;
use crate::server;
use crate::settings::AppSettings;
use crate::types::{BleCmd, BleEvent, DeviceInfo, Message, Screen};
use crate::ui;

pub struct App {
  cmd_tx: UnboundedSender<BleCmd>,
  cmd_rx: Arc<Mutex<Option<UnboundedReceiver<BleCmd>>>>,
  evt_tx: UnboundedSender<BleEvent>,
  evt_rx: Arc<Mutex<Option<UnboundedReceiver<BleEvent>>>>,
  hr_tx: Arc<watch::Sender<Option<u8>>>,
  settings_tx: Arc<watch::Sender<AppSettings>>,

  pub lang: &'static Strings,
  pub screen: Screen,
  pub devices: Vec<DeviceInfo>,
  pub status: String,
  pub scanning: bool,
  pub connected_id: Option<String>,
  pub connected_name: Option<String>,
  pub heart_rate: Option<u8>,

  pub settings: AppSettings,
  pub settings_open: bool,
}

impl App {
  pub fn new() -> Self {
    let (cmd_tx, cmd_rx) = unbounded_channel();
    let (evt_tx, evt_rx) = unbounded_channel();
    let (hr_tx, hr_rx) = watch::channel(None::<u8>);

    let loaded = config::load();
    let (settings_tx, settings_rx) = watch::channel(loaded.clone());

    server::spawn(hr_rx, settings_rx);

    let lang = i18n::load();

    Self {
      cmd_tx,
      cmd_rx: Arc::new(Mutex::new(Some(cmd_rx))),
      evt_tx,
      evt_rx: Arc::new(Mutex::new(Some(evt_rx))),
      hr_tx: Arc::new(hr_tx),
      settings_tx: Arc::new(settings_tx),

      lang,
      screen: Screen::Scanning,
      devices: Vec::new(),
      status: format!(
        "{}  {} http://localhost:{}",
        lang.status_initial, lang.overlay_hint, PORT
      ),
      scanning: false,
      connected_id: None,
      connected_name: None,
      heart_rate: None,

      settings: loaded,
      settings_open: false,
    }
  }

  pub fn title(&self) -> String {
    self.lang.app_title.to_string()
  }
  pub fn theme(&self) -> Theme {
    Theme::Dark
  }

  pub fn subscription(&self) -> Subscription<Message> {
    ble::ble_subscription(
      self.cmd_rx.clone(),
      self.evt_rx.clone(),
      self.evt_tx.clone(),
    )
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    let lang = self.lang;
    match message {
      Message::StartScan => {
        let _ = self.cmd_tx.send(BleCmd::StartScan);
        self.scanning = true;
        self.status = lang.status_scanning.to_string();
      }
      Message::StopScan => {
        let _ = self.cmd_tx.send(BleCmd::StopScan);
        self.scanning = false;
        self.status = lang.status_stopped.to_string();
      }
      Message::ConnectSelected(id) => {
        let _ = self.cmd_tx.send(BleCmd::Connect(id));
        self.scanning = false;
        self.status = lang.status_connecting.to_string();
      }
      Message::Disconnect => {
        let _ = self.cmd_tx.send(BleCmd::Disconnect);
        self.status = lang.status_disconnecting.to_string();
      }
      Message::OpenSettings => {
        self.settings_open = true;
      }
      Message::CloseSettings => {
        self.settings_open = false;
      }
      Message::SetZoneHex(zone, hex) => {
        self.settings.set_zone_hex(zone, hex);
        config::save(&self.settings);
        let _ = self.settings_tx.send(self.settings.clone());
      }
      Message::SetOverlayStyle(style) => {
        self.settings.overlay_style = style;
        config::save(&self.settings);
        let _ = self.settings_tx.send(self.settings.clone());
      }
      Message::SetPanelBg(hex) => {
        self.settings.panel_bg_hex = hex;
        config::save(&self.settings);
        let _ = self.settings_tx.send(self.settings.clone());
      }
      Message::BleEvent(event) => match event {
        BleEvent::ScanUpdate(list) => {
          self.devices = list;
        }
        BleEvent::Connected(id, name) => {
          self.connected_id = Some(id);
          self.connected_name = Some(name);
          self.scanning = false;
          self.screen = Screen::Connected;
          self.status = format!(
            "{}  {} http://localhost:{}",
            lang.status_connected, lang.overlay_hint, PORT
          );
        }
        BleEvent::Disconnected => {
          self.connected_id = None;
          self.connected_name = None;
          self.heart_rate = None;
          let _ = self.hr_tx.send(None);
          self.screen = Screen::Scanning;
          self.status = lang.status_disconnected.to_string();
        }
        BleEvent::HeartRate(hr) => {
          self.heart_rate = Some(hr);
          let _ = self.hr_tx.send(Some(hr));
        }
        BleEvent::Error(err) => {
          self.status = format!("{}: {err}", lang.status_error);
        }
        BleEvent::Status(s) => {
          self.status = s;
        }
      },
    }
    Task::none()
  }

  pub fn view(&self) -> Element<'_, Message> {
    if self.settings_open {
      return ui::settings_view::settings_view(self);
    }
    match self.screen {
      Screen::Scanning => ui::scan_view::scan_view(self),
      Screen::Connected => ui::connected_view::connected_view(self),
    }
  }
}
