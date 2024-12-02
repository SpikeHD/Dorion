use std::fs;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::Manager;

use crate::functionality::extension::add_extension;
use crate::util::paths::get_main_extension_path;

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

  add_extension(window, main_ext_path.clone());

  // Refresh the page to ensure extensions are loaded
  window.eval("window.location.reload();").unwrap_or_default();
}
