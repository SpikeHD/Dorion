use std::path::PathBuf;

use tauri::path::BaseDirectory;
use tauri::Manager;
use webkit2gtk::{PermissionRequestExt, SettingsExt, WebContextExt, WebViewExt};

use crate::gpu::disable_hardware_accel_linux;
use crate::log;

pub fn configure(window: &tauri::WebviewWindow) {
  let handle = window.app_handle().clone();

  disable_hardware_accel_linux(window);
  enable_webrtc(window);

  // Extension patch
  window
    .with_webview(move |webview| {
      let webview = webview.inner();
      let context = webview.context();
      let path = handle
        .path()
        .resolve(PathBuf::from("extension_webkit"), BaseDirectory::Resource)
        .unwrap_or_default();

      if let Some(context) = context {
        let path_str = path.as_os_str().to_str();

        if let Some(path_str) = path_str {
          context.set_web_extensions_directory(path_str);
        }
      }
    })
    .unwrap_or_else(|e| log!("Failed to set web extensions directory: {:?}", e));
}

pub fn enable_webrtc(window: &tauri::WebviewWindow) {
  window
    .with_webview(move |webview| {
      let wv = webview.inner();
      let settings = WebViewExt::settings(&wv).unwrap_or_default();

      settings.set_enable_webrtc(true);
      settings.set_enable_media_stream(true);

      // We also need to handle permission requests
      wv.connect_permission_request(|_, req| {
        req.allow();
        true
      });
    })
    .unwrap_or_else(|_| log!("Failed to enable WebRTC"));
}
