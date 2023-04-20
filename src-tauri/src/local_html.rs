use std::fs;

use crate::helpers::resource_folder;

pub fn get_html(dir: &str) -> String {
  match fs::read_to_string(resource_folder().join(dir)) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read {} in local dir: {}", dir, e);

      String::new()
    }
  }
}

#[tauri::command]
pub fn get_index() -> String {
  get_html("html/index.html")
}

#[tauri::command]
pub fn get_settings() -> String {
  get_html("html/settings.html")
}

#[tauri::command]
pub fn get_top_bar() -> String {
  get_html("html/top.html")
}
