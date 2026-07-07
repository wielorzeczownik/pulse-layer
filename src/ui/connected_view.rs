use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};

use super::{helpers, style};
use crate::App;
use crate::settings::{ZoneKind, parse_hex_color};
use crate::types::Message;

// BPM zone upper bounds (Alarm = everything above FAST_MAX).
const BPM_ZONE_CALM_MAX: u8 = 64;
const BPM_ZONE_NORMAL_MAX: u8 = 80;
const BPM_ZONE_HIGH_MAX: u8 = 100;
const BPM_ZONE_FAST_MAX: u8 = 130;

// Lower bounds derived from upper bounds, used in match range patterns to avoid overlapping arms.
const BPM_ZONE_NORMAL_MIN: u8 = BPM_ZONE_CALM_MAX + 1;
const BPM_ZONE_HIGH_MIN: u8 = BPM_ZONE_NORMAL_MAX + 1;
const BPM_ZONE_FAST_MIN: u8 = BPM_ZONE_HIGH_MAX + 1;

// These thresholds double as the match arms below and the UI zone labels; change them here to update both.
fn current_zone(bpm: u8) -> ZoneKind {
  match bpm {
    0..=BPM_ZONE_CALM_MAX => ZoneKind::Calm,
    BPM_ZONE_NORMAL_MIN..=BPM_ZONE_NORMAL_MAX => ZoneKind::Normal,
    BPM_ZONE_HIGH_MIN..=BPM_ZONE_HIGH_MAX => ZoneKind::High,
    BPM_ZONE_FAST_MIN..=BPM_ZONE_FAST_MAX => ZoneKind::Fast,
    _ => ZoneKind::Alarm,
  }
}

fn zone_label(zone: ZoneKind, lang: &crate::i18n::Strings) -> &str {
  match zone {
    ZoneKind::Calm => lang.zone_calm,
    ZoneKind::Normal => lang.zone_normal,
    ZoneKind::High => lang.zone_high,
    ZoneKind::Fast => lang.zone_fast,
    ZoneKind::Alarm => lang.zone_alarm,
  }
}

pub fn connected_view(app: &App) -> Element<'_, Message> {
  let lang = app.lang;
  let settings = &app.settings;
  let device_name = app.connected_name.as_deref().unwrap_or(lang.unknown_device);

  let (hr_str, zone_col, badge_label) = if let Some(bpm) = app.heart_rate {
    let zone = current_zone(bpm);
    let color = parse_hex_color(settings.zone_hex(zone));
    (format!("{bpm}"), color, zone_label(zone, lang))
  } else {
    let idle = parse_hex_color(settings.zone_hex(ZoneKind::Calm));
    ("--".to_string(), idle, "---")
  };

  let header = row![
    column![
      text(device_name).size(14.0).color(Color::WHITE),
      text(&app.status).size(11.0).color(Color {
        r: 0.46,
        g: 0.46,
        b: 0.54,
        a: 1.0
      }),
    ]
    .spacing(2)
    .width(Length::Fill),
    button(text("⚙").size(17.0))
      .style(style::btn_icon)
      .on_press(Message::OpenSettings)
      .padding([5, 9]),
  ]
  .align_y(Alignment::Center);

  let hr_card = container(
    column![
      row![
        text("♥").size(44.0).color(zone_col),
        text(hr_str).size(80.0).color(zone_col),
      ]
      .spacing(8)
      .align_y(Alignment::Center),
      text(lang.bpm).size(11.5).color(style::TEXT_FAINT),
      Space::new().height(6.0),
      helpers::zone_badge(badge_label, zone_col),
    ]
    .spacing(4)
    .align_x(Alignment::Center),
  )
  .style(style::card)
  .padding([26, 40])
  .align_x(Alignment::Center)
  .width(Length::Fill);

  let content = column![
    header,
    Space::new().height(14.0),
    hr_card,
    Space::new().height(14.0),
    button(text(format!("⏏  {}", lang.btn_disconnect)).size(13.0))
      .style(style::btn_ghost)
      .on_press(Message::Disconnect)
      .padding([9, 18]),
  ]
  .padding(18)
  .spacing(0);

  container(content)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
