pub fn register_deep_link_handler(win: tauri::Window) {
  tauri_plugin_deep_link::register("dorion", move |_| {
    win.set_focus().unwrap();
    win.unminimize().unwrap();
  }).unwrap();
}