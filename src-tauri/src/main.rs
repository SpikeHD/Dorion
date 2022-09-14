#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{fs, path::PathBuf};
use tauri::{utils::config::AppUrl, Window, WindowBuilder};

mod config;
mod helpers;
mod theme;
mod injection;

#[tauri::command]
fn load_plugins() -> String {
  let mut contents = String::new();
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let plugins_dir = exe_dir.join("plugins");

  if fs::metadata(&plugins_dir).is_err() {
    match fs::create_dir_all(&plugins_dir) {
      Ok(()) => (),
      Err(e) => {
        println!("Error creating plugins dir: {}", e);

        return String::new();
      }
    };
  }

  let plugin_folders = fs::read_dir(&plugins_dir).unwrap();

  for path in plugin_folders {
    if let Err(_path) = path {
      continue;
    }

    let folder = path.unwrap().file_name().clone();
    let plugin_dir = plugins_dir.join(&folder);
    let index_file = plugin_dir.join("index.js");

    if folder.to_str().unwrap_or("").starts_with('_') {
      continue;
    }

    if fs::metadata(&index_file).is_ok() {
      let plugin_contents = fs::read_to_string(&index_file).unwrap();

      contents = format!("{};(() => {{ {} }})()", contents, plugin_contents);
    }
  }

  contents
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn change_zoom(window: tauri::Window, zoom: f64) {
  window
    .with_webview(move |webview| unsafe {
      webview.controller().SetZoomFactor(zoom).unwrap_or(());
    })
    .unwrap_or(());
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn change_zoom(window: tauri::Window, zoom: f64) {
  use webkit2gtk::WebViewExt;

  window
    .with_webview(move |webview| {
      webview.inner().set_zoom_level(zoom);
    })
    .unwrap_or(());
}

#[cfg(target_os = "macos")]
#[tauri::command]
fn change_zoom(_window: tauri::Window, _zoom: f64) {}

fn main() {
  let mut context = tauri::generate_context!("tauri.conf.json");
  let win_url = tauri::WindowUrl::App(PathBuf::from("../dist"));

  // For ensuring config exists
  config::init();

  context.config_mut().build.dist_dir = AppUrl::Url(win_url.clone());
  context.config_mut().build.dev_path = AppUrl::Url(win_url.clone());

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      load_plugins,
      change_zoom,
      injection::get_injection_js,
      injection::load_injection_js,
      config::read_config_file,
      config::write_config_file,
      theme::get_theme,
      theme::get_theme_names,
      helpers::open_themes,
      helpers::open_plugins
    ])
    .setup(move |app| {
      let title = format!("Dorion - v{}", app.package_info().version);
      let win = WindowBuilder::new(app, "main", win_url)
        .title(title.as_str())
        .resizable(true)
        .build()?;

      modify_window(win);

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
}

// Big fat credit to icidasset & FabianLars
// https://github.com/icidasset/diffuse/blob/main/src-tauri/src/main.rs
fn modify_window(window: Window) {
  window
    .with_webview(move |webview| {
      #[cfg(windows)]
      unsafe {
        use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings2;
        use windows::core::Interface;

        let settings: ICoreWebView2Settings2 = webview
          .controller()
          .CoreWebView2()
          .unwrap()
          .Settings()
          .unwrap()
          .cast()
          .unwrap();

        // settings.SetUserAgent(user_agent).unwrap();
        settings.SetIsZoomControlEnabled(true).unwrap();

        // Grab and set this config option, it's fine if it silently fails
        webview
          .controller()
          .SetZoomFactor(config::get_zoom())
          .unwrap_or(());
      }

      #[cfg(target_os = "linux")]
      {
        use webkit2gtk::WebViewExt;
        let webview = webview.inner();
        //let settings = webview.settings().unwrap();

        webview.set_zoom_level(config::get_zoom());
      }

      // untested
      // #[cfg(target_os = "macos")]
      // unsafe {
      //   use objc::{msg_send, sel, sel_impl};
      //   use objc_foundation::{INSString, NSString};
      //   let agent = NSString::from_str(user_agent);

      //   // TODO: zoom level n stuff
      // }
    })
    .unwrap();
}
