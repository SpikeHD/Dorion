use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
  theme: Option<String>,
  zoom: Option<String>,
  client_type: Option<String>,
  sys_tray: Option<bool>,
  block_telemetry: Option<bool>,
  push_to_talk: Option<bool>,
  push_to_talk_keys: Option<Vec<String>>,
}

pub fn init() {
  let appdata = tauri::api::path::data_dir().unwrap();
  let config_file = appdata.join("dorion").join("config.json");

  if fs::metadata(appdata.join("dorion")).is_err() {
    fs::create_dir_all(appdata.join("dorion")).expect("Error creating appdata dir");
  }

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      config_file,
      r#"{ "theme": "none", "zoom": "1.0", "client_type": "default", "sys_tray": false, "block_telemetry": false }"#,
    )
    .unwrap_or(());
  }
}

#[tauri::command]
pub fn read_config_file() -> String {
  init();

  let appdata = tauri::api::path::data_dir().unwrap();
  let config_file = appdata.join("dorion").join("config.json");

  fs::read_to_string(config_file).expect("Config does not exist!")
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  init();

  let appdata = tauri::api::path::data_dir().unwrap();
  let config_file = appdata.join("dorion").join("config.json");

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
  parsed.push_to_talk_keys.unwrap_or(vec!["RControl".to_string()])
}