use std::sync::atomic::AtomicBool;

use tauri::WebviewWindow;

use crate::log;

static EXTENSION_INJECTED: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "windows")]
pub fn add_extension(win: &WebviewWindow, path: std::path::PathBuf) {
  use std::path::PathBuf;
  use tauri::webview::PlatformWebview;
  use webview2_com::{
    Microsoft::Web::WebView2::Win32::{ICoreWebView2Profile7, ICoreWebView2_13},
    ProfileAddBrowserExtensionCompletedHandler,
  };
  use windows::core::{Interface, HSTRING};

  win
    .with_webview(move |webview| unsafe {
      pub unsafe fn add(
        webview: PlatformWebview,
        path: PathBuf,
      ) -> Result<(), Box<dyn std::error::Error>> {
        let profile = webview
          .controller()
          .CoreWebView2()?
          .cast::<ICoreWebView2_13>()?
          .Profile()?
          .cast::<ICoreWebView2Profile7>()?;

        log!("Attempting to add extension {:?}", path);

        let handler =
          ProfileAddBrowserExtensionCompletedHandler::create(Box::new(|result, _ext| {
            match result {
              Ok(_) => {
                log!("Extension added successfully!");
                EXTENSION_INJECTED.store(true, std::sync::atomic::Ordering::Relaxed);
              }
              Err(e) => {
                log!("Failed to add extension: {:?}", e);
              }
            }

            Ok(())
          }));

        if !path.exists() {
          return Err("Extension folder does not exist!".into());
        }

        let path_str = path.to_str().unwrap_or_default();
        let ext = HSTRING::from(path_str);

        profile
          .AddBrowserExtension(&ext, &handler)
          .unwrap_or_else(|e| log!("Failed to add extension: {:?}", e));

        Ok(())
      }

      add(webview, path).unwrap_or_else(|e| log!("Failed to add extension: {:?}", e));
    })
    .unwrap_or_default();
}

#[tauri::command]
pub fn extension_injected() -> bool {
  #[cfg(target_os = "windows")]
  {
    use crate::args;

    if args::is_legacy_fetch() {
      return false;
    }
  }

  EXTENSION_INJECTED.load(std::sync::atomic::Ordering::Relaxed)
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
