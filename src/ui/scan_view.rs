use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};

use super::style;
use crate::App;
use crate::settings::{ZoneKind, parse_hex_color};
use crate::types::Message;

pub fn scan_view(app: &App) -> Element<'_, Message> {
  let lang = app.lang;
  // idle heart color matches the calm zone
  let heart_col = parse_hex_color(app.settings.zone_hex(ZoneKind::Calm));

  let header = row![
    row![
      text(lang.app_title).size(22.0).color(heart_col),
      Space::new().width(5),
      text(format!("v{}", env!("CARGO_PKG_VERSION"))).size(11)
    ],
    Space::new().width(Length::Fill),
    button(text("⚙").size(17.0))
      .style(style::btn_icon)
      .on_press(Message::OpenSettings)
      .padding([5, 9]),
  ]
  .align_y(Alignment::Center);

  let status_str = if app.scanning && !app.devices.is_empty() {
    format!("{} — {} devices", app.status, app.devices.len())
  } else {
    app.status.clone()
  };

  let status_text = text(status_str).size(12.0).color(Color {
    r: 0.50,
    g: 0.50,
    b: 0.58,
    a: 1.0,
  });

  let scan_btn = if app.scanning {
    button(text(format!("⏹  {}", lang.scan_stop)).size(13.5))
      .style(style::btn_ghost)
      .on_press(Message::StopScan)
      .padding([10, 22])
  } else {
    button(text(format!("⟳  {}", lang.scan_start)).size(13.5))
      .style(style::btn_primary)
      .on_press(Message::StartScan)
      .padding([10, 22])
  };

  let mut device_list = column![].spacing(7);

  if app.devices.is_empty() && app.scanning {
    device_list = device_list.push(text(lang.scan_searching).size(12.5).color(Color {
      r: 0.42,
      g: 0.42,
      b: 0.48,
      a: 1.0,
    }));
  }

  for device in &app.devices {
    let rssi_str = device.rssi.map(|r| format!("{r} dBm")).unwrap_or_default();

    let entry = container(
      row![
        text(device.name.clone())
          .size(13.5)
          .color(Color {
            r: 0.88,
            g: 0.88,
            b: 0.92,
            a: 1.0
          })
          .width(Length::Fill),
        text(rssi_str).size(11.5).color(Color {
          r: 0.40,
          g: 0.40,
          b: 0.47,
          a: 1.0
        }),
        button(text(lang.btn_connect).size(12.5))
          .style(style::btn_primary)
          .on_press(Message::ConnectSelected(device.id.clone()))
          .padding([5, 13]),
      ]
      .spacing(8)
      .align_y(Alignment::Center),
    )
    .style(style::card)
    .padding([9, 14]);

    device_list = device_list.push(entry);
  }

  let content = column![
    header,
    Space::new().height(3.0),
    status_text,
    Space::new().height(14.0),
    scan_btn,
    Space::new().height(14.0),
    scrollable(device_list).height(Length::Fill),
  ]
  .padding(18)
  .spacing(0);

  container(content)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
