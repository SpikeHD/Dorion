pub fn configure(window: &tauri::WebviewWindow) {
  super::menu::create_menubar(window.app_handle()).unwrap_or_default();
}
