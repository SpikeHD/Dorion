use auto_launch::AutoLaunchBuilder;
use tauri::Manager;
use tauri::Window;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::config::get_config;
use crate::deep_link;
use crate::log;
use crate::util::window_helpers::window_zoom_level;
use crate::window::blur::apply_effect;

// Minimize
#[tauri::command]
pub fn minimize(win: Window) {
  win.minimize().unwrap_or_default();
}

// Toggle maximize
#[tauri::command]
pub fn toggle_maximize(win: Window) {
  if win.is_maximized().unwrap_or_default() {
    win.unmaximize().unwrap_or_default();
  } else {
    win.maximize().unwrap_or_default();
  }
}

// Close
#[tauri::command]
pub fn close(win: Window) {
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
pub fn after_build(window: &Window) {
  let startup = std::env::args().any(|arg| arg == "--startup");
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
    deep_link::register_deep_link_handler(window.clone());
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
  }

  window_zoom_level(window.clone(), None);
}

pub fn setup_autostart(app: &mut tauri::App) {
  let app_name = &app.package_info().name;
  let current_exe = std::env::current_exe().unwrap_or_default();
  let exe_str = current_exe.to_str().unwrap_or_default();

  // if the string is empty, just return
  if exe_str.is_empty() {
    return;
  }

  let autolaunch = match AutoLaunchBuilder::new()
    .set_app_name(app_name)
    .set_app_path(exe_str)
    .set_use_launch_agent(true)
    .set_args(&["--startup"])
    .build()
  {
    Ok(autolaunch) => autolaunch,
    Err(_) => return,
  };

  let should_enable = get_config().open_on_startup.unwrap_or(false);

  autolaunch.enable().unwrap_or_default();

  if !should_enable {
    autolaunch.disable().unwrap_or_default();
  }

  log!(
    "Autolaunch enabled: {}",
    autolaunch.is_enabled().unwrap_or_default()
  );
}
