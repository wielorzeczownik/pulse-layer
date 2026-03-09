use axum::{
  Router,
  extract::{
    State, WebSocketUpgrade,
    ws::{Message, WebSocket},
  },
  response::{Html, IntoResponse},
  routing::get,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::watch;

use crate::constants::{OVERLAY_HTML, PORT};
use crate::settings::{AppSettings, OverlayStyle};

#[derive(Clone)]
struct ServerState {
  hr_rx: watch::Receiver<Option<u8>>,
  settings_rx: watch::Receiver<AppSettings>,
}

// Runs in a separate OS thread with its own Tokio runtime.
pub fn spawn(hr_rx: watch::Receiver<Option<u8>>, settings_rx: watch::Receiver<AppSettings>) {
  std::thread::Builder::new()
    .name("overlay-server".into())
    .spawn(move || {
      tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("server runtime")
        .block_on(run(hr_rx, settings_rx));
    })
    .expect("spawn server thread");
}

async fn run(hr_rx: watch::Receiver<Option<u8>>, settings_rx: watch::Receiver<AppSettings>) {
  let state = ServerState { hr_rx, settings_rx };
  let app = Router::new()
    .route("/", get(serve_overlay))
    .route("/ws", get(ws_handler))
    .with_state(state);

  let addr = std::net::SocketAddr::from(([127, 0, 0, 1], PORT));
  let listener = tokio::net::TcpListener::bind(addr)
    .await
    .unwrap_or_else(|_| panic!("bind port {}", PORT));

  axum::serve(listener, app).await.expect("server error");
}

async fn serve_overlay() -> impl IntoResponse {
  Html(OVERLAY_HTML)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<ServerState>) -> impl IntoResponse {
  ws.on_upgrade(|socket| handle_ws(socket, state))
}

async fn handle_ws(socket: WebSocket, mut state: ServerState) {
  let (mut tx, _rx) = socket.split();

  let cfg = state.settings_rx.borrow().clone();
  if tx
    .send(Message::Text(format_config(&cfg).into()))
    .await
    .is_err()
  {
    return;
  }

  let hr = *state.hr_rx.borrow();
  if tx.send(Message::Text(format_hr(hr).into())).await.is_err() {
    return;
  }

  loop {
    tokio::select! {
        result = state.hr_rx.changed() => {
            if result.is_err() { break; }
            let hr = *state.hr_rx.borrow_and_update();
            if tx.send(Message::Text(format_hr(hr).into())).await.is_err() { break; }
        }
        result = state.settings_rx.changed() => {
            if result.is_err() { break; }
            let cfg = state.settings_rx.borrow_and_update().clone();
            if tx.send(Message::Text(format_config(&cfg).into())).await.is_err() { break; }
        }
    }
  }
}

fn format_hr(hr: Option<u8>) -> String {
  match hr {
    Some(bpm) => format!(r#"{{"bpm":{bpm}}}"#),
    None => r#"{"bpm":null}"#.to_string(),
  }
}

fn format_config(s: &AppSettings) -> String {
  let style = match s.overlay_style {
    OverlayStyle::Heart => "heart",
    OverlayStyle::Pulse => "pulse",
  };
  format!(
    r#"{{"config":{{"calm":"{calm}","normal":"{normal}","high":"{high}","fast":"{fast}","alarm":"{alarm}","style":"{style}","panel_bg":"{panel_bg}"}}}}"#,
    calm = s.zone_calm_hex,
    normal = s.zone_normal_hex,
    high = s.zone_high_hex,
    fast = s.zone_fast_hex,
    alarm = s.zone_alarm_hex,
    style = style,
    panel_bg = s.panel_bg_hex,
  )
}
