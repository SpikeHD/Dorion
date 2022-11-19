use std::{fs, path::PathBuf};

#[tauri::command]
pub fn get_index() -> String {
  match fs::read_to_string(PathBuf::from("html/index.html")) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read index.html in local dir: {}", e);
      println!("Checking usr/lib");

      // This is where the .deb installer throws it.
      match fs::read_to_string(PathBuf::from("/usr/lib/dorion/html/index.html")) {
        Ok(f) => f,
        Err(e) => {
          println!("Failed to read index.html: {}", e);

          String::new()
        }
      }
    }
  }
}

#[tauri::command]
pub fn get_settings() -> String {
  match fs::read_to_string(PathBuf::from("html/settings.html")) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read settings.html in local dir: {}", e);
      println!("Checking usr/lib");

      // This is where the .deb installer throws it.
      match fs::read_to_string(PathBuf::from("/usr/lib/dorion/html/settings.html")) {
        Ok(f) => f,
        Err(e) => {
          println!("Failed to read settings.html: {}", e);

          String::new()
        }
      }
    }
  }
}
