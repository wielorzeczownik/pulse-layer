use crate::settings::{OverlayStyle, ZoneKind};

#[derive(Debug, Clone)]
pub struct DeviceInfo {
  pub id: String,
  pub name: String,
  pub rssi: Option<i16>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
  Scanning,
  Connected,
}

#[derive(Debug, Clone)]
pub enum BleCmd {
  StartScan,
  StopScan,
  Connect(String),
  Disconnect,
}

#[derive(Debug, Clone)]
pub enum BleEvent {
  ScanUpdate(Vec<DeviceInfo>),
  Connected(String, String),
  Disconnected,
  HeartRate(u8),
  Error(String),
  Status(String),
}

#[derive(Debug, Clone)]
pub enum Message {
  // Scan
  StartScan,
  StopScan,
  ConnectSelected(String),

  // Connected
  Disconnect,

  // Settings
  OpenSettings,
  CloseSettings,
  OpenColorPicker(ZoneKind),
  OpenBgPicker,
  CloseColorPicker,
  SetZoneHex(ZoneKind, String),
  SetOverlayStyle(OverlayStyle),
  SetPanelBg(String),

  // BLE events
  BleEvent(BleEvent),
}
