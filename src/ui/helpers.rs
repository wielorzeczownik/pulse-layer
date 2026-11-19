use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use super::style;
use crate::App;
use crate::app::UpdateState;
use crate::types::Message;

pub fn section_label(label: &str) -> Element<'_, Message> {
  text(label.to_string())
    .size(10.5)
    .color(Color {
      r: 0.48,
      g: 0.48,
      b: 0.54,
      a: 1.0,
    })
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

/// Thin bar above every screen while a newer signed release is waiting.
pub fn update_bar(app: &App) -> Option<Element<'_, Message>> {
  let (found, installing) = match &app.update {
    UpdateState::None => return None,
    UpdateState::Available(found) => (found, false),
    UpdateState::Installing(found) => (found, true),
  };
  let btn = button(text(app.lang.btn_update).size(12.0))
    .style(style::btn_ghost)
    .padding([4, 12])
    .on_press_maybe((!installing).then_some(Message::InstallUpdate));
  Some(
    container(
      row![
        text(format!("{} v{}", app.lang.update_available, found.version))
          .size(12.5)
          .color(style::TEXT_MUTED),
        Space::new().width(Length::Fill),
        btn,
      ]
      .align_y(Alignment::Center),
    )
    .style(style::bar)
    .padding([6, 18])
    .width(Length::Fill)
    .into(),
  )
}
