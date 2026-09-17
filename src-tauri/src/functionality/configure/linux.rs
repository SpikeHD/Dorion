use crate::log;

pub fn configure(_window: &tauri::WebviewWindow<crate::Runtime>) {
  log!("Skipping WebKitGTK window configuration on the CEF runtime");
}
