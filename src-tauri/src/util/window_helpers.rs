use crate::config;
use tauri::window::PlatformWebview;

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn _clear_cache(win: tauri::Window) {
  unsafe {
    use cocoa::base::id;
    use objc::{class, msg_send, sel, sel_impl};

    // let _: () = msg_send![webview.ns_window(), windowRef];

    let _ = win.with_webview(|webview| {
      // Disable cache for now
      let config: id = msg_send![webview.inner(), configuration];
      let store: id = msg_send![config, websiteDataStore];
      let all_data_types: id = msg_send![class!(WKWebsiteDataStore), allWebsiteDataTypes];
      let date: id = msg_send![class!(NSDate), dateWithTimeIntervalSince1970: 0.0];
      let handler = block::ConcreteBlock::new(|| {});
      let _: () = msg_send![store, removeDataOfTypes:all_data_types modifiedSince:date completionHandler:handler];
    });
  }
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn _clear_cache(_win: tauri::Window) {
  // win.with_webview(|webview| {
  //   if let Some(context) = WebViewExt::context(webview) {
  //     use webkit2gtk::WebContextExt;
  //     if let Some(data_manger) = context.website_data_manager() {
  //       webkit2gtk::WebsiteDataManagerExtManual::clear(
  //         &data_manger,
  //         webkit2gtk::WebsiteDataTypes::ALL,
  //         glib::TimeSpan::from_seconds(0),
  //         None::<&Cancellable>,
  //         |_| {},
  //       );
  //     }
  //   }
  // });
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn _clear_cache(_win: tauri::Window) {
  // win.with_webview(|webview| {
  //   use webview2_com::ClearBrowsingDataCompletedHandler;
  //   use webview2_com::Microsoft::Web::WebView2::Win32::{ICoreWebView2_13, ICoreWebView2Profile2};
  //   use windows::core::Interface;

  //   let handler = ClearBrowsingDataCompletedHandler::create(Box::new(move |_| Ok(())));
  //   unsafe {
  //     webview.controller()
  //       .cast::<ICoreWebView2_13>()
  //       .map_err(|e| Error::WebView2Error(webview2_com::Error::WindowsError(e)))?
  //       .Profile()
  //       .map_err(|e| Error::WebView2Error(webview2_com::Error::WindowsError(e)))?
  //       .cast::<ICoreWebView2Profile2>()
  //       .map_err(|e| Error::WebView2Error(webview2_com::Error::WindowsError(e)))?
  //       .ClearBrowsingDataAll(&handler)
  //       .map_err(|e| Error::WebView2Error(webview2_com::Error::WindowsError(e)))
  //   }
  // });
}

#[cfg(target_os = "windows")]
pub fn window_zoom_level(win: &tauri::Window) {
  win.with_webview(|webview| unsafe {
    let zoom = config::get_zoom();

    webview
      .controller()
      .SetZoomFactor(zoom)
      .unwrap_or_default();
  }).unwrap_or_default();
}

#[cfg(not(target_os = "windows"))]
pub fn window_zoom_level(win: &tauri::Window) {
  let zoom = config::get_zoom();

  win.eval(&format!("
    document.body.style.zoom = '{}';
  ", zoom)).expect("Failed to set zoom level!");
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn remove_top_bar(win: tauri::Window) {
  win.set_decorations(false).unwrap_or(());
}

// Top bar is broken for MacOS currently
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn remove_top_bar(_win: tauri::Window) {}
