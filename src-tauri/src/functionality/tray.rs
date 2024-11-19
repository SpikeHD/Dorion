use std::{
  str::FromStr,
  sync::atomic::{AtomicUsize, Ordering},
};

use include_flate::flate;
use tauri::{
  image::Image,
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  AppHandle, Emitter, Manager,
};

use crate::{log, util::window_helpers::ultrashow};

#[cfg(target_os = "macos")]
flate!(static DEFAULT: [u8] from "./icons/tray/macos.png");
#[cfg(not(target_os = "macos"))]
flate!(static DEFAULT: [u8] from "./icons/32x32.png");
flate!(static UNREAD: [u8] from "./icons/tray/unread.png");
flate!(static CONNECTED: [u8] from "./icons/tray/connected.png");
flate!(static MUTED: [u8] from "./icons/tray/muted.png");
flate!(static DEAFENED: [u8] from "./icons/tray/deafened.png");
flate!(static SPEAKING: [u8] from "./icons/tray/speaking.png");
flate!(static VIDEO: [u8] from "./icons/tray/video.png");
flate!(static STREAMING: [u8] from "./icons/tray/streaming.png");

pub enum TrayIcon {
  Default,
  Unread,
  Connected,
  Muted,
  Deafened,
  Speaking,
  Video,
  Streaming,
}

impl TrayIcon {
  pub fn image(&self) -> Result<Image<'_>, tauri::Error> {
    match self {
      TrayIcon::Default => Image::from_bytes(&DEFAULT),
      TrayIcon::Unread => Image::from_bytes(&UNREAD),
      TrayIcon::Connected => Image::from_bytes(&CONNECTED),
      TrayIcon::Muted => Image::from_bytes(&MUTED),
      TrayIcon::Deafened => Image::from_bytes(&DEAFENED),
      TrayIcon::Speaking => Image::from_bytes(&SPEAKING),
      TrayIcon::Video => Image::from_bytes(&VIDEO),
      TrayIcon::Streaming => Image::from_bytes(&STREAMING),
    }
  }

  pub fn get_usize(&self) -> usize {
    match self {
      TrayIcon::Default => 0,
      TrayIcon::Unread => 1,
      TrayIcon::Connected => 2,
      TrayIcon::Muted => 3,
      TrayIcon::Deafened => 4,
      TrayIcon::Speaking => 5,
      TrayIcon::Video => 6,
      TrayIcon::Streaming => 7,
    }
  }

  pub fn from_usize(value: usize) -> Self {
    match value {
      0 => TrayIcon::Default,
      1 => TrayIcon::Unread,
      2 => TrayIcon::Connected,
      3 => TrayIcon::Muted,
      4 => TrayIcon::Deafened,
      5 => TrayIcon::Speaking,
      6 => TrayIcon::Video,
      7 => TrayIcon::Streaming,
      _ => TrayIcon::Default,
    }
  }

  // Check if it makes sense for us to overwrite the tray icon (basically, "is the tray anything other than default/unread?")
  pub fn is_overwrite(&self) -> bool {
    matches!(self, TrayIcon::Default | TrayIcon::Unread)
  }
}

impl FromStr for TrayIcon {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      #[cfg(not(target_os = "macos"))]
      "unread" => Ok(TrayIcon::Unread),
      "connected" => Ok(TrayIcon::Connected),
      "muted" => Ok(TrayIcon::Muted),
      "deafened" => Ok(TrayIcon::Deafened),
      "speaking" => Ok(TrayIcon::Speaking),
      "video" => Ok(TrayIcon::Video),
      "streaming" => Ok(TrayIcon::Streaming),
      _ => Ok(TrayIcon::Default),
    }
  }
}

pub static TRAY_STATE: AtomicUsize = AtomicUsize::new(0);

#[tauri::command]
pub fn set_tray_icon(app: AppHandle, event: String) {
  log!("Setting tray icon to {}", event.as_str());

  let tray_icon = match event.as_str().parse::<TrayIcon>() {
    Ok(i) => i,
    Err(_) => TrayIcon::Default,
  };

  let icon = match tray_icon.image() {
    Ok(i) => i,
    Err(e) => {
      log!("Error creating tray icon: {:?}", e);
      return;
    }
  };

  if let Some(tray) = app.tray_by_id("main") {
    tray.set_icon(Some(icon)).unwrap_or_default();
    TRAY_STATE.store(tray_icon.get_usize(), Ordering::Relaxed);
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
          ultrashow(win);
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
          ultrashow(win);
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
