use std::{fs, path::PathBuf};

pub fn get_config_dir() -> PathBuf {
  // First check for a local config file
  let local_config_dir = std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .join("config.json");

  if fs::metadata(&local_config_dir).is_ok() {
    return local_config_dir;
  }

  println!("No local config file found. Using default.");

  let appdata = tauri::api::path::data_dir().unwrap();
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

pub fn get_plugin_dir() -> std::path::PathBuf {
  // First check for a local plugin dir
  let local_plugin_dir = std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .join("plugins");

  if fs::metadata(&local_plugin_dir).is_ok() {
    return local_plugin_dir;
  }

  println!("No local plugin dir found. Using default.");

  let plugin_dir = tauri::api::path::home_dir()
    .unwrap()
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
  let local_theme_dir = std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .join("themes");

  if fs::metadata(&local_theme_dir).is_ok() {
    return local_theme_dir;
  }

  println!("No local theme dir found. Using default.");

  let theme_dir = tauri::api::path::home_dir()
    .unwrap()
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
