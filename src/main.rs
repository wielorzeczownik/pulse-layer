#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ble;
mod config;
mod constants;
mod i18n;
mod server;
mod settings;
mod types;
mod ui;

use crate::app::App;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> iced::Result {
  iced::application(App::new, App::update, App::view)
    .title(App::title)
    .theme(App::theme)
    .subscription(App::subscription)
    .font(iced_aw::ICED_AW_FONT_BYTES)
    .window(iced::window::Settings {
      size: iced::Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
      resizable: false,
      ..Default::default()
    })
    .run()
}
