#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::time::Duration;
use tauri::{Manager, WebviewWindowBuilder};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

use config::get_config;
use injection::{
  client_mod::{self, load_mods_js},
  injection_runner::{self, PREINJECT},
  local_html, plugin, theme,
};
use processors::{css_preprocess, js_preprocess};
use profiles::init_profiles_folders;
use util::{
  helpers,
  logger::log,
  notifications,
  paths::get_webdata_dir,
  window_helpers::{self, clear_cache_check, set_user_agent},
};

use crate::{
  functionality::window::{after_build, setup_autostart},
  util::logger,
};

mod config;
mod functionality;
mod gpu;
mod injection;
mod processors;
mod profiles;
mod release;
mod util;
mod window;

#[tauri::command]
fn should_disable_plugins() -> bool {
  std::env::args().any(|arg| arg == "--disable-plugins")
}

fn main() {
  // Ensure config is created
  config::init();

  // Also init logging
  logger::init(true);

  std::panic::set_hook(Box::new(|info| {
    log!("Panic occurred: {:?}", info);
  }));

  let config = get_config();

  std::thread::sleep(Duration::from_millis(200));

  // before anything else, check if the clear_cache file exists
  clear_cache_check();

  // Run the steps to init profiles
  init_profiles_folders();

  // maybe disable hardware acceleration for windows
  if config.disable_hardware_accel.unwrap_or(false) {
    #[cfg(target_os = "windows")]
    gpu::disable_hardware_accel_windows();
  }

  #[cfg(target_os = "linux")]
  gpu::disable_dma();

  let context = tauri::generate_context!("tauri.conf.json");
  let client_type = config.client_type.unwrap_or("default".to_string());
  let mut url = String::new();

  log!(
    "Starting Dorion version v{}",
    context
      .config()
      .version
      .as_ref()
      .unwrap_or(&String::from("0.0.0"))
  );
  log!("Opening Discord {}", client_type);

  if client_type == "default" {
    url += "https://discord.com/app";
  } else {
    url = format!("https://{}.discord.com/app", client_type);
  }

  let parsed = reqwest::Url::parse(&url).unwrap();
  let url_ext = tauri::WebviewUrl::External(parsed);

  // Safemode check
  let safemode = std::env::args().any(|arg| arg == "--safemode");
  log!("Safemode enabled: {}", safemode);

  let client_mods = load_mods_js();

  #[allow(clippy::single_match)]
  tauri::Builder::default()
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent,
      None,
    ))
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .invoke_handler(tauri::generate_handler![
      should_disable_plugins,
      functionality::window::minimize,
      functionality::window::toggle_maximize,
      functionality::window::close,
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
      client_mod::available_mods,
      client_mod::load_mods_css,
      profiles::get_profile_list,
      profiles::get_current_profile_folder,
      profiles::create_profile,
      profiles::delete_profile,
      release::do_update,
      release::update_check,
      functionality::rpc::get_windows,
      functionality::rpc::get_local_detectables,
      functionality::hotkeys::get_keybinds,
      functionality::hotkeys::set_keybinds,
      functionality::hotkeys::set_keybind,
      injection_runner::get_injection_js,
      config::get_config,
      config::set_config,
      config::read_config_file,
      config::write_config_file,
      config::default_config,
      theme::get_theme,
      theme::get_theme_names,
      theme::theme_from_link,
      helpers::get_platform,
      helpers::open_themes,
      helpers::open_plugins,
      helpers::fetch_image,
      window::blur::available_blurs,
      window::blur::apply_effect,
      // window::blur::remove_effect,
      window_helpers::remove_top_bar,
      window_helpers::set_clear_cache,
      window_helpers::window_zoom_level,
      functionality::tray::set_tray_icon,
    ])
    .on_window_event(|window, event| match event {
      tauri::WindowEvent::Resized { .. } => {
        // Sleep for a millisecond (blocks the thread but it doesn't really matter)
        // https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495
        std::thread::sleep(Duration::from_millis(1));
      }
      tauri::WindowEvent::Destroyed { .. } => {
        functionality::cache::maybe_clear_cache();
      }
      tauri::WindowEvent::CloseRequested { api, .. } => {
        // Just hide the window if the config calls for it
        if get_config().sys_tray.unwrap_or(false) {
          window.hide().unwrap_or_default();
          api.prevent_close();
        }

        window
          .app_handle()
          .save_window_state(StateFlags::all())
          .unwrap_or_default();
      }
      _ => {}
    })
    .setup(move |app: &mut tauri::App| {
      // Init plugin list
      plugin::get_new_plugins();

      // First, grab preload plugins
      let title = format!("Dorion - v{}", app.package_info().version);
      let win = WebviewWindowBuilder::new(app, "main", url_ext)
        .title(title.as_str())
        .initialization_script(
          format!(r#"
            {};{}
          "#,
            PREINJECT.clone(),
            client_mods,
          ).as_str()
        )
        .resizable(true)
        .min_inner_size(100.0, 100.0)
        .disable_drag_drop_handler()
        .data_directory(get_webdata_dir())
        // Prevent flickering by starting hidden, and show later
        .visible(false)
        .decorations(true)
        .shadow(true)
        .transparent(
          config.blur.unwrap_or("none".to_string()) != "none"
        )
        .build()?;

      // Set the user agent to one that enables all normal Discord features
      set_user_agent(&win);

      // Multi-instance check
      if !config.multi_instance.unwrap_or(false) {
        log!("Multi-instance disabled, registering single instance plugin...");

        app.handle().plugin(tauri_plugin_single_instance::init(move |app, _argv, _cwd| {
          let win = match app.get_webview_window("main") {
            Some(win) => win,
            None => {
              log!("No windows open with name \"main\"(???)");
              return;
            },
          };
    
          win.set_focus().unwrap_or_default();
          win.unminimize().unwrap_or_default();
          win.show().unwrap_or_default();
        })).unwrap_or_else(|_| log!("Failed to register single instance plugin"));
      }

      // If safemode is enabled, stop here
      if safemode {
        win.show().unwrap_or_default();
        return Ok(());
      }

      // restore state BEFORE after_build, since that may change the window
      win.restore_state(StateFlags::all()).unwrap_or_default();

      plugin::load_plugins(win.clone(), Some(true));

      // begin the RPC server if needed
      if get_config().rpc_server.unwrap_or(false) {
        let win_cln = win.clone();
        std::thread::spawn(|| {
          functionality::rpc::start_rpc_server(win_cln);
        });
      }

      after_build(&win);

      setup_autostart(app);

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
}
