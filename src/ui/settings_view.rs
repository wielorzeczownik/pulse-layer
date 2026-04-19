use iced::widget::{Space, button, column, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};
use iced_aw::ColorPicker;

use super::{helpers, style};
use crate::App;
use crate::settings::{OverlayStyle, ZoneKind, color_to_hex, parse_hex_color};
use crate::types::Message;

type ZoneDef = (
  ZoneKind,
  fn(&crate::i18n::Strings) -> &'static str,
  &'static str,
);

const ZONES: &[ZoneDef] = &[
  (ZoneKind::Calm, |l| l.zone_calm, "0-64"),
  (ZoneKind::Normal, |l| l.zone_normal, "65-80"),
  (ZoneKind::High, |l| l.zone_high, "81-100"),
  (ZoneKind::Fast, |l| l.zone_fast, "101-130"),
  (ZoneKind::Alarm, |l| l.zone_alarm, "131+"),
];

pub fn settings_view(app: &App) -> Element<'_, Message> {
  let lang = app.lang;
  let s = &app.settings;

  let header = row![
    text(lang.settings_title).size(18.0).color(Color::WHITE),
    Space::new().width(Length::Fill),
    button(text(format!("✕  {}", lang.btn_close)).size(12.5))
      .style(style::btn_ghost)
      .on_press(Message::CloseSettings)
      .padding([7, 14]),
  ]
  .align_y(Alignment::Center);

  let mut zone_inputs = column![].spacing(10);
  for &(zone, get_label, bpm_range) in ZONES {
    let label_str = format!("{} ({})", get_label(lang), bpm_range);
    let hex = s.zone_hex(zone);
    let color = parse_hex_color(hex);
    let show = app.color_picker_zone == Some(zone);

    let dot_btn = button(text(""))
      .width(28.0)
      .height(28.0)
      .style(
        move |_: &iced::Theme, status: iced::widget::button::Status| {
          let border_alpha = if matches!(status, iced::widget::button::Status::Hovered) {
            0.80
          } else {
            0.45
          };
          iced::widget::button::Style {
            background: Some(Background::Color(color)),
            border: Border {
              color: Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: border_alpha,
              },
              width: 2.0,
              radius: 6.0.into(),
            },
            ..Default::default()
          }
        },
      )
      .on_press(Message::OpenColorPicker(zone));

    let picker = ColorPicker::new(show, color, dot_btn, Message::CloseColorPicker, move |c| {
      Message::SetZoneHex(zone, color_to_hex(c))
    })
    .style(style::color_picker);

    let hex_input = text_input(lang.hex_placeholder, hex)
      .on_input(move |v| Message::SetZoneHex(zone, v))
      .padding([6, 10])
      .size(13.5)
      .style(style::text_input_field)
      .width(Length::Fill);

    let zone_row = row![
      text(label_str)
        .size(12.0)
        .color(Color {
          r: 0.70,
          g: 0.70,
          b: 0.76,
          a: 1.0
        })
        .width(Length::Fixed(110.0)),
      picker,
      hex_input,
    ]
    .spacing(8)
    .align_y(Alignment::Center);
    zone_inputs = zone_inputs.push(zone_row);
  }

  let mut preview_zones = column![].spacing(3).align_x(Alignment::Center);
  for &(zone, get_label, _) in ZONES {
    let col = parse_hex_color(s.zone_hex(zone));
    let row = row![
      text("♥").size(13.0).color(col),
      text(get_label(lang)).size(8.0).color(col),
    ]
    .spacing(5)
    .align_y(Alignment::Center);
    preview_zones = preview_zones.push(row);
  }

  let preview_big_col = parse_hex_color(s.zone_hex(ZoneKind::Normal));
  let preview = container(
    column![
      row![
        text("♥").size(44.0).color(preview_big_col),
        text("72").size(50.0).color(preview_big_col),
      ]
      .spacing(8)
      .align_y(Alignment::Center),
      text("BPM").size(10.5).color(Color {
        r: 0.40,
        g: 0.40,
        b: 0.48,
        a: 1.0
      }),
      Space::new().height(8.0),
      preview_zones,
    ]
    .spacing(3)
    .align_x(Alignment::Center),
  )
  .style(style::card)
  .padding([16, 22])
  .align_x(Alignment::Center);

  let is_heart = s.overlay_style == OverlayStyle::Heart;
  let style_toggle = row![
    button(text(format!("♥  {}", lang.style_heart)).size(12.5))
      .style(if is_heart {
        style::btn_primary
      } else {
        style::btn_ghost
      })
      .on_press(Message::SetOverlayStyle(OverlayStyle::Heart))
      .padding([7, 14]),
    button(text(format!("〰  {}", lang.style_pulse)).size(12.5))
      .style(if is_heart {
        style::btn_ghost
      } else {
        style::btn_primary
      })
      .on_press(Message::SetOverlayStyle(OverlayStyle::Pulse))
      .padding([7, 14]),
  ]
  .spacing(8);

  let bg_color = parse_hex_color(&s.panel_bg_hex);
  let bg_dot = button(text(""))
    .width(28.0)
    .height(28.0)
    .style(
      move |_: &iced::Theme, status: iced::widget::button::Status| {
        let border_alpha = if matches!(status, iced::widget::button::Status::Hovered) {
          0.80
        } else {
          0.45
        };
        iced::widget::button::Style {
          background: Some(Background::Color(bg_color)),
          border: Border {
            color: Color {
              r: bg_color.r,
              g: bg_color.g,
              b: bg_color.b,
              a: border_alpha,
            },
            width: 2.0,
            radius: 6.0.into(),
          },
          ..Default::default()
        }
      },
    )
    .on_press(Message::OpenBgPicker);

  let bg_picker = ColorPicker::new(
    app.bg_picker_open,
    bg_color,
    bg_dot,
    Message::CloseColorPicker,
    |c| Message::SetPanelBg(color_to_hex(c)),
  )
  .style(style::color_picker);

  let panel_bg_row = row![
    text(lang.label_bg)
      .size(12.0)
      .color(Color {
        r: 0.70,
        g: 0.70,
        b: 0.76,
        a: 1.0
      })
      .width(Length::Fixed(110.0)),
    bg_picker,
    text_input(lang.hex_placeholder, &s.panel_bg_hex)
      .on_input(Message::SetPanelBg)
      .padding([6, 10])
      .size(13.5)
      .style(style::text_input_field)
      .width(Length::Fill),
  ]
  .spacing(8)
  .align_y(Alignment::Center);

  let controls = column![
    helpers::section_label(lang.label_style),
    Space::new().height(8.0),
    style_toggle,
    Space::new().height(16.0),
    helpers::section_label(lang.label_bg),
    Space::new().height(8.0),
    panel_bg_row,
    Space::new().height(16.0),
    helpers::section_label(lang.label_indicator_color),
    Space::new().height(8.0),
    zone_inputs,
  ]
  .spacing(0)
  .width(Length::Fill);

  let preview_col = column![
    helpers::section_label(lang.label_preview),
    Space::new().height(6.0),
    preview,
  ]
  .spacing(0);

  let content = column![
    header,
    Space::new().height(16.0),
    row![controls, Space::new().width(16.0), preview_col],
  ]
  .padding(18)
  .spacing(0);

  container(content)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
