use tauri::Manager;

use crate::functionality::menu;

pub fn configure(window: &tauri::WebviewWindow) {
  menu::create_menubar(window.app_handle()).unwrap_or_default();
}
