#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use auto_launch::*;
use std::time::Duration;

use config::get_config;
use injection::{injection_runner, local_html, plugin, theme};
use processors::{css_preprocess, js_preprocess};
use profiles::init_profiles_folders;
use tauri::{
  utils::config::AppUrl, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
  Window, WindowBuilder,
};
use util::{
  helpers, notifications,
  paths::get_webdata_dir,
  process,
  window_helpers::{self, clear_cache_check, window_zoom_level},
};

use crate::util::{helpers::move_injection_scripts, paths::injection_is_local};

mod config;
mod deep_link;
mod functionality;
mod hotkeys;
mod init;
mod injection;
mod processors;
mod profiles;
mod release;
mod util;

fn create_systray() -> SystemTray {
  let open_btn = CustomMenuItem::new("open".to_string(), "Open");
  let quit_btn = CustomMenuItem::new("quit".to_string(), "Quit");
  let tray_menu = SystemTrayMenu::new().add_item(open_btn).add_item(quit_btn);

  SystemTray::new().with_menu(tray_menu)
}

#[tauri::command]
fn should_disable_plugins() -> bool {
  std::env::args().any(|arg| arg == "--disable-plugins")
}

fn main() {
  // Ensure config is created
  config::init();
  let config = get_config();

  std::thread::sleep(Duration::from_millis(200));

  if !config.multi_instance.unwrap_or(false) {
    tauri_plugin_deep_link::prepare("com.dorion.dev");
  }

  // before anything else, check if the clear_cache file exists
  clear_cache_check();

  // Run the steps to init profiles
  init_profiles_folders();

  let mut context = tauri::generate_context!("tauri.conf.json");
  let dorion_open = process::process_already_exists();
  let client_type = config.client_type.unwrap_or("default".to_string());
  let mut url = String::new();

  if client_type == "default" {
    url += "https://discord.com/app";
  } else {
    url = format!("https://{}.discord.com/app", client_type);
  }

  let url_ext = tauri::WindowUrl::External(reqwest::Url::parse(&url).unwrap());

  context.config_mut().build.dist_dir = AppUrl::Url(url_ext.clone());
  context.config_mut().build.dev_path = AppUrl::Url(url_ext.clone());

  println!("multi_instance?: {}", config.multi_instance.unwrap_or(false));

  // If another process of Dorion is already open, show a dialog
  // in the future I want to actually *reveal* the other runnning process
  // instead of showing a popup, but this is fine for now
  if dorion_open && !config.multi_instance.unwrap_or(false) {
    // Send the dorion://open deep link request
    helpers::open_scheme("dorion://open".to_string());

    // Exit
    std::process::exit(0);
  }

  // Safemode check
  let safemode = std::env::args().any(|arg| arg == "--safemode");
  println!("Safemode enabled: {}", safemode);

  // Begin the RPC server
  let rpc_thread = std::thread::spawn(|| {
    if !get_config().rpc_server.unwrap_or(false) {
      return;
    }

    println!("Starting RPC server...");
    functionality::rpc::start_rpc_server();
  });

  #[allow(clippy::single_match)]
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .system_tray(create_systray())
    .invoke_handler(tauri::generate_handler![
      minimize,
      maximize,
      close,
      should_disable_plugins,
      css_preprocess::clear_css_cache,
      css_preprocess::localize_imports,
      js_preprocess::localize_all_js,
      local_html::get_index,
      local_html::get_top_bar,
      local_html::get_extra_css,
      notifications::notif_count,
      plugin::load_plugins,
      plugin::get_plugin_list,
      plugin::toggle_plugin,
      plugin::toggle_preload,
      plugin::get_plugin_import_urls,
      profiles::get_profile_list,
      profiles::get_current_profile_folder,
      profiles::create_profile,
      profiles::delete_profile,
      release::do_update,
      release::update_check,
      hotkeys::save_ptt_keys,
      hotkeys::toggle_ptt,
      injection_runner::do_injection,
      injection_runner::get_injection_js,
      injection_runner::is_injected,
      injection_runner::load_injection_js,
      config::read_config_file,
      config::write_config_file,
      config::default_config,
      theme::get_theme,
      theme::get_theme_names,
      helpers::get_platform,
      helpers::open_themes,
      helpers::open_plugins,
      window_helpers::remove_top_bar,
      window_helpers::set_clear_cache,
    ])
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        // Close to tray if the config calls for it
        if get_config().sys_tray.unwrap_or(false) {
          event.window().hide().unwrap();
          api.prevent_close();

          return;
        }

        maybe_clear_cache();
      }
      tauri::WindowEvent::Destroyed { .. } => {
        maybe_clear_cache();
      }
      _ => {}
    })
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        // Reopen the window if the tray menu icon is clicked
        app.get_window("main").unwrap().show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        if id == "quit" {
          // Close the process
          app.get_window("main").unwrap().close().unwrap();
        }

        if id == "open" {
          // Reopen the window
          app.get_window("main").unwrap().show().unwrap();
          app.get_window("main").unwrap().set_focus().unwrap();
          app.get_window("main").unwrap().unminimize().unwrap();
        }
      }
      _ => {}
    })
    .setup(move |app| {
      // First, grab preload plugins
      let title = format!("Dorion - v{}", app.package_info().version);
      let win = WindowBuilder::new(app, "main", url_ext)
        .title(title.as_str())
        .resizable(true)
        .disable_file_drop_handler()
        .data_directory(get_webdata_dir())
        .visible(false)
        .build()?;

      // If safemode is enabled, stop here
      if safemode {
        return Ok(());
      }

      // Init injection scripts
      if !injection_is_local() {
        move_injection_scripts(&win, false);
      }

      modify_window(&win);

      setup_autostart(app);

      #[cfg(not(target_os = "macos"))]
      hotkeys::start_hotkey_watcher(win.clone());

      // Deep link registry
      if !config.multi_instance.unwrap_or(false) {
        deep_link::register_deep_link_handler(win.clone());
      }

      win.show().unwrap();

      init::inject_routine(win);

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");

  println!("This is atest");
  // Join threads
  rpc_thread.join().unwrap();
}

// Minimize
#[tauri::command]
fn minimize(win: Window) {
  win.minimize().unwrap();
}

// Maximize
#[tauri::command]
fn maximize(win: Window) {
  win.maximize().unwrap();
}

// Close
#[tauri::command]
fn close(win: Window) {
  // Ensure we minimize to tray if the config calls for it
  if get_config().sys_tray.unwrap_or(false) {
    win.hide().unwrap();
  } else {
    win.close().unwrap();
  }
}

/**
 * Applies various window modifications, most being platform-dependent
 */
fn modify_window(window: &Window) {
  let startup = std::env::args().any(|arg| arg == "--startup");

  // If we are opening on startup (which we know from the --startup arg), check to minimize the window
  if startup && get_config().startup_minimized.unwrap_or(false) {
    window.hide().unwrap_or_default();
  }

  if get_config().start_maximized.unwrap_or(false) {
    window.maximize().unwrap_or_default();
  }

  window_zoom_level(window, None);
}

fn setup_autostart(app: &mut tauri::App) {
  let app_name = &app.package_info().name;
  let current_exe = std::env::current_exe().unwrap();

  let autolaunch = match AutoLaunchBuilder::new()
    .set_app_name(app_name)
    .set_app_path(current_exe.to_str().unwrap())
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

fn maybe_clear_cache() {
  if get_config().auto_clear_cache.unwrap_or(false) {
    functionality::cache::clear_cache();
  }
}
