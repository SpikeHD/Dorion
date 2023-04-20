#[tauri::command]
pub fn remove_top_bar(win: tauri::Window) {
  win.set_decorations(false).unwrap_or(());
}