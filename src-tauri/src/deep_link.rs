use tauri::{Listener, Manager};

use crate::log;

pub fn register_deep_link_handler(app: &tauri::AppHandle) {
  let handle = app.clone();

  log!("Registering deep link handler...");

  app.listen("deep-link://open", move |_| {
    let win = handle.get_webview_window("main");

    if win.is_none() {
      return;
    }

    let win = win.unwrap();

    win.show().unwrap_or_default();
    win.set_focus().unwrap_or_default();
    win.unminimize().unwrap_or_default();
  });
}
