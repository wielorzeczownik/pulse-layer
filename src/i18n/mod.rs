pub mod en;
pub mod pl;

pub struct Strings {
  // App
  pub app_title: &'static str,

  // Scan view
  pub scan_start: &'static str,
  pub scan_stop: &'static str,
  pub scan_searching: &'static str,
  pub btn_connect: &'static str,

  // Connected view
  pub unknown_device: &'static str,
  pub zone_calm: &'static str,
  pub zone_normal: &'static str,
  pub zone_high: &'static str,
  pub zone_fast: &'static str,
  pub zone_alarm: &'static str,
  pub bpm: &'static str,
  pub btn_disconnect: &'static str,

  // Settings view
  pub settings_title: &'static str,
  pub btn_close: &'static str,
  pub label_bg: &'static str,
  pub label_indicator_color: &'static str,
  pub label_style: &'static str,
  pub style_heart: &'static str,
  pub style_pulse: &'static str,
  pub label_preview: &'static str,
  pub hex_placeholder: &'static str,

  // Status messages (for main.rs update logic)
  pub status_initial: &'static str,
  pub status_scanning: &'static str,
  pub status_stopped: &'static str,
  pub status_connecting: &'static str,
  pub status_connected: &'static str,
  pub status_disconnecting: &'static str,
  pub status_disconnected: &'static str,
  pub status_error: &'static str,
  pub overlay_hint: &'static str,
}

pub fn load() -> &'static Strings {
  let locale = sys_locale::get_locale().unwrap_or_default();
  if locale.starts_with("pl") {
    &pl::PL
  } else {
    &en::EN
  }
}
