#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{fs, path::PathBuf, time::Duration};
use tauri::{utils::config::AppUrl, Window, WindowBuilder};

#[tauri::command]
fn load_injection_js(window: tauri::Window, contents: String) {
  std::thread::spawn(move || {
    std::thread::sleep(Duration::from_millis(1000));
    window.eval(contents.as_str()).unwrap();
  });
}

#[tauri::command]
fn load_plugins() -> String {
  let mut contents = "".to_string();
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let plugins_dir = exe_dir.join("plugins");

  if !fs::metadata(&plugins_dir).is_ok() {
    fs::create_dir_all(&plugins_dir).unwrap();
  }

  let plugin_folders = fs::read_dir(&plugins_dir).unwrap();

  for path in plugin_folders {
    if let Err(_path) = path {
      continue;
    }

    let folder = path.unwrap().file_name().clone();
    let plugin_dir = plugins_dir.join(&folder);
    let index_file = plugin_dir.join("index.js");

    if folder.to_str().unwrap_or("").starts_with("_") {
      continue;
    }

    if fs::metadata(&index_file).is_ok() {
      let plugin_contents = fs::read_to_string(&index_file).unwrap();

      contents = format!("{};(() => {{ {} }})()", contents, plugin_contents);
    }
  }

  contents
}

fn main() {
  let mut context = tauri::generate_context!("tauri.conf.json");
  let win_url = tauri::WindowUrl::App(PathBuf::from("../dist"));

  context.config_mut().build.dist_dir = AppUrl::Url(win_url.clone());
  context.config_mut().build.dev_path = AppUrl::Url(win_url.clone());

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![load_injection_js, load_plugins])
    .setup(move |app| {
      let win = WindowBuilder::new(app, "main", win_url)
        .title("Dorion")
        .maximized(true)
        .resizable(true)
        .build()?;

      set_user_agent(win);

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
}

// Big fat credit to icidasset & FabianLars
// https://github.com/icidasset/diffuse/blob/main/src-tauri/src/main.rs
fn set_user_agent(window: Window) {
  let user_agent = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Dorion/1.0.1018 Chrome/91.0.4472.164 Electron/13.6.6 Safari/537.36";

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

        settings.SetUserAgent(user_agent).unwrap();

        settings.SetIsZoomControlEnabled(true).unwrap();
      }

      #[cfg(target_os = "linux")]
      {
        use webkit2gtk::{SettingsExt, WebViewExt};
        let webview = webview.inner();
        let settings = webview.settings().unwrap();
        settings.set_user_agent(Some(user_agent));
      }

      // untested
      #[cfg(target_os = "macos")]
      unsafe {
        use objc::{msg_send, sel, sel_impl};
        use objc_foundation::{INSString, NSString};
        let agent = NSString::from_str(user_agent);
        let () = msg_send![webview.inner(), setCustomUserAgent: agent];
      }
    })
    .unwrap();
}
