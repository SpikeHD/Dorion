use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::config::get_config;
use crate::deep_link;
use crate::log;
use crate::util::window_helpers::window_zoom_level;
use crate::window::blur::apply_effect;

use super::extension::add_extension;
use super::tray;

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

  if config.streamer_mode_detection.unwrap_or(false) {
    log!("Starting streamer mode watcher...");
    super::streamer_mode::start_streamer_mode_watcher(window.clone());
  }

  if config.keybinds_enabled.unwrap_or(false) {
    log!("Starting global keybind watcher...");
    super::hotkeys::start_keybind_watcher(window);
  }

  // Deep link registry
  if !config.multi_instance.unwrap_or(false) {
    deep_link::register_deep_link_handler(app);
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

  apply_effect(
    window.clone(),
    config.blur.unwrap_or("none".to_string()).as_str(),
  );

  // Set user-agent through WebkitGTK config
  #[cfg(target_os = "linux")]
  {
    use crate::gpu::disable_hardware_accel_linux;
    disable_hardware_accel_linux(window);
    enable_webrtc(window);
  }

  match tray::create_tray(app) {
    Ok(_) => {}
    Err(e) => {
      log!("Error creating tray icon: {:?}", e);
    }
  }

  add_extension(&window);

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
