#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use config::get_client_type;
use tauri::{utils::config::AppUrl, Window, WindowBuilder};

mod config;
mod css_preprocess;
mod helpers;
mod injection;
mod js_preprocess;
mod local_html;
mod plugin;
mod theme;

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
  // Ensure config is created
  config::init();

  let mut context = tauri::generate_context!("tauri.conf.json");
  let client_type = get_client_type();
  let mut url = String::new();

  if client_type == "default" {
    url += "https://discord.com/app";
  } else {
    url = format!("https://{}.discord.com/app", client_type);
  }

  let url_ext = tauri::WindowUrl::External(reqwest::Url::parse(&url).unwrap());

  context.config_mut().build.dist_dir = AppUrl::Url(url_ext.clone());
  context.config_mut().build.dev_path = AppUrl::Url(url_ext.clone());

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      change_zoom,
      css_preprocess::localize_imports,
      js_preprocess::localize_all_js,
      local_html::get_index,
      local_html::get_settings,
      plugin::load_plugins,
      plugin::get_plugin_list,
      plugin::toggle_plugin,
      plugin::toggle_preload,
      plugin::get_plugin_import_urls,
      injection::get_injection_js,
      injection::is_injected,
      injection::load_injection_js,
      config::read_config_file,
      config::write_config_file,
      theme::get_theme,
      theme::get_theme_names,
      helpers::open_themes,
      helpers::open_plugins
    ])
    .setup(move |app| {
      // First, grab preload plugins
      let preload_plugins = plugin::load_plugins(Option::Some(true));
      let title = format!("Dorion - v{}", app.package_info().version);
      let win = WindowBuilder::new(app, "main", url_ext)
        .title(title.as_str())
        .resizable(true)
        .build()?;
      
      modify_window(&win);

      // Execute preload scripts
      for (_name, script) in &preload_plugins {
        win.eval(script).unwrap_or(());
      }

      // Gotta make sure the window location is where it needs to be
      std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));

        injection::preinject(&win);
      });

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
}

// Big fat credit to icidasset & FabianLars
// https://github.com/icidasset/diffuse/blob/main/src-tauri/src/main.rs
fn modify_window(window: &Window) {
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
