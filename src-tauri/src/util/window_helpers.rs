use crate::config::get_config;
use crate::log;

use super::paths::get_webdata_dir;

#[cfg(target_os = "windows")]
static OS: &str = "(Windows NT 10.0; Win64; x64)";
#[cfg(target_os = "macos")]
static OS: &str = "(Macintosh; Intel Mac OS X 10_15_7)";
#[cfg(target_os = "linux")]
static OS: &str = "(X11; Linux x86_64)";

fn useragent(chrome_version: Option<String>) -> String {
  let chrome_version = chrome_version.unwrap_or("131.0.0.0".to_string());

  format!(
    "Mozilla/5.0 {OS} AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{chrome_version} Safari/537.36"
  )
}

pub fn clear_cache_check() {
  let appdata = dirs::data_dir().unwrap_or_default().join("dorion");

  if !appdata.exists() {
    std::fs::create_dir_all(&appdata).expect("Failed to create dorion appdata dir!");
  }

  let cache_file = appdata.join("clear_cache");

  if cache_file.exists() {
    // Delete the file
    std::fs::remove_file(&cache_file).expect("Failed to remove clear_cache file!");
    clear_cache();
  }
}

#[tauri::command]
pub fn set_clear_cache(win: tauri::WebviewWindow) {
  // Create a file called "clear_cache" in the appdata dir
  // This will be read by the window when it closes
  let appdata = dirs::data_dir().unwrap_or_default().join("dorion");

  if !appdata.exists() {
    std::fs::create_dir_all(&appdata).expect("Failed to create dorion appdata dir!");
  }

  let cache_file = appdata.join("clear_cache");

  std::fs::write(cache_file, "").expect("Failed to create clear_cache file!");

  win.close().unwrap_or_default();
}

#[tauri::command]
pub fn clear_cache() {
  // Remove %appdata%/dorion/webdata
  let webdata_dir = get_webdata_dir();

  if webdata_dir.exists() {
    log!("Deleting cache...");
    std::fs::remove_dir_all(webdata_dir).expect("Failed to remove webdata dir!");
  }
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn window_zoom_level(win: tauri::WebviewWindow, value: Option<f64>) {
  win
    .with_webview(move |webview| unsafe {
      let zoom = value.unwrap_or(
        get_config()
          .zoom
          .unwrap_or("1.0".to_string())
          .parse::<f64>()
          .unwrap_or(1.0),
      );

      webview.controller().SetZoomFactor(zoom).unwrap_or_default();
    })
    .unwrap_or_default();
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn window_zoom_level(win: tauri::WebviewWindow, value: Option<f64>) {
  let zoom = value.unwrap_or(
    get_config()
      .zoom
      .unwrap_or("1.0".to_string())
      .parse::<f64>()
      .unwrap_or(1.0),
  );

  win
    .eval(&format!("document.body.style.zoom = '{zoom}'"))
    .expect("Failed to set zoom level!");
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn remove_top_bar(win: tauri::WebviewWindow) {
  win.set_decorations(false).unwrap_or(());
}

// Top bar is broken for MacOS currently
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn remove_top_bar(_win: tauri::WebviewWindow) {}

#[cfg(target_os = "windows")]
pub fn set_user_agent(win: &tauri::WebviewWindow) {
  use tauri::webview::PlatformWebview;
  use webview2_com::Microsoft::Web::WebView2::Win32::{ICoreWebView2_2, ICoreWebView2Settings2};
  use windows::core::{HSTRING, Interface, PWSTR};

  win
    .with_webview(|webview| unsafe {
      unsafe fn inner(webview: PlatformWebview) -> Result<(), Box<dyn std::error::Error>> {
        let wv = unsafe { webview.controller().CoreWebView2() }?.cast::<ICoreWebView2_2>()?;
        let settings = unsafe { wv.Settings() }?.cast::<ICoreWebView2Settings2>()?;
        let env = unsafe { wv.Environment() }?;
        let mut browser_version = PWSTR::null();

        unsafe { env.BrowserVersionString(&mut browser_version) }?;

        let browser_version = unsafe { browser_version.to_string() }?
          .chars()
          .take_while(|&c| c != '.')
          .collect::<String>();

        log!("Webview2 Chromium version: {browser_version}.0.0.0");

        let browser_version = if browser_version.is_empty() {
          None
        } else {
          Some(format!("{browser_version}.0.0.0"))
        };

        unsafe { settings.SetUserAgent(&HSTRING::from(useragent(browser_version))) }?;

        Ok(())
      }

      inner(webview).unwrap_or_else(|e| log!("Failed to set user-agent: {:?}", e));
    })
    .unwrap_or_else(|e| log!("Failed to set user-agent: {:?}", e));

  log!("Set user agent!");
}

#[cfg(target_os = "linux")]
pub fn set_user_agent(win: &tauri::WebviewWindow) {
  use webkit2gtk::{SettingsExt, WebViewExt};

  win
    .with_webview(|webview| {
      let webview = webview.inner();
      let settings = webview.settings().unwrap();

      settings.set_user_agent(Some(&useragent(None)));
    })
    .unwrap_or_else(|e| log!("Failed to set user-agent: {:?}", e));
}

#[cfg(target_os = "macos")]
pub fn set_user_agent(win: &tauri::WebviewWindow) {
  use objc2_foundation::NSString;
  use objc2_web_kit::WKWebView;

  win
    .with_webview(|webview| unsafe {
      let webview: &WKWebView = &*webview.inner().cast();
      let useragent = NSString::from_str(&useragent(None));

      webview.setCustomUserAgent(Some(&useragent));
    })
    .unwrap_or_else(|e| log!("Failed to set user-agent: {:?}", e));
}

/// Stupid name but this just ensures the window is visible regardless of being unfocused/minimized/hidden
#[tauri::command]
pub fn ultrashow(win: tauri::WebviewWindow) {
  win.unminimize().unwrap_or_default();
  win.show().unwrap_or_default();
  win.set_focus().unwrap_or_default();
}
