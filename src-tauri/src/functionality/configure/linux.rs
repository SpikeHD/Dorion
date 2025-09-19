use std::path::PathBuf;

use tauri::path::BaseDirectory;
use tauri::Manager;
use webkit2gtk::{
  PermissionRequestExt, SecurityManagerExt, SettingsExt, WebContextExt, WebView, WebViewExt
};

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
      let settings = WebViewExt::settings(&webview).unwrap_or_default();
      let path = handle
        .path()
        .resolve(PathBuf::from("extension_webkit"), BaseDirectory::Resource)
        .unwrap_or_default();

      setup_popouts(&webview);

      settings.set_javascript_can_access_clipboard(true);
      settings.set_javascript_can_open_windows_automatically(true);

      if let Some(context) = context {
        let path_str = path.as_os_str().to_str();

        if let Some(path_str) = path_str {
          context.set_web_extensions_directory(path_str);
        }

        // Register `ws` as secure so we can connect to RPC
        if let Some(manager) = context.security_manager() {
          manager.register_uri_scheme_as_secure("ws");
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

// For stream popouts, etc.
pub fn setup_popouts(webview: &webkit2gtk::WebView) {
  use gtk::prelude::*;

  webview.connect_create(|wv, _action| {
    let new = WebView::with_related_view(wv);
    let settings = WebViewExt::settings(&new).unwrap();

    settings.set_enable_developer_extras(true);

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_default_size(800, 600);
    window.add(&new);

    let window_weak = window.downgrade();

    // When ready-to-show is fired, show the window
    new.connect_ready_to_show(move |_| {
      if let Some(window) = window_weak.upgrade() {
        window.show_all();
      }
    });

    Some(new.upcast::<gtk::Widget>())
  });
}
