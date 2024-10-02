use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::functionality::hotkeys::KeyStruct;
use crate::log;
use crate::util::paths::get_config_file;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  /// Deprecated
  pub theme: Option<String>,

  pub themes: Option<Vec<String>>,
  pub zoom: Option<String>,
  pub client_type: Option<String>,
  pub sys_tray: Option<bool>,
  pub push_to_talk: Option<bool>,
  pub push_to_talk_keys: Option<Vec<String>>,
  pub cache_css: Option<bool>,
  pub use_native_titlebar: Option<bool>,
  pub start_maximized: Option<bool>,
  pub profile: Option<String>,
  pub streamer_mode_detection: Option<bool>,
  pub rpc_server: Option<bool>,
  pub open_on_startup: Option<bool>,
  pub startup_minimized: Option<bool>,
  pub autoupdate: Option<bool>,
  pub update_notify: Option<bool>,
  pub desktop_notifications: Option<bool>,
  pub auto_clear_cache: Option<bool>,
  pub multi_instance: Option<bool>,
  pub disable_hardware_accel: Option<bool>,
  pub blur: Option<String>,
  pub blur_css: Option<bool>,
  pub client_mods: Option<Vec<String>>,
  pub unread_badge: Option<bool>,
  pub client_plugins: Option<bool>,
  pub tray_icon_enabled: Option<bool>,

  pub keybinds: Option<HashMap<String, Vec<KeyStruct>>>,
  pub keybinds_enabled: Option<bool>,
}

impl Config {
  pub fn default() -> Self {
    Config {
      // Deprecated
      theme: Option::from("none".to_string()),

      themes: Option::from(vec!["none".to_string()]),
      zoom: Option::from("1.0".to_string()),
      client_type: Option::from("default".to_string()),
      sys_tray: Option::from(false),
      push_to_talk: Option::from(false),
      push_to_talk_keys: Option::from(vec!["RControl".to_string()]),
      cache_css: Option::from(false),
      use_native_titlebar: Option::from(false),
      start_maximized: Option::from(false),
      profile: Option::from("default".to_string()),
      streamer_mode_detection: Option::from(false),
      rpc_server: Option::from(false),
      open_on_startup: Option::from(false),
      startup_minimized: Option::from(false),
      autoupdate: Option::from(false),
      update_notify: Option::from(true),
      desktop_notifications: Option::from(false),
      auto_clear_cache: Option::from(false),
      multi_instance: Option::from(false),
      disable_hardware_accel: Option::from(false),
      blur: Option::from("none".to_string()),
      blur_css: Option::from(true),
      client_mods: Option::from(vec!["Shelter".to_string()]),
      unread_badge: Option::from(true),
      client_plugins: Option::from(true),
      tray_icon_enabled: Option::from(true),

      keybinds: Option::from(HashMap::new()),
      keybinds_enabled: Option::from(true),
    }
  }

  pub fn init() {
    get_config_file();
  }

  pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config_str = config_str.as_str();

    match serde_json::from_str(config_str) {
      Ok(config) => Ok(config),
      Err(e) => {
        log!("Failed to parse config, using default config!");
        log!("Error: {}", e);

        Ok(Self::default())
      }
    }
  }

  pub fn to_file(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, serde_json::to_string(&self)?)?;

    Ok(())
  }

  pub fn from_file_str(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::from_file(path)?;
    let config_str = serde_json::to_string(&config)?;

    Ok(config_str)
  }

  pub fn from_str(contents: impl AsRef<str>) -> Result<Self, Box<dyn std::error::Error>> {
    match serde_json::from_str(contents.as_ref()) {
      Ok(config) => Ok(config),
      Err(e) => {
        log!("Failed to parse config, using default config!");
        log!("Error: {}", e);

        Ok(Self::default())
      }
    }
  }
}

#[tauri::command]
pub fn read_config_file() -> String {
  Config::from_file_str(get_config_file()).expect("Config does not exist!")
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  let config = Config::from_str(&contents).expect("Error parsing config!");
  config
    .to_file(get_config_file())
    .expect("Error writing config!");
}

#[tauri::command]
pub fn default_config() -> Config {
  Config::default()
}

#[tauri::command]
pub fn get_config() -> Config {
  let config_str = read_config_file();

  Config::from_str(&config_str).expect("Error parsing config!")
}

#[tauri::command]
pub fn set_config(config: Config) {
  let config_str = match serde_json::to_string(&config) {
    Ok(config_str) => config_str,
    Err(e) => {
      log!("Failed to serialize config, using default config!");
      log!("Error: {}", e);

      return;
    }
  };

  write_config_file(config_str);
}
