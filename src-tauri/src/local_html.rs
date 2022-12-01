use std::fs;

use crate::helpers::resource_folder;

#[tauri::command]
pub fn get_index() -> String {
  match fs::read_to_string(resource_folder().join("html/index.html")) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read index.html in local dir: {}", e);

      String::new()
    }
  }
}

#[tauri::command]
pub fn get_settings() -> String {
  match fs::read_to_string(resource_folder().join("html/settings.html")) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read settings.html in local dir: {}", e);

      String::new()
    }
  }
}
