use crate::config::get_config;

use super::paths::get_webdata_dir;

pub fn clear_cache_check() {
  let appdata = tauri::api::path::data_dir().unwrap().join("dorion");

  if !appdata.exists() {
    std::fs::create_dir_all(&appdata).unwrap();
  }

  let cache_file = appdata.join("clear_cache");

  if cache_file.exists() {
    // Delete the file
    std::fs::remove_file(&cache_file).unwrap();
    clear_cache();
  }
}

#[tauri::command]
pub fn set_clear_cache(win: tauri::Window) {
  // Create a file called "clear_cache" in the appdata dir
  // This will be read by the window when it closes
  let appdata = tauri::api::path::data_dir().unwrap().join("dorion");

  if !appdata.exists() {
    std::fs::create_dir_all(&appdata).unwrap();
  }

  let cache_file = appdata.join("clear_cache");

  std::fs::write(cache_file, "").unwrap();

  win.close().unwrap_or_default();
}

#[tauri::command]
pub fn clear_cache() {
  // Remove %appdata%/dorion/webdata
  let webdata_dir = get_webdata_dir();

  if webdata_dir.exists() {
    println!("Deleting cache...");
    std::fs::remove_dir_all(webdata_dir).unwrap();
  }
}

#[cfg(target_os = "windows")]
pub fn window_zoom_level(win: &tauri::Window) {
  win
    .with_webview(|webview| unsafe {
      let zoom = config::get_zoom();

      webview.controller().SetZoomFactor(zoom).unwrap_or_default();
    })
    .unwrap_or_default();
}

#[cfg(not(target_os = "windows"))]
pub fn window_zoom_level(win: &tauri::Window) {
  let zoom = get_config().zoom.unwrap_or("1.0".to_string());

  win
    .eval(&format!(
      "
    document.body.style.zoom = '{}';
  ",
      zoom
    ))
    .expect("Failed to set zoom level!");
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn remove_top_bar(win: tauri::Window) {
  win.set_decorations(false).unwrap_or(());
}

// Top bar is broken for MacOS currently
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn remove_top_bar(_win: tauri::Window) {}
