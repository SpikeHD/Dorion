use tauri::WebviewWindow;

use crate::log;

#[cfg(target_os = "windows")]
pub fn add_extension(win: &WebviewWindow, path: std::path::PathBuf) {
  use webview2_com::{
    Microsoft::Web::WebView2::Win32::{ICoreWebView2Profile7, ICoreWebView2_13},
    ProfileAddBrowserExtensionCompletedHandler,
  };
  use windows::core::{Interface, HSTRING};

  win
    .with_webview(move |webview| unsafe {
      let core = match webview.controller().CoreWebView2() {
        Ok(profile) => profile,
        Err(e) => {
          log!("Failed to get CoreWebView2: {:?}", e);
          return;
        }
      };

      let casted = match core.cast::<ICoreWebView2_13>() {
        Ok(profile) => profile,
        Err(e) => {
          log!("Failed to cast webview: {:?}", e);
          return;
        }
      };

      let profile = match casted.Profile() {
        Ok(profile) => profile,
        Err(e) => {
          log!("Failed to get Profile: {:?}", e);
          return;
        }
      };

      let profile = match profile.cast::<ICoreWebView2Profile7>() {
        Ok(profile) => profile,
        Err(e) => {
          log!("Failed to cast profile: {:?}", e);
          return;
        }
      };

      log!("Attempting to add extension...");

      let handler = ProfileAddBrowserExtensionCompletedHandler::create(Box::new(|result, _ext| {
        log!("Extension added?: {:?}", result);
        Ok(())
      }));

      if !path.exists() {
        log!("Extension folder does not exist!");
        return;
      }

      let path_str = path.to_str().unwrap_or_default();
      let ext = HSTRING::from(path_str);

      profile
        .AddBrowserExtension(&ext, &handler)
        .unwrap_or_else(|e| log!("Failed to add extension: {:?}", e));
    })
    .unwrap_or_default();
}

#[cfg(target_os = "windows")]
pub fn load_extensions(win: &WebviewWindow) {
  use crate::util::paths::get_extensions_dir;
  use std::fs;

  log!("Loading extensions...");

  let extensions_dir = get_extensions_dir();

  // Read all files in the extensions dir
  if let Ok(files) = fs::read_dir(extensions_dir) {
    for file in files.flatten() {
      // Path can be file or folder, doesn't matter
      log!("Loading extension: {:?}", file.path());
      add_extension(win, file.path());
    }
  }
}

#[cfg(not(target_os = "windows"))]
pub fn load_extensions(_win: &WebviewWindow) {
  log!("Extensions are unsupported on non-Windows platforms!");
}
