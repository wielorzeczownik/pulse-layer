use iced::widget::{container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use super::style;
use crate::types::Message;

pub fn section_label(s: &str) -> Element<'_, Message> {
  text(s.to_string())
    .size(10.5)
    .color(Color {
      r: 0.48,
      g: 0.48,
      b: 0.54,
      a: 1.0,
    })
    .into()
}

pub fn color_hex_input<'a, F>(
  placeholder: &'a str,
  value: &'a str,
  on_change: F,
) -> Element<'a, Message>
where
  F: Fn(String) -> Message + 'a,
{
  let dot_color = crate::settings::parse_hex_color(value);
  let dot = container(text(""))
    .width(16.0)
    .height(16.0)
    .style(move |_: &iced::Theme| iced::widget::container::Style {
      background: Some(Background::Color(dot_color)),
      border: Border {
        radius: 4.0.into(),
        ..Default::default()
      },
      ..Default::default()
    });

  row![
    dot,
    text_input(placeholder, value)
      .on_input(on_change)
      .padding([7, 10])
      .size(13.5)
      .style(style::text_input_field)
      .width(Length::Fill),
  ]
  .spacing(8)
  .align_y(Alignment::Center)
  .into()
}

pub fn zone_badge(label: &str, color: Color) -> Element<'_, Message> {
  container(text(label.to_string()).size(11.0).color(color))
    .style(move |_: &iced::Theme| iced::widget::container::Style {
      background: Some(Background::Color(Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: 0.14,
      })),
      border: Border {
        color: Color {
          r: color.r,
          g: color.g,
          b: color.b,
          a: 0.30,
        },
        width: 1.0,
        radius: 5.0.into(),
      },
      ..Default::default()
    })
    .padding([4, 10])
    .into()
}
