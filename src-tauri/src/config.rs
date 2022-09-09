use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Config {
  theme: String,
  zoom: String,
  client_type: String,
}

pub fn init() {
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let config_file = exe_dir.join("config.json");

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      config_file,
      r#"{ "theme": "none", "zoom": "1.0", "client_type": "default" }"#,
    )
    .unwrap();
  }
}

#[tauri::command]
pub fn read_config_file() -> String {
  init();

  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let config_file = exe_dir.join("config.json");

  fs::read_to_string(config_file).unwrap()
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  init();

  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let config_file = exe_dir.join("config.json");

  fs::write(config_file, contents).unwrap()
}

pub fn get_zoom() -> f64 {
  init();

  let parsed: Config = serde_json::from_str(read_config_file().as_str()).unwrap_or(Config {
    theme: "none".to_string(),
    zoom: "1.0".to_string(),
    client_type: "default".to_string()
  });

  parsed.zoom.parse().unwrap_or(1.0)
}