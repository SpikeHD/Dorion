use include_flate::flate;
use tauri::{
  image::Image,
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  AppHandle, Emitter, Manager,
};

use crate::{log, util::window_helpers::ultrashow};

flate!(static DEFAULT: [u8] from "./icons/32x32.png");
flate!(static UNREAD: [u8] from "./icons/tray/unread.png");
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
    "connected" => Image::from_bytes(&CONNECTED),
    "disconnected" => Image::from_bytes(&DEFAULT),
    "muted" => Image::from_bytes(&MUTED),
    "deafened" => Image::from_bytes(&DEAFENED),
    "speaking" => Image::from_bytes(&SPEAKING),
    "video" => Image::from_bytes(&VIDEO),
    "streaming" => Image::from_bytes(&STREAMING),
    "unread" => Image::from_bytes(&UNREAD),
    _ => Image::from_bytes(&DEFAULT),
  };

  let icon = match icon {
    Ok(icon) => icon,
    Err(e) => {
      log!("Error creating tray icon: {}", e);
      return;
    }
  };

  if let Some(tray) = app.tray_by_id("main") {
    tray.set_icon(Some(icon)).unwrap_or_default();
  }
}

pub fn create_tray(app: &AppHandle) -> Result<(), tauri::Error> {
  let open_item = MenuItemBuilder::with_id("open", "Open").build(app)?;
  let reload_item = MenuItemBuilder::with_id("reload", "Reload").build(app)?;
  let restart_item = MenuItemBuilder::with_id("restart", "Restart").build(app)?;
  let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

  let menu = MenuBuilder::new(app)
    .items(&[&open_item, &reload_item, &restart_item, &quit_item])
    .id("main")
    .build()?;

  TrayIconBuilder::with_id("main")
    .icon(Image::from_bytes(&DEFAULT)?)
    .menu(&menu)
    .on_menu_event(move |app, event| match event.id().as_ref() {
      "quit" => {
        if let Some(win) = app.get_webview_window("main") {
          win.emit("beforeunload", ()).unwrap_or_default();
        }
        app.exit(0);
      }
      "open" => {
        if let Some(win) = app.get_webview_window("main") {
          ultrashow(&win);
        }
      }
      "restart" => {
        if let Some(win) = app.get_webview_window("main") {
          win.emit("beforeunload", ()).unwrap_or_default();
        }
        app.restart();
      }
      "reload" => {
        let window = match app.get_webview_window("main") {
          Some(win) => win,
          None => return,
        };
        window.eval("window.location.reload();").unwrap_or_default();
      }
      _ => {}
    })
    .on_tray_icon_event(|tray, event| {
      if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } = event
      {
        let app = tray.app_handle();
        if let Some(win) = app.get_webview_window("main") {
          ultrashow(&win);
        }
      }
    })
    .build(app)?;

  Ok(())
}

#[cfg(target_os = "macos")]
pub fn get_tray(app: &AppHandle) -> Option<tauri::tray::TrayIcon> {
  app.tray_by_id("main")
}
