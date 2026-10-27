use crate::log;

pub fn configure(_window: &tauri::WebviewWindow<crate::Runtime>) {
  log!("Skipping Linux window configuration");
}
