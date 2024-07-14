use tauri::{Manager, Listener};

use crate::log;

pub fn register_deep_link_handler(app: &tauri::AppHandle) {
  let handle = app.clone();
  app.listen("dorion://open", move |url| {
    log!("Received deep link message: {:?}", url);

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
