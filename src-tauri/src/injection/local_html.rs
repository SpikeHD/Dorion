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

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn get_top_bar(win: tauri::Window) -> String {
  get_html(win.app_handle(), "html/top.html")
}

// Top bar is broken for MacOS currently
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn get_top_bar(_win: tauri::Window) -> String {
  String::new()
}

#[tauri::command]
pub fn get_notif(win: tauri::Window) -> String {
  get_html(win.app_handle(), "html/notification.html")
}

#[tauri::command]
pub fn get_extra_css(win: tauri::Window) -> String {
  get_html(win.app_handle(), "html/extra.css")
}
