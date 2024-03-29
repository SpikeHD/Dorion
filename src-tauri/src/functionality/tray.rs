use include_flate::flate;
use tauri::{AppHandle, Icon};

use crate::log;

flate!(static DEFAULT: [u8] from "./icons/icon.png");
flate!(static CONNECTED: [u8] from "./icons/tray/connected.png");
flate!(static MUTED: [u8] from "./icons/tray/muted.png");
flate!(static DEAFENED: [u8] from "./icons/tray/deafened.png");
flate!(static SPEAKING: [u8] from "./icons/tray/speaking.png");
flate!(static VIDEO: [u8] from "./icons/tray/video.png");
flate!(static STREAMING: [u8] from "./icons/tray/streaming.png");

#[tauri::command]
pub fn set_tray_icon(app: AppHandle, event: String) {
  log!("Setting tray icon to {}", event.as_str());

  let icon = match event.as_str() {
    "connected" => Icon::Raw(CONNECTED.to_vec()),
    "disconnected" => Icon::Raw(DEFAULT.to_vec()),
    "muted" => Icon::Raw(MUTED.to_vec()),
    "deafened" => Icon::Raw(DEAFENED.to_vec()),
    "speaking" => Icon::Raw(SPEAKING.to_vec()),
    "video" => Icon::Raw(VIDEO.to_vec()),
    "streaming" => Icon::Raw(STREAMING.to_vec()),
    _ => Icon::Raw(DEFAULT.to_vec()),
  };

  app.tray_handle().set_icon(icon).unwrap_or_default();
}
