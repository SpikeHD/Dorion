use tauri::WebviewWindow;
use crate::log;

#[cfg(target_os = "windows")]
pub fn add_extension(win: &WebviewWindow) {
  use webview2_com::{Microsoft::Web::WebView2::Win32::{
    ICoreWebView2Profile7, ICoreWebView2_13, ICoreWebView2EnvironmentOptions6,
  }, ProfileAddBrowserExtensionCompletedHandler};
  use windows::core::{Interface, HSTRING};

  win.with_webview(|webview| unsafe {
    let core = match webview
      .controller()
      .CoreWebView2() {
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

    let environment = match casted.Environment() {
      Ok(environment) => environment,
      Err(e) => {
        log!("Failed to get Environment: {:?}", e);
        return;
      }
    };

    let environment = match environment.cast::<ICoreWebView2EnvironmentOptions6>() {
      Ok(environment) => environment,
      Err(e) => {
        log!("Failed to cast environment: {:?}", e);
        return;
      }
    };

    match environment.SetAreBrowserExtensionsEnabled(true) {
      Ok(_) => (),
      Err(e) => log!("Failed to set browser extensions enabled: {:?}", e),
    };

    log!("Attempting to add extension...");
    
    let handler = ProfileAddBrowserExtensionCompletedHandler::create(Box::new(|result, _ext| {
      log!("Extension added?: {:?}", result);
      Ok(())
    }));

    let exe_path = match std::env::current_exe() {
      Ok(path) => path,
      Err(e) => {
        log!("Failed to get current exe path: {:?}", e);
        return;
      }
    };

    let parent = match exe_path.parent() {
      Some(path) => path,
      None => {
        log!("Failed to get parent path of current exe path!");
        return;
      }
    };

    let mut ext = parent.to_path_buf();
    ext.push("extension");

    if !ext.exists() {
      log!("Extension folder does not exist!");
      return;
    }

    let path_str = ext.to_str().unwrap_or_default();
    let ext = HSTRING::from(path_str);

    profile.AddBrowserExtension(&ext, &handler).unwrap_or_else(|e| log!("Failed to add extension: {:?}", e));
  }).unwrap_or_default();
}

#[cfg(not(target_os = "windows"))]
pub fn add_extension(_win: &WebviewWindow) {
  log!("Extension is unsupported on non-Windows platforms!");
}
