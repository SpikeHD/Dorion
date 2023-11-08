#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::time::Duration;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder};

use config::get_config;
use injection::{injection_runner, local_html, plugin, theme};
use processors::{css_preprocess, js_preprocess};
use profiles::init_profiles_folders;
use util::{
  helpers, notifications,
  paths::get_webdata_dir,
  process,
  window_helpers::{self, clear_cache_check},
};

use crate::{
  functionality::window::{after_build, setup_autostart},
  util::{helpers::move_injection_scripts, paths::injection_is_local},
};

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

  let context = tauri::generate_context!("tauri.conf.json");
  let dorion_open = process::process_already_exists();
  let client_type = config.client_type.unwrap_or("default".to_string());
  let mut url = String::new();

  if client_type == "default" {
    url += "https://discord.com/app";
  } else {
    url = format!("https://{}.discord.com/app", client_type);
  }

  let parsed = reqwest::Url::parse(&url).unwrap();
  let url_ext = tauri::WindowUrl::External(parsed);

  // If another process of Dorion is already open, show a dialog
  // in the future I want to actually *reveal* the other runnning process
  // instead of showing a popup, but this is fine for now
  if dorion_open && !config.multi_instance.unwrap_or(false) {
    // Send the dorion://open deep link request
    helpers::open_scheme("dorion://open".to_string()).unwrap_or_default();

    // Exit
    std::process::exit(0);
  }

  // Safemode check
  let safemode = std::env::args().any(|arg| arg == "--safemode");
  println!("Safemode enabled: {}", safemode);

  // Begin the RPC server
  let mut rpc_thread = None;

  if get_config().rpc_server.unwrap_or(false) {
    rpc_thread = Some(std::thread::spawn(|| {
      functionality::rpc::start_rpc_server();
    }));
  }

  #[allow(clippy::single_match)]
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .system_tray(create_systray())
    .invoke_handler(tauri::generate_handler![
      functionality::window::minimize,
      functionality::window::maximize,
      functionality::window::close,
      should_disable_plugins,
      css_preprocess::clear_css_cache,
      css_preprocess::localize_imports,
      js_preprocess::localize_all_js,
      local_html::get_index,
      local_html::get_top_bar,
      local_html::get_extra_css,
      notifications::notif_count,
      notifications::send_notification,
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
          event.window().hide().unwrap_or_default();
          api.prevent_close();

          return;
        }

        functionality::cache::maybe_clear_cache();
      }
      tauri::WindowEvent::Destroyed { .. } => {
        functionality::cache::maybe_clear_cache();
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
        match app.get_window("main") {
          Some(win) => {
            win.show().unwrap_or_default();
          }
          None => {}
        }
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        let window = match app.get_window("main") {
          Some(win) => win,
          None => return,
        };

        if id == "quit" {
          // Close the process
          window.close().unwrap_or_default();
        }

        if id == "open" {
          // Reopen the window
          window.show().unwrap_or_default();
          window.set_focus().unwrap_or_default();
          window.unminimize().unwrap_or_default();
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
        // Prevent flickering by starting hidden, and show later
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

      after_build(&win);

      setup_autostart(app);

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");

  // Join threads
  if let Some(rpc_thread) = rpc_thread {
    rpc_thread.join().unwrap();
  }
}
