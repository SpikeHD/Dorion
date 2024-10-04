use std::{fs, path::PathBuf};

use tauri::path::BaseDirectory;
use tauri::Manager;

use crate::config::{default_config, get_config};
use crate::log;

fn create_if_not_exists(path: &PathBuf) {
  if fs::metadata(path).is_err() {
    match fs::create_dir_all(path) {
      Ok(()) => (),
      Err(e) => {
        log!("Error creating dir: {}", e);
      }
    };
  }
}

pub fn is_portable() -> bool {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let portable_signifier = current_exe.parent().unwrap().join(".portable");

  fs::metadata(portable_signifier).is_ok()
}

pub fn get_config_dir() -> PathBuf {
  // First check for a local config file
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().to_path_buf();

  if is_portable() {
    return local_config_dir;
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let config_dir = appdata.join("dorion");

  create_if_not_exists(&config_dir);

  config_dir
}

pub fn get_config_file() -> PathBuf {
  let config_dir = get_config_dir();
  let config_file = config_dir.join("config.json");

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      &config_file,
      serde_json::to_string_pretty(&default_config()).unwrap_or_default(),
    )
    .unwrap_or(());
  }

  config_file
}

pub fn config_is_local() -> bool {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  fs::metadata(local_config_dir).is_ok()
}

pub fn get_plugin_dir() -> std::path::PathBuf {
  // First check for a local plugin dir
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_plugin_dir = current_exe.parent().unwrap().join("plugins");

  if is_portable() {
    // Create dir if it doesn't exist
    create_if_not_exists(&local_plugin_dir);

    return local_plugin_dir;
  }

  log!("No local plugin dir found. Using default.");

  #[cfg(target_os = "windows")]
  let plugin_dir = dirs::home_dir()
    .unwrap_or_default()
    .join("dorion")
    .join("plugins");

  #[cfg(not(target_os = "windows"))]
  let plugin_dir = dirs::config_dir()
    .unwrap_or_default()
    .join("dorion")
    .join("plugins");

  create_if_not_exists(&plugin_dir);

  plugin_dir
}

pub fn get_theme_dir() -> std::path::PathBuf {
  // First see if there is a local theme dir
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_theme_dir = current_exe.parent().unwrap().join("themes");

  if is_portable() {
    // Create dir if it doesn't exist
    create_if_not_exists(&local_theme_dir);

    return local_theme_dir;
  }

  log!("No local theme dir found. Using default.");

  #[cfg(target_os = "windows")]
  let theme_dir = dirs::home_dir()
    .unwrap_or_default()
    .join("dorion")
    .join("themes");

  #[cfg(not(target_os = "windows"))]
  let theme_dir = dirs::config_dir()
    .unwrap_or_default()
    .join("dorion")
    .join("themes");

  create_if_not_exists(&theme_dir);

  // Also create theme cache dir
  let cache_dir = theme_dir.join("cache");

  create_if_not_exists(&cache_dir);

  theme_dir
}

pub fn get_extensions_dir() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  // Check for local/portable file paths
  if is_portable() {
    let extensions_folder = current_exe.parent().unwrap().join("extensions");

    create_if_not_exists(&extensions_folder);

    return extensions_folder;
  }

  #[cfg(target_os = "windows")]
  let extensions_dir = dirs::home_dir()
    .unwrap_or_default()
    .join("dorion")
    .join("extensions");

  #[cfg(not(target_os = "windows"))]
  let extensions_dir = dirs::config_dir()
    .unwrap_or_default()
    .join("dorion")
    .join("extensions");

  create_if_not_exists(&extensions_dir);

  extensions_dir
}

#[cfg(target_os = "windows")]
pub fn get_main_extension_path() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  // Check for local/portable file paths
  if is_portable() {
    let extension_folder = current_exe.parent().unwrap().join("extension");

    create_if_not_exists(&extension_folder);

    return extension_folder;
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let extension_dir = appdata.join("dorion").join("extension");

  create_if_not_exists(&extension_dir);

  extension_dir
}

pub fn profiles_dir() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  // Check for local/portable file paths
  if is_portable() {
    let profile_folder = current_exe.parent().unwrap().join("profiles");

    create_if_not_exists(&profile_folder);

    return profile_folder;
  }

  // This is created automatically
  dirs::data_dir()
    .unwrap_or_default()
    .join("dorion")
    .join("profiles")
}

pub fn get_webdata_dir() -> PathBuf {
  let profile = get_config().profile.unwrap_or("default".to_string());
  let profiles = profiles_dir();

  profiles.join(profile).join("webdata")
}

pub fn updater_dir(win: &tauri::WebviewWindow) -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  if is_portable() {
    // This is a portable install, so we can use the local dir
    return current_exe.parent().unwrap().join("updater");
  }

  win
    .app_handle()
    .path()
    .resolve(PathBuf::from("updater"), BaseDirectory::Resource)
    .unwrap_or_default()
}

pub fn custom_detectables_path() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  if is_portable() {
    // This is a portable install, so we can use the local dir
    return current_exe.parent().unwrap().join("detectables.json");
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  appdata.join("dorion").join("detectables.json")
}

pub fn log_file_path() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  if is_portable() {
    // This is a portable install, so we can use the local dir
    return current_exe
      .parent()
      .unwrap()
      .join("logs")
      .join("latest.log");
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  appdata.join("dorion").join("logs").join("latest.log")
}
