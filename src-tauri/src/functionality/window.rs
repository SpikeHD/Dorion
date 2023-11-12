use auto_launch::AutoLaunchBuilder;
use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use tauri::Window;

use crate::config::get_config;
use crate::deep_link;
use crate::hotkeys;
use crate::init;
use crate::util::window_helpers::window_zoom_level;

// Minimize
#[tauri::command]
pub fn minimize(win: Window) {
  win.minimize().unwrap_or_default();
}

// Maximize
#[tauri::command]
pub fn maximize(win: Window) {
  win.maximize().unwrap_or_default();
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

  #[cfg(not(target_os = "macos"))]
  hotkeys::start_hotkey_watcher(window.clone());

  // Deep link registry
  if !get_config().multi_instance.unwrap_or(false) {
    deep_link::register_deep_link_handler(window.clone());
  }

  // If we are opening on startup (which we know from the --startup arg), check to keep the window minimized
  if !startup && !get_config().startup_minimized.unwrap_or(false) {
    // Now that we are ready, and shouldn't start minimized, show the window
    window.show().unwrap_or_default();
  } else if get_config().startup_minimized.unwrap_or(false) {
    // Depending on whether we have "close to system tray" enabled, we either minimize or hide
    if get_config().sys_tray.unwrap_or(false) {
      window.hide().unwrap_or_default();
    } else {
      window.minimize().unwrap_or_default();
    }
  }

  if get_config().start_maximized.unwrap_or(false) {
    window.maximize().unwrap_or_default();
  }

  // Set user-agent through WebkitGTK config
  #[cfg(target_os = "linux")]
  {
    window.with_webview(|webview| {
      use webkit2gtk::{WebViewExt, SettingsExt, PermissionRequestExt};

      let wv = webview.inner();
      let wv = wv.as_ref();
      let settings = WebViewExt::settings(wv).unwrap_or_default();

      settings.set_user_agent(Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"));

      // We also need to manually ask for permission to use the microphone and camera
      wv.connect_permission_request(|_, req| {
        req.allow();
        true
      });
    }).unwrap_or_else(|_| println!("Failed to set user-agent"));
  }

  window_zoom_level(window, None);

  init::inject_routine(window.to_owned());
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

  println!(
    "Autolaunch enabled: {}",
    autolaunch.is_enabled().unwrap_or_default()
  );
}
