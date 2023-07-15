#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn remove_top_bar(win: tauri::Window) {
  win.set_decorations(false).unwrap_or(());
}

// Top bar is broken for MacOS currently
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn remove_top_bar(_win: tauri::Window) {}
