pub fn register_deep_link_handler(win: tauri::Window) {
  tauri_plugin_deep_link::register("dorion", move |_| {
    win.show().unwrap_or_default();
    win.set_focus().unwrap_or_default();
    win.unminimize().unwrap_or_default();
  })
  .unwrap_or_default();
}
