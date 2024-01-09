use serde::{Deserialize, Serialize};
use std::fs;

use crate::util::logger::log;
use crate::util::paths::{get_config_dir, get_client_mod_dir};

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
  pub client_mods: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientMod {
  pub name: String,
  pub script: String,
  pub styles: String,
  pub enabled: bool,
  pub fallback: String,
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
pub fn read_client_mods_file() -> String {
  let config_file = get_client_mod_dir();

  fs::read_to_string(config_file).expect("Client_mods does not exist!")
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  init();

  let config_file = get_config_dir();

  fs::write(config_file, contents).expect("Error writing client mods!")
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn write_client_mods_file(contents: String) {
  let config_file = get_client_mod_dir();

  fs::write(config_file, contents).expect("Error writing client_mods!")
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
    client_mods: Option::from(vec!["Shelter".to_string()]),
  }
}

#[tauri::command]
pub fn default_client_mods() -> Vec<ClientMod> {
  vec![
    ClientMod {
      name: "Shelter".to_string(),
      script: "https://raw.githubusercontent.com/uwu/shelter-builds/main/shelter.js".to_string(),
      styles: "".to_string(),
      enabled: true,
      fallback: "./injection/shelter.js".to_string(), 
    },
    ClientMod {
      name: "Vencord".to_string(),
      script: "https://github.com/Vendicated/Vencord/releases/download/devbuild/browser.js".to_string(),
      styles: "https://github.com/Vendicated/Vencord/releases/download/devbuild/browser.css".to_string(),
      enabled: true,
      fallback: "".to_string(),
    },
  ]
}

pub fn get_config() -> Config {
  let config_str = read_config_file();
  let config_str = config_str.as_str();

  match serde_json::from_str(config_str) {
    Ok(config) => config,
    Err(e) => {
      log("Failed to parse config, using default config!");
      log(format!("Error: {}", e));

      default_config()
    }
  }
}

pub fn get_client_mods_config() -> Vec<ClientMod> {
  let config_str = read_client_mods_file();
  let config_str = config_str.as_str();

  match serde_json::from_str(config_str) {
    Ok(config) => config,
    Err(e) => {
      log("Failed to parse config, using default config!");
      log(format!("Error: {}", e));

      default_client_mods()
    }
  }
}