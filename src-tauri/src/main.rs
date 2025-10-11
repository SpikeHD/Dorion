#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
use notify_rust::set_application;
use std::{env, str::FromStr, time::Duration};
use tauri::{Manager, Url, WebviewWindowBuilder};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use config::{Config, get_config, set_config};
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
  paths::{get_webdata_dir, is_portable},
  window_helpers::{self, clear_cache_check},
};

use crate::{
  functionality::{configure::configure, extension::load_extensions, window::setup_autostart},
  util::{
    logger,
    url::{get_client_app_url, get_client_url},
    window_helpers::ultrashow,
  },
};

mod args;
mod config;
mod functionality;
mod gpu;
mod injection;
mod processors;
mod profiles;
mod release;
mod util;
mod window;

const HASH: Option<&'static str> = std::option_env!("GIT_HASH");

#[cfg(target_os = "windows")]
pub fn additional_args() {
  // We set some of these internally, so make sure they stick around if we are about to add more
  let browser_args = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").unwrap_or_default();
  let new_args = args::get_webview_args();

  unsafe {
    std::env::set_var(
      "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
      format!("{browser_args} {new_args}"),
    );
  };

  log!("Running with the following WebView2 arguments: {browser_args} {new_args}");
}

#[tauri::command]
fn git_hash() -> String {
  HASH.unwrap_or("Unknown").to_string()
}

#[tauri::command]
fn should_disable_plugins() -> bool {
  args::is_safemode()
}

