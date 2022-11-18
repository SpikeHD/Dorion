use std::fs;

#[tauri::command]
pub fn get_index() -> String {
  fs::read_to_string("../dist/index.html").unwrap_or(String::new())
}

#[tauri::command]
pub fn get_settings() -> String {
  fs::read_to_string("../dist/settings.html").unwrap_or(String::new())
}