use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
  theme: String,
  zoom: String,
  client_type: String,
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
      r#"{ "theme": "none", "zoom": "1.0", "client_type": "default" }"#,
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
    theme: "none".to_string(),
    zoom: "1.0".to_string(),
    client_type: "default".to_string(),
  }
}

pub fn get_zoom() -> f64 {
  init();

  let parsed: Config =
    serde_json::from_str(read_config_file().as_str()).unwrap_or_else(|_| default_config());

  parsed.zoom.parse().unwrap_or(1.0)
}

pub fn get_client_type() -> String {
  let parsed: Config =
    serde_json::from_str(read_config_file().as_str()).unwrap_or_else(|_| default_config());
  parsed
    .client_type
    .parse()
    .unwrap_or_else(|_| "default".to_string())
}
