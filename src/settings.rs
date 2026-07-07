use iced::Color;
use serde::{Deserialize, Serialize};

pub fn color_to_hex(color: iced::Color) -> String {
  let [red, green, blue, _alpha] = color.into_rgba8();
  format!("#{red:02X}{green:02X}{blue:02X}")
}

/// Parses "#RRGGBB" or "RRGGBB" into an iced Color. Falls back to gray on bad input.
pub fn parse_hex_color(hex: &str) -> Color {
  let trimmed = hex.trim_start_matches('#');
  if trimmed.len() == 6
    && let (Ok(red), Ok(green), Ok(blue)) = (
      u8::from_str_radix(&trimmed[0..2], 16),
      u8::from_str_radix(&trimmed[2..4], 16),
      u8::from_str_radix(&trimmed[4..6], 16),
    )
  {
    return Color::from_rgb8(red, green, blue);
  }
  Color {
    r: 0.6,
    g: 0.6,
    b: 0.65,
    a: 1.0,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum OverlayStyle {
  #[default]
  Heart,
  Pulse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoneKind {
  Calm,
  Normal,
  High,
  Fast,
  Alarm,
}

// Colors are hex strings ("#RRGGBB") zone color applies to glyph, number, and badge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
  pub zone_calm_hex: String,
  pub zone_normal_hex: String,
  pub zone_high_hex: String,
  pub zone_fast_hex: String,
  pub zone_alarm_hex: String,
  pub overlay_style: OverlayStyle,
  pub panel_bg_hex: String,
}

impl AppSettings {
  pub fn zone_hex(&self, zone: ZoneKind) -> &str {
    match zone {
      ZoneKind::Calm => &self.zone_calm_hex,
      ZoneKind::Normal => &self.zone_normal_hex,
      ZoneKind::High => &self.zone_high_hex,
      ZoneKind::Fast => &self.zone_fast_hex,
      ZoneKind::Alarm => &self.zone_alarm_hex,
    }
  }

  pub fn set_zone_hex(&mut self, zone: ZoneKind, hex: String) {
    match zone {
      ZoneKind::Calm => self.zone_calm_hex = hex,
      ZoneKind::Normal => self.zone_normal_hex = hex,
      ZoneKind::High => self.zone_high_hex = hex,
      ZoneKind::Fast => self.zone_fast_hex = hex,
      ZoneKind::Alarm => self.zone_alarm_hex = hex,
    }
  }
}

impl Default for AppSettings {
  fn default() -> Self {
    Self {
      zone_calm_hex: "#52C27A".to_string(),   // soft green
      zone_normal_hex: "#5B9BD5".to_string(), // calm blue
      zone_high_hex: "#E5B950".to_string(),   // warm amber
      zone_fast_hex: "#E07A30".to_string(),   // orange
      zone_alarm_hex: "#D94545".to_string(),  // red
      overlay_style: OverlayStyle::Heart,
      panel_bg_hex: "#0A0A12".to_string(),
    }
  }
}
