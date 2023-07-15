use std::{fs, path::PathBuf};

use tauri::Manager;

pub fn get_html(app: tauri::AppHandle, dir: &str) -> String {
  let dir = app
    .path_resolver()
    .resolve_resource(PathBuf::from(dir))
    .unwrap();
  match fs::read_to_string(&dir) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read {:?} in local dir: {}", dir, e);

      String::new()
    }
  }
}

#[tauri::command]
pub fn get_index(win: tauri::Window) -> String {
  get_html(win.app_handle(), "html/index.html")
}

#[tauri::command]
pub fn get_top_bar(win: tauri::Window) -> String {
  get_html(win.app_handle(), "html/top.html")
}

#[tauri::command]
pub fn get_notif(win: tauri::Window) -> String {
  get_html(win.app_handle(), "html/notification.html")
}
