use std::{fs, path::PathBuf};

use tauri::path::BaseDirectory;
use tauri::Manager;

use crate::config::{default_config, get_config};
use crate::log;

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
    // Create file if it doesn't exist
    if fs::metadata(&local_config_dir).is_err() {
      fs::write(
        &local_config_dir,
        serde_json::to_string_pretty(&default_config()).unwrap_or_default(),
      )
      .unwrap_or(());
    }

    return local_config_dir;
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let config_dir = appdata.join("dorion");

  if fs::metadata(&config_dir).is_err() {
    fs::create_dir_all(appdata.join("dorion")).expect("Error creating appdata dir");
  }

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
    if fs::metadata(&local_plugin_dir).is_err() {
      match fs::create_dir_all(&local_plugin_dir) {
        Ok(()) => (),
        Err(e) => {
          log!("Error creating local plugins dir: {}", e);
          return local_plugin_dir;
        }
      };
    }

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

  if fs::metadata(&plugin_dir).is_err() {
    match fs::create_dir_all(&plugin_dir) {
      Ok(()) => (),
      Err(e) => {
        log!("Error creating plugins dir: {}", e);
        return plugin_dir;
      }
    };
  }

  plugin_dir
}

pub fn get_theme_dir() -> std::path::PathBuf {
  // First see if there is a local theme dir
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_theme_dir = current_exe.parent().unwrap().join("themes");

  if is_portable() {
    // Create dir if it doesn't exist
    if fs::metadata(&local_theme_dir).is_err() {
      match fs::create_dir_all(&local_theme_dir) {
        Ok(()) => (),
        Err(e) => {
          log!("Error creating local themes dir: {}", e);
          return local_theme_dir;
        }
      };
    }

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

  if fs::metadata(&theme_dir).is_err() {
    match fs::create_dir_all(&theme_dir) {
      Ok(()) => (),
      Err(e) => {
        log!("Error creating theme dir: {}", e);
        return theme_dir;
      }
    };
  }

  // Also create theme cache dir
  let cache_dir = theme_dir.join("cache");

  if fs::metadata(&cache_dir).is_err() {
    match fs::create_dir_all(&cache_dir) {
      Ok(()) => (),
      Err(e) => {
        log!("Error creating theme cache dir: {}", e);
        return theme_dir;
      }
    };
  }

  theme_dir
}

pub fn profiles_dir() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();

  // Check for local/portable file paths
  if is_portable() {
    let profile_folder = current_exe.parent().unwrap().join("profiles");

    if fs::metadata(&profile_folder).is_err() {
      match fs::create_dir_all(&profile_folder) {
        Ok(()) => (),
        Err(e) => {
          log!("Error creating local profiles dir: {}", e);
          return profile_folder;
        }
      };
    }

    return profile_folder;
  }

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
