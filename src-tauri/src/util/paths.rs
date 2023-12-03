use std::{fs, path::PathBuf};

use tauri::Manager;

use super::helpers::move_injection_scripts;
use crate::config::get_config;

pub fn get_config_dir() -> PathBuf {
  // First check for a local config file
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if fs::metadata(&local_config_dir).is_ok() {
    return local_config_dir;
  }

  println!("No local config file found. Using default.");

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let config_file = appdata.join("dorion").join("config.json");

  if fs::metadata(appdata.join("dorion")).is_err() {
    fs::create_dir_all(appdata.join("dorion")).expect("Error creating appdata dir");
  }

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      &config_file,
      r#"{ "theme": "none", "zoom": "1.0", "client_type": "default", "sys_tray": false, "block_telemetry": false }"#,
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

pub fn get_injection_dir(win: Option<&tauri::Window>) -> PathBuf {
  // Check if config is local, and if so, create (if needed) and use local injection dir
  if config_is_local() {
    let current_exe = std::env::current_exe().unwrap_or_default();
    let local_inject_dir = current_exe.parent().unwrap().join("injection");

    if fs::metadata(&local_inject_dir).is_err() {
      match fs::create_dir_all(&local_inject_dir) {
        Ok(()) => (),
        Err(e) => {
          println!("Error creating local injection dir: {}", e);
          return local_inject_dir;
        }
      };
    }

    return local_inject_dir;
  }

  // If not, grab the normal one
  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let injection_dir = appdata.join("dorion").join("injection");

  if fs::metadata(&injection_dir).is_err() {
    match fs::create_dir_all(&injection_dir) {
      Ok(()) => {
        // If we were passed the window, we can also copy the injection files
        if win.is_none() {
          return injection_dir;
        }

        move_injection_scripts(win.unwrap(), true);
      }
      Err(e) => {
        println!("Error creating injection dir: {}", e);
        return injection_dir;
      }
    };
  }

  // Check if "shelter.js" exists in the dir
  let injection_js = injection_dir.join("shelter.js");

  if let Some(win) = win {
    if fs::metadata(injection_js).is_ok() {
      return injection_dir;
    }

    println!("Moving injection scripts");

    move_injection_scripts(win, true);
  }

  injection_dir
}

pub fn injection_is_local() -> bool {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_inject_dir = current_exe.parent().unwrap().join("injection");

  fs::metadata(local_inject_dir).is_ok()
}

pub fn get_plugin_dir() -> std::path::PathBuf {
  // First check for a local plugin dir
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_plugin_dir = current_exe.parent().unwrap().join("plugins");

  if fs::metadata(&local_plugin_dir).is_ok() {
    return local_plugin_dir;
  }

  println!("No local plugin dir found. Using default.");

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
        println!("Error creating plugins dir: {}", e);
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

  if fs::metadata(&local_theme_dir).is_ok() {
    return local_theme_dir;
  }

  println!("No local theme dir found. Using default.");

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
        println!("Error creating theme dir: {}", e);
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
        println!("Error creating theme cache dir: {}", e);
        return theme_dir;
      }
    };
  }

  theme_dir
}

pub fn profiles_dir() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  // Check for local/portable file paths
  if local_config_dir.exists() {
    let profile_folder = local_config_dir.parent().unwrap().join("profiles");

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

pub fn updater_dir(win: &tauri::Window) -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if fs::metadata(local_config_dir).is_ok() {
    // This is a portable install, so we can use the local injection dir
    return current_exe.parent().unwrap().join("updater");
  }

  win
    .app_handle()
    .path_resolver()
    .resolve_resource(PathBuf::from("updater"))
    .unwrap_or_default()
}

pub fn custom_detectables_path() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if fs::metadata(local_config_dir).is_ok() {
    // This is a portable install, so we can use the local injection dir
    return current_exe.parent().unwrap().join("detectables.json");
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  appdata.join("dorion").join("detectables.json")
}
