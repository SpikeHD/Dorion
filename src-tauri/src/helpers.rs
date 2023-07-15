use std::path::*;
use std::process::Command;

#[tauri::command]
pub fn open_plugins() {
  let plugin_folder = tauri::api::path::home_dir()
    .unwrap()
    .join("dorion")
    .join("plugins");

  open_folder(plugin_folder)
}

#[tauri::command]
pub fn open_themes() {
  let theme_folder = tauri::api::path::home_dir()
    .unwrap()
    .join("dorion")
    .join("themes");

  open_folder(theme_folder)
}

#[cfg(target_os = "windows")]
fn open_folder(path: PathBuf) {
  Command::new("explorer").arg(path).spawn().unwrap();
}

#[cfg(target_os = "macos")]
fn open_folder(path: PathBuf) {
  Command::new("open").arg(path).spawn().unwrap();
}

#[cfg(target_os = "linux")]
fn open_folder(path: PathBuf) {
  Command::new("xdg-open").arg(path).spawn().unwrap();
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn clear_cache(win: tauri::Window) {
  unsafe {
    // Set zoom level
    use objc::{msg_send, sel, sel_impl, class};
    use cocoa::base::id;

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
pub fn clear_cache(win: tauri::Window) {
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
pub fn clear_cache(win: tauri::Window) {
  // win.with_webview(|webview| {
  //   webview.clear_all_browsing_data();
  // });
}
