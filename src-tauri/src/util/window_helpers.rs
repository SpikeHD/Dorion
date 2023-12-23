use crate::config::get_config;
use crate::util::logger::log;

use super::paths::get_webdata_dir;

pub fn clear_cache_check() {
  let appdata = dirs::data_dir().unwrap_or_default().join("dorion");

  if !appdata.exists() {
    std::fs::create_dir_all(&appdata).expect("Failed to create dorion appdata dir!");
  }

  let cache_file = appdata.join("clear_cache");

  if cache_file.exists() {
    // Delete the file
    std::fs::remove_file(&cache_file).expect("Failed to remove clear_cache file!");
    clear_cache();
  }
}

#[tauri::command]
pub fn set_clear_cache(win: tauri::Window) {
  // Create a file called "clear_cache" in the appdata dir
  // This will be read by the window when it closes
  let appdata = dirs::data_dir().unwrap_or_default().join("dorion");

  if !appdata.exists() {
    std::fs::create_dir_all(&appdata).expect("Failed to create dorion appdata dir!");
  }

  let cache_file = appdata.join("clear_cache");

  std::fs::write(cache_file, "").expect("Failed to create clear_cache file!");

  win.close().unwrap_or_default();
}

#[tauri::command]
pub fn clear_cache() {
  // Remove %appdata%/dorion/webdata
  let webdata_dir = get_webdata_dir();

  if webdata_dir.exists() {
    log("Deleting cache...".to_string());
    std::fs::remove_dir_all(webdata_dir).expect("Failed to remove webdata dir!");
  }
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn window_zoom_level(win: &tauri::Window, value: Option<f64>) {
  win
    .with_webview(move |webview| unsafe {
      let zoom = value.unwrap_or(
        get_config()
          .zoom
          .unwrap_or("1.0".to_string())
          .parse::<f64>()
          .unwrap_or(1.0),
      );

      webview.controller().SetZoomFactor(zoom).unwrap_or_default();
    })
    .unwrap_or_default();
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn window_zoom_level(win: &tauri::Window, value: Option<f64>) {
  let zoom = value.unwrap_or(
    get_config()
      .zoom
      .unwrap_or("1.0".to_string())
      .parse::<f64>()
      .unwrap_or(1.0),
  );

  win
    .eval(&format!("document.body.style.zoom = '{}'", zoom))
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
