use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::config::get_config;
use crate::log;



// Minimize
#[tauri::command]
pub fn minimize(win: tauri::WebviewWindow) {
  win.minimize().unwrap_or_default();
}

// Toggle maximize
#[tauri::command]
pub fn toggle_maximize(win: tauri::WebviewWindow) {
  if win.is_maximized().unwrap_or_default() {
    win.unmaximize().unwrap_or_default();
  } else {
    win.maximize().unwrap_or_default();
  }
}

#[tauri::command]
#[cfg(not(target_os = "macos"))]
pub fn set_decorations(win: tauri::WebviewWindow, enable: bool) {
  win.set_decorations(enable).unwrap_or_default();
}

// Close
#[tauri::command]
pub fn close(win: tauri::WebviewWindow) {
  // Save window state
  let app = win.app_handle();
  app.save_window_state(StateFlags::all()).unwrap_or_default();

  // Ensure we minimize to tray if the config calls for it
  if get_config().sys_tray.unwrap_or(false) {
    win.hide().unwrap_or_default();
  } else {
    win.close().unwrap_or_default();
  }
}

pub fn setup_autostart(app: &mut tauri::App) {
  let autostart_manager = app.autolaunch();
  let should_enable = get_config().open_on_startup.unwrap_or(false);

  if !should_enable {
    autostart_manager.disable().unwrap_or_default();
  } else {
    autostart_manager.enable().unwrap_or_default();
  }

  log!(
    "Autolaunch enabled: {}",
    autostart_manager.is_enabled().unwrap_or_default()
  );
}

