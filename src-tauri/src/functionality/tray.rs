use include_flate::flate;
use tauri::{
  image::Image, menu::{
    MenuBuilder,
    MenuItemBuilder,
  }, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, AppHandle, Manager
};

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
    "connected" => Image::new(&CONNECTED, 48, 48),
    "disconnected" =>Image::new(&DEFAULT, 48, 48),
    "muted" => Image::new(&MUTED, 48, 48),
    "deafened" => Image::new(&DEAFENED, 48, 48),
    "speaking" => Image::new(&SPEAKING, 48, 48),
    "video" => Image::new(&VIDEO, 48, 48),
    "streaming" => Image::new(&STREAMING, 48, 48),
    _ => Image::new(&DEFAULT, 48, 48),
  };

  if let Some(tray) = app.tray_by_id("main") {
    tray.set_icon(Some(icon)).unwrap_or_default();
  }
}

pub fn create_tray(app: &tauri::App) -> Result<(), tauri::Error> {
  let open_item = MenuItemBuilder::with_id("open", "Open").build(app)?;
  let reload_item = MenuItemBuilder::with_id("reload", "Reload").build(app)?;
  let restart_item = MenuItemBuilder::with_id("restart", "Restart").build(app)?;
  let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

  let menu = MenuBuilder::new(app).items(&[&open_item, &reload_item, &restart_item, &quit_item]).build()?;
  
  TrayIconBuilder::with_id("main")
    .menu(&menu)
    .on_menu_event(move |app, event| match event.id().as_ref() {
      "quit" => {
        app.exit(0);
      }
      "open" => {
        match app.get_webview_window("main") {
          Some(win) => {
            win.show().unwrap_or_default();
            win.set_focus().unwrap_or_default();
            win.unminimize().unwrap_or_default();
          }
          None => {}
        }
      }
      "restart" => {
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
          if let Some(webview_window) = app.get_webview_window("main") {
          let _ = webview_window.show();
          let _ = webview_window.set_focus();
          }
      }
    })
    .build(app)?;

  Ok(())
}