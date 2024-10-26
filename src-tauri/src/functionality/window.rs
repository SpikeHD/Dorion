use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::config::get_config;
use crate::log;
use crate::util::color::start_os_accent_subscriber;
use crate::util::window_helpers::window_zoom_level;

#[cfg(feature = "blur")]
use crate::window::blur::apply_effect;

// Minimize
#[tauri::command]
pub fn minimize(win: tauri::WebviewWindow) {
  win.minimize().unwrap_or_default();
}

// Toggle maximize
#[tauri::command]
pub fn toggle_maximize(win: tauri::WebviewWindow) {
  if win.is_maximized().unwrap_or_default() {
    win.unmaximize().unwrap_or_default();
  } else {
    win.maximize().unwrap_or_default();
  }
}

#[tauri::command]
#[cfg(not(target_os = "macos"))]
pub fn set_decorations(win: tauri::WebviewWindow, enable: bool) {
  win.set_decorations(enable).unwrap_or_default();
}

// Close
#[tauri::command]
pub fn close(win: tauri::WebviewWindow) {
  // Save window state
  let app = win.app_handle();
  app.save_window_state(StateFlags::all()).unwrap_or_default();

  // Ensure we minimize to tray if the config calls for it
  if get_config().sys_tray.unwrap_or(false) {
    win.hide().unwrap_or_default();
  } else {
    win.close().unwrap_or_default();
  }
}

/**
 * Applies various window modifications, most being platform-dependent
 */
pub fn after_build(window: &tauri::WebviewWindow) {
  let startup = std::env::args().any(|arg| arg == "--startup");
  let app = window.app_handle();
  let config = get_config();

  // If the subscription is dropped, Mundy's internal thread will exit and no events will ever be recieved
  Box::leak(Box::new(start_os_accent_subscriber(window)));

  if config.streamer_mode_detection.unwrap_or(false) {
    log!("Starting streamer mode watcher...");
    super::streamer_mode::start_streamer_mode_watcher(window.clone());
  }

  #[cfg(feature = "hotkeys")]
  #[cfg(not(target_os = "macos"))]
  if config.keybinds_enabled.unwrap_or(false) {
    log!("Starting global keybind watcher...");
    super::hotkeys::start_keybind_watcher(window);
  }

  // If we are opening on startup (which we know from the --startup arg), check to keep the window minimized
  if !startup || !config.startup_minimized.unwrap_or(false) {
    // Now that we are ready, and shouldn't start minimized, show the window
    window.show().unwrap_or_default();
  } else {
    window.hide().unwrap_or_default();
  }

  if config.start_maximized.unwrap_or(false) {
    window.maximize().unwrap_or_default();
  }

  #[cfg(feature = "blur")]
  apply_effect(
    window.clone(),
    config.blur.unwrap_or("none".to_string()).as_str(),
  );

  // Set WebkitGTK config
  #[cfg(target_os = "linux")]
  {
    use std::path::PathBuf;

    use tauri::path::BaseDirectory;
    use webkit2gtk::WebContextExt;
    use webkit2gtk::WebViewExt;

    use crate::gpu::disable_hardware_accel_linux;

    disable_hardware_accel_linux(window);
    enable_webrtc(window);

    let handle = app.clone();

    // Extension patch
    window
      .with_webview(move |webview| {
        let webview = webview.inner();
        let context = webview.context();
        let path = handle
          .path()
          .resolve(PathBuf::from("extension_webkit"), BaseDirectory::Resource)
          .unwrap_or_default();

        if let Some(context) = context {
          let path_str = path.as_os_str().to_str();

          if let Some(path_str) = path_str {
            context.set_web_extensions_directory(path_str);
          }
        }
      })
      .unwrap_or_else(|e| log!("Failed to set web extensions directory: {:?}", e));
  }

  match super::tray::create_tray(app) {
    Ok(_) => {}
    Err(e) => {
      log!("Error creating tray icon: {:?}", e);
    }
  }

  #[cfg(target_os = "windows")]
  {
    use super::extension::add_extension;
    use crate::util::paths::get_main_extension_path;
    use std::fs;
    use std::path::PathBuf;
    use tauri::path::BaseDirectory;

    // This should be the last extension loaded, the others are loaded early on
    let main_ext_res_path = window
      .app_handle()
      .path()
      .resolve(PathBuf::from("extension"), BaseDirectory::Resource)
      .unwrap_or_default();

    let main_ext_path = get_main_extension_path();

    // TODO on Windows, this needs to be copied somewhere more accessible to the user for some reason
    // Copy the files in the resource dir to the main extension dir if the files don't already exist
    if let Ok(read_dir) = fs::read_dir(main_ext_res_path) {
      for file in read_dir.flatten() {
        let file_path = file.path();
        let file_name = file_path.clone();
        let file_name = file_name.file_name().unwrap_or_default();

        fs::copy(file_path, main_ext_path.join(file_name)).unwrap_or_default();
      }
    }

    add_extension(window, main_ext_path.clone());

    // Refresh the page to ensure extensions are loaded
    window.eval("window.location.reload();").unwrap_or_default();
  }

  #[cfg(target_os = "macos")]
  super::menu::create_menubar(app).unwrap_or_default();

  window_zoom_level(window.clone(), None);
}

pub fn setup_autostart(app: &mut tauri::App) {
  let autostart_manager = app.autolaunch();
  let should_enable = get_config().open_on_startup.unwrap_or(false);

  if !should_enable {
    autostart_manager.disable().unwrap_or_default();
  } else {
    autostart_manager.enable().unwrap_or_default();
  }

  log!(
    "Autolaunch enabled: {}",
    autostart_manager.is_enabled().unwrap_or_default()
  );
}

#[cfg(target_os = "linux")]
pub fn enable_webrtc(window: &tauri::WebviewWindow) {
  use crate::log;
  use webkit2gtk::{PermissionRequestExt, SettingsExt, WebViewExt};

  window
    .with_webview(move |webview| {
      let wv = webview.inner();
      let settings = WebViewExt::settings(&wv).unwrap_or_default();

      settings.set_enable_webrtc(true);
      settings.set_enable_media_stream(true);

      // We also need to handle permission requests
      wv.connect_permission_request(|_, req| {
        req.allow();
        true
      });
    })
    .unwrap_or_else(|_| log!("Failed to enable WebRTC"));
}
