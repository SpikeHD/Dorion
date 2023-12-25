use serde::{Deserialize, Serialize};
use std::fs;

use crate::util::logger::log;
use crate::util::paths::get_config_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub theme: Option<String>,
  pub zoom: Option<String>,
  pub client_type: Option<String>,
  pub sys_tray: Option<bool>,
  pub block_telemetry: Option<bool>,
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
}

pub fn init() {
  get_config_dir();
}

#[tauri::command]
pub fn read_config_file() -> String {
  init();

  let config_file = get_config_dir();

  fs::read_to_string(config_file).expect("Config does not exist!")
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  init();

  let config_file = get_config_dir();

  fs::write(config_file, contents).expect("Error writing config!")
}

#[tauri::command]
pub fn default_config() -> Config {
  Config {
    theme: Option::from("none".to_string()),
    zoom: Option::from("1.0".to_string()),
    client_type: Option::from("default".to_string()),
    sys_tray: Option::from(false),
    block_telemetry: Option::from(false),
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
  }
}

pub fn get_config() -> Config {
  let config_str = read_config_file();
  let config_str = config_str.as_str();

  match serde_json::from_str(config_str) {
    Ok(config) => config,
    Err(e) => {
      log("Failed to parse config, using default config!".to_string());
      log(format!("Error: {}", e));

      default_config()
    }
  }
}
