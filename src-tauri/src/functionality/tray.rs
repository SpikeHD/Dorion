use include_flate::flate;
use tauri::{SystemTray, Window, Icon, Manager};

use crate::util::logger::log;

flate!(static DEFAULT: [u8] from "./icons/icon.png");
flate!(static CONNECTED: [u8] from "./icons/tray/connected.png");
flate!(static MUTED: [u8] from "./icons/tray/muted.png");
flate!(static DEAFENED: [u8] from "./icons/tray/deafened.png");
flate!(static SPEAKING: [u8] from "./icons/tray/speaking.png");
flate!(static VIDEO: [u8] from "./icons/tray/video.png");

#[tauri::command]
pub fn set_default_tray_icon(app: tauri::AppHandle, event: String) {
	println!("{}", event);

  log(format!("Setting tray icon"));

  if event == "connected" {
		app.tray_handle().set_icon(Icon::Raw(CONNECTED.to_vec())).unwrap_or_default();
	} else if event == "disconnected" {
		app.tray_handle().set_icon(Icon::Raw(DEFAULT.to_vec())).unwrap_or_default();
	} else if event == "muted" {
		app.tray_handle().set_icon(Icon::Raw(MUTED.to_vec())).unwrap_or_default();
	} else if event == "deafened" {
		app.tray_handle().set_icon(Icon::Raw(DEAFENED.to_vec())).unwrap_or_default();
	} else if event == "speaking" {
		app.tray_handle().set_icon(Icon::Raw(SPEAKING.to_vec())).unwrap_or_default();
	} else if event == "video" {
		app.tray_handle().set_icon(Icon::Raw(VIDEO.to_vec())).unwrap_or_default();
	}
}
