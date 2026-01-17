use std::fs;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::webview::PlatformWebview;
use tauri::Manager;
use webview2_com::Microsoft::Web::WebView2::Win32::{
  ICoreWebView2Profile4, ICoreWebView2_13, COREWEBVIEW2_PERMISSION_KIND_CAMERA,
  COREWEBVIEW2_PERMISSION_KIND_MICROPHONE, COREWEBVIEW2_PERMISSION_STATE_ALLOW,
};
use webview2_com::SetPermissionStateCompletedHandler;
use windows::core::{Interface, HSTRING};

use crate::functionality::extension::add_extension;
use crate::log;
use crate::util::paths::get_main_extension_path;
use crate::util::url::get_client_url;

pub fn configure(window: &tauri::WebviewWindow) {
  install_extension(window);
}

pub fn install_extension(window: &tauri::WebviewWindow) {
  // This should be the last extension loaded, the others are loaded early on
  let main_ext_res_path = window
    .app_handle()
    .path()
    .resolve(PathBuf::from("extension"), BaseDirectory::Resource)
    .unwrap_or_default();

  let main_ext_path = get_main_extension_path();

  // TODO on Windows, this needs to be copied somewhere more accessible to the user for some reason
  // Copy the files in the resource dir to the main extension dir if the files don't already exist
  if let Ok(read_dir) = fs::read_dir(main_ext_res_path) {
    for file in read_dir.flatten() {
      let file_path = file.path();
      let file_name = file_path.clone();
      let file_name = file_name.file_name().unwrap_or_default();

      fs::copy(file_path, main_ext_path.join(file_name)).unwrap_or_default();
    }
  }

  // Setup microphone and camera permissions
  window
    .with_webview(move |webview| {
      fn set_perms(webview: &PlatformWebview) -> Result<(), Box<dyn std::error::Error>> {
        let profile = unsafe {
          webview
            .controller()
            .CoreWebView2()?
            .cast::<ICoreWebView2_13>()?
            .Profile()?
            .cast::<ICoreWebView2Profile4>()?
        };
        let dummy_handler = SetPermissionStateCompletedHandler::create(Box::new(|result| {
          match result {
            Ok(_) => {
              log!("Set permission successfully!");
            }
            Err(e) => {
              log!("Failed to set permission: {:?}", e);
            }
          }

          Ok(())
        }));

        let origin = HSTRING::from(get_client_url());

        unsafe {
          profile.SetPermissionState(
            COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
            &origin,
            COREWEBVIEW2_PERMISSION_STATE_ALLOW,
            &dummy_handler,
          )?;

          profile.SetPermissionState(
            COREWEBVIEW2_PERMISSION_KIND_CAMERA,
            &origin,
            COREWEBVIEW2_PERMISSION_STATE_ALLOW,
            &dummy_handler,
          )?;
        };

        Ok(())
      }

      set_perms(&webview).unwrap_or_else(|e| log!("Failed to set permissions: {:?}", e));
    })
    .unwrap_or_default();

  add_extension(window, main_ext_path.clone());

  // Refresh the page to ensure extensions are loaded
  window.eval("window.location.reload();").unwrap_or_default();
}