fn main() {
  if args::is_help() {
    return;
  }

  // Ensure config is created
  Config::init();

  // Also init logging
  logger::init(true);

  std::panic::set_hook(Box::new(|info| {
    log!("Panic occurred: {:?}", info);
  }));

  #[cfg(target_os = "windows")]
  log!("Are we on Windows 7: {}", helpers::is_windows_7());

  let mut config = get_config();

  // Check if the deprecated theme option is being used
  if let Some(theme) = config.theme
    && config.themes.is_none()
  {
    // If this is "none" then it's fine to leave the vec empty
    if theme != "none" {
      log!("Deprecated theme option detected, using \"none\" and setting `themes` instead...");

      config.themes = Option::from(vec![theme]);
      config.theme = Option::from("none".to_string());

      set_config(config.clone());
    }
  }

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

  log!("Are we portable? {}", is_portable());

  let context = tauri::generate_context!("tauri.conf.json");
  let url = get_client_app_url();

  #[cfg(target_os = "macos")]
  set_application(&context.config().identifier)
    .unwrap_or_else(|e| log!("Failed to set application: {:?}", e));

  log!(
    "Starting Dorion version v{}",
    context
      .config()
      .version
      .as_ref()
      .unwrap_or(&String::from("0.0.0"))
  );
  log!(
    "Opening Discord {}",
    config.client_type.unwrap_or("default".to_string())
  );

  let parsed = reqwest::Url::parse(&url).unwrap();
  let url_ext = tauri::WebviewUrl::External(parsed);
  let client_mods = load_mods_js();

  #[cfg(target_os = "windows")]
  additional_args();

  #[allow(clippy::single_match)]
  #[allow(unused_mut)]
  let mut builder = tauri::Builder::default();

  if !config.multi_instance.unwrap_or(false) {
    builder = builder.plugin(tauri_plugin_single_instance::init(
      move |app, _argv, _cwd| {
        if let Some(win) = app.get_webview_window("main") {
          ultrashow(win);
        }
      },
    ));
  }

  builder
    .plugin(tauri_plugin_deep_link::init())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent,
      Some(vec!["--startup"]),
    ))
    .plugin(tauri_plugin_process::init())
    .plugin(
      tauri_plugin_prevent_default::Builder::new()
        .with_flags(tauri_plugin_prevent_default::Flags::FIND)
        .build(),
    )
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .invoke_handler(tauri::generate_handler![
      should_disable_plugins,
      git_hash,
      functionality::extension::extension_injected,
      functionality::window::minimize,
      functionality::window::toggle_maximize,
      #[cfg(not(target_os = "macos"))]
      functionality::window::set_decorations,
      functionality::window::close,
      css_preprocess::clear_css_cache,
      css_preprocess::localize_imports,
      js_preprocess::localize_all_js,
      local_html::get_index,
      local_html::get_extra_css,
      notifications::notification_count,
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
      #[cfg(feature = "rpc")]
      #[cfg(not(target_os = "macos"))]
      functionality::rpc::get_windows,
      #[cfg(feature = "rpc")]
      #[cfg(not(target_os = "macos"))]
      functionality::rpc::get_local_detectables,
      #[cfg(feature = "hotkeys")]
      #[cfg(not(target_os = "macos"))]
      functionality::hotkeys::get_keybinds,
      #[cfg(feature = "hotkeys")]
      #[cfg(not(target_os = "macos"))]
      functionality::hotkeys::set_keybinds,
      #[cfg(feature = "hotkeys")]
      #[cfg(not(target_os = "macos"))]
      functionality::hotkeys::set_keybind,
      #[cfg(feature = "hotkeys")]
      #[cfg(all(not(target_os = "macos"), not(target_os = "linux")))]
      functionality::hotkeys::trigger_keys_pressed,
      functionality::tray::set_tray_icon,
      injection_runner::get_injection_js,
      config::get_config,
      config::set_config,
      config::read_config_file,
      config::write_config_file,
      config::default_config,
      theme::get_themes,
      theme::get_theme_names,
      theme::get_enabled_themes,
      theme::theme_from_link,
      helpers::get_platform,
      helpers::open_themes,
      helpers::open_plugins,
      helpers::open_extensions,
      helpers::fetch_image,
      #[cfg(feature = "blur")]
      window::blur::available_blurs,
      #[cfg(feature = "blur")]
      window::blur::apply_effect,
      // window::blur::remove_effect,
      window_helpers::remove_top_bar,
      window_helpers::set_clear_cache,
      window_helpers::ultrashow,
      window_helpers::window_zoom_level,
      util::color::get_os_accent,
    ])
    .on_window_event(|window, event| match event {
      tauri::WindowEvent::Resized { .. } => {
        // Sleep for a millisecond (blocks the thread but it doesn't really matter)
        // https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495
        std::thread::sleep(Duration::from_millis(1));
      }
      tauri::WindowEvent::Destroyed => {
        log!("Destroyed window");
        functionality::cache::maybe_clear_cache();
      }
      tauri::WindowEvent::CloseRequested { api, .. } => {
        // Just hide the window if the config calls for it
        if get_config().sys_tray.unwrap_or(false) {
          // https://github.com/tauri-apps/tauri/issues/3084#issuecomment-1477675840
          #[cfg(target_os = "macos")]
          tauri::AppHandle::hide(window.app_handle()).unwrap_or_default();

          #[cfg(not(target_os = "macos"))]
          window.hide().unwrap_or_default();
          api.prevent_close();
        }

        log!("Closing window");

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

      let config = get_config();
      let preinject = PREINJECT.clone();
      let title = format!("Dorion - v{}", app.package_info().version);
      let mut win = WebviewWindowBuilder::new(app, "main", url_ext)
        .title(title.as_str())
        .resizable(true)
        .disable_drag_drop_handler()
        .data_directory(get_webdata_dir())
        // Prevent flickering by starting hidden, and show later
        .visible(false)
        .decorations(true)
        .shadow(true)
        .transparent(
          config.blur.unwrap_or("none".to_string()) != "none"
        )
        .zoom_hotkeys_enabled(true)
        .browser_extensions_enabled(true);

      if !args::is_safemode() {
        // Preinject is bundled with "use strict" so we put it in it's own function to prevent potential client mod issues
        win = win.initialization_script(format!("console.log(window.location);if(window.__DORION_INIT__) {{throw new Error('Dorion already began initializing');}} window.__DORION_INIT__ = true; {preinject};{client_mods}").as_str());
      }

      if config.proxy_uri.is_some() || args::get_proxy().is_some() {
        // Prefer proxy from args if available
        let proxy = args::get_proxy().unwrap_or_else(|| config.proxy_uri.unwrap_or_default().to_string());

        if !proxy.is_empty() {
          log!("Using proxy: {proxy}");
          if let Ok(url) = Url::from_str(&proxy) {
            win = win.proxy_url(url);
          } else {
            log!("Invalid proxy URL: {proxy}");
            // We should exit, people using proxies probably don't want to use Dorion without it
            std::process::exit(1);
          }
        }
      }

      let win = win.build()?;

      // Prevent race condition by loading as early as possible
      if !args::is_safemode() {
        load_extensions(&win);
      }

      app.deep_link().on_open_url({
        let handle = app.handle().clone();

        move |event| {
          if let Some(url) = event.urls().first() {
            let path = url.path();
            log!("Deep link event: {path}");

            if let Some(win) = handle.get_webview_window("main") {
              let full_url = get_client_url();
              let url = Url::parse(format!("{full_url}{path}").as_str());

              if let Ok(url) = url {
                win.navigate(url).unwrap_or_default();
              }
            }
          }
        }
      });

      configure(&win);
      setup_autostart(app);

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");

  log!("App exited");
}
