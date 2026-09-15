use crate::config::{get_config, set_config};
use crate::log;

use super::paths::get_webdata_dir;

#[cfg(target_os = "windows")]
static OS: &str = "(Windows NT 10.0; Win64; x64)";
#[cfg(target_os = "macos")]
static OS: &str = "(Macintosh; Intel Mac OS X 10_15_7)";
#[cfg(target_os = "linux")]
static OS: &str = "(X11; Linux x86_64)";

#[cfg(target_os = "windows")]
fn useragent(chrome_version: Option<String>) -> String {
  let chrome_version = chrome_version.unwrap_or("138.0.0.0".to_string());

  format!(
    "Mozilla/5.0 {OS} AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{chrome_version} Safari/537.36"
  )
}

#[cfg(not(target_os = "windows"))]
fn useragent(_chrome_version: Option<String>) -> String {
  format!("Mozilla/5.0 {OS} AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15")
    .to_string()
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

fn persist_zoom(zoom: f64) {
  let mut config = get_config();
  config.zoom = Some(zoom.to_string());
  set_config(config);
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn window_zoom_level(win: tauri::WebviewWindow, value: Option<f64>) {
  let zoom = value.unwrap_or(
    get_config()
      .zoom
      .unwrap_or("1.0".to_string())
      .parse::<f64>()
      .unwrap_or(1.0),
  );

  if value.is_some() {
    persist_zoom(zoom);
  }

  win
    .with_webview(move |webview| unsafe {
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

  if value.is_some() {
    persist_zoom(zoom);
  }

  win
    .eval(format!("document.body.style.zoom = '{zoom}'"))
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
        let wv = webview
          .controller()
          .CoreWebView2()?
          .cast::<ICoreWebView2_2>()?;
        let settings = wv.Settings()?.cast::<ICoreWebView2Settings2>()?;
        let env = wv.Environment()?;
        let mut browser_version = PWSTR::null();

        env.BrowserVersionString(&mut browser_version)?;

        let browser_version = browser_version
          .to_string()?
          .chars()
          .take_while(|&c| c != '.')
          .collect::<String>();

        log!("Webview2 Chromium version: {browser_version}.0.0.0");

        let browser_version = if browser_version.is_empty() {
          None
        } else {
          Some(format!("{browser_version}.0.0.0"))
        };

        settings.SetUserAgent(&HSTRING::from(useragent(browser_version)))?;

        Ok(())
      }

      inner(webview).unwrap_or_else(|e| log!("Failed to set user-agent: {:?}", e));
    })
    .unwrap_or_else(|e| log!("Failed to set user-agent: {:?}", e));

  log!("Set user agent!");
}

#[cfg(target_os = "windows")]
#[derive(Default)]
struct WebviewKeybind {
  key: u32,
  shift: bool,
  ctrl: bool,
  alt: bool,
}

#[cfg(target_os = "windows")]
const ESCAPE_KEY: u32 = 0x1B;

#[cfg(target_os = "windows")]
const KEYBINDS_TO_DISABLE: &[WebviewKeybind] = &[WebviewKeybind {
  key: ESCAPE_KEY,
  shift: true,
  ctrl: false,
  alt: false,
}];

#[cfg(target_os = "windows")]
pub fn disable_webview_keybinds(win: &tauri::WebviewWindow) {
  use tauri::webview::PlatformWebview;
  use webview2_com::{
    AcceleratorKeyPressedEventHandler,
    Microsoft::Web::WebView2::Win32::{
      COREWEBVIEW2_KEY_EVENT_KIND_KEY_DOWN, ICoreWebView2AcceleratorKeyPressedEventArgs2,
    },
  };
  use windows::{
    Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VK_CONTROL, VK_MENU, VK_SHIFT},
    core::Interface,
  };

  win
    .with_webview(move |webview| {
      fn subscribe(webview: PlatformWebview) -> Result<(), Box<dyn std::error::Error>> {
        let controller = webview.controller();

        let handler = AcceleratorKeyPressedEventHandler::create(Box::new(|_sender, args| {
          if let Some(args) = args {
            let mut kind = COREWEBVIEW2_KEY_EVENT_KIND_KEY_DOWN;
            let mut key = 0u32;

            if unsafe { args.KeyEventKind(&mut kind).is_ok() && args.VirtualKey(&mut key).is_ok() }
              && kind == COREWEBVIEW2_KEY_EVENT_KIND_KEY_DOWN
            {
              let shift = unsafe { GetKeyState(VK_SHIFT.0 as i32) < 0 };
              let ctrl = unsafe { GetKeyState(VK_CONTROL.0 as i32) < 0 };
              let alt = unsafe { GetKeyState(VK_MENU.0 as i32) < 0 };

              let matched = KEYBINDS_TO_DISABLE.iter().any(|bind| {
                bind.key == key && bind.shift == shift && bind.ctrl == ctrl && bind.alt == alt
              });

              if matched
                && let Ok(ext) = args.cast::<ICoreWebView2AcceleratorKeyPressedEventArgs2>()
              {
                unsafe {
                  ext
                    .SetIsBrowserAcceleratorKeyEnabled(false)
                    .unwrap_or_default();
                }
              }
            }
          }

          Ok(())
        }));

        let mut token = 0i64;
        unsafe {
          controller.add_AcceleratorKeyPressed(&handler, &mut token)?;
        }

        Ok(())
      }

      subscribe(webview).unwrap_or_else(|e| log!("Failed to disable webview keybinds: {:?}", e));
    })
    .unwrap_or_else(|e| log!("Failed to disable webview keybinds: {:?}", e));
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
