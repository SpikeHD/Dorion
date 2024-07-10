use tauri::Manager;
use tauri_plugin_deep_link::DeepLinkExt;

pub fn register_deep_link_handler(app: &tauri::AppHandle) {
  app.listen("dorion://open", |url| {
    // TODO do this
    // let win = app.get_webview_window("main");

    // if win.is_none() {
    //   return;
    // }

    // let win = win.unwrap();

    // win.show().unwrap_or_default();
    // win.set_focus().unwrap_or_default();
    // win.unminimize().unwrap_or_default();
  });
}
