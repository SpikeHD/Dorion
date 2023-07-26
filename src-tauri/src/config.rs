use serde::{Deserialize, Serialize};
use std::fs;

use crate::paths::get_config_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub theme: Option<String>,
  pub zoom: Option<String>,
  pub client_type: Option<String>,
  pub sys_tray: Option<bool>,
  pub block_telemetry: Option<bool>,
  pub push_to_talk: Option<bool>,
  pub push_to_talk_keys: Option<Vec<String>>,
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

pub fn default_config() -> Config {
  Config {
    theme: Option::from("none".to_string()),
    zoom: Option::from("1.0".to_string()),
    client_type: Option::from("default".to_string()),
    sys_tray: Option::from(false),
    block_telemetry: Option::from(false),
    push_to_talk: Option::from(false),
    push_to_talk_keys: Option::from(vec!["RControl".to_string()]),
  }
}

pub fn get_zoom() -> f64 {
  init();

  let parsed: Config =
    serde_json::from_str(read_config_file().as_str()).unwrap_or_else(|_| default_config());

  parsed
    .zoom
    .unwrap_or_else(|| "1.0".to_string())
    .parse()
    .unwrap_or(1.0)
}

pub fn get_client_type() -> String {
  let parsed: Config =
    serde_json::from_str(read_config_file().as_str()).unwrap_or_else(|_| default_config());
  parsed
    .client_type
    .unwrap_or_else(|| "default".to_string())
    .parse()
    .unwrap_or_else(|_| "default".to_string())
}

pub fn get_systray() -> bool {
  let parsed: Config =
    serde_json::from_str(read_config_file().as_str()).unwrap_or_else(|_| default_config());
  parsed.sys_tray.unwrap_or(false)
}

pub fn get_ptt() -> bool {
  let parsed: Config =
    serde_json::from_str(read_config_file().as_str()).unwrap_or_else(|_| default_config());
  parsed.push_to_talk.unwrap_or(false)
}

pub fn get_ptt_keys() -> Vec<String> {
  let parsed: Config =
    serde_json::from_str(read_config_file().as_str()).unwrap_or_else(|_| default_config());
  parsed
    .push_to_talk_keys
    .unwrap_or_else(|| vec!["RControl".to_string()])
}
