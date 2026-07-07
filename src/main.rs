#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ble;
mod config;
mod i18n;
mod server;
mod settings;
mod types;
mod ui;

use crate::app::App;

const WINDOW_WIDTH: f32 = 440.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> iced::Result {
  iced::application(App::new, App::update, App::view)
    .title(App::title)
    .theme(|_state: &App| iced::Theme::Dark)
    .subscription(App::subscription)
    .font(iced_aw::ICED_AW_FONT_BYTES)
    .window(iced::window::Settings {
      size: iced::Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
      resizable: false,
      ..Default::default()
    })
    .run()
}
