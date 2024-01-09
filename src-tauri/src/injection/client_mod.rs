use lazy_static::lazy_static;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
  config::{get_config, write_config_file},
  util::logger::log,
};

pub struct ClientMod {
  pub script: String,
  pub styles: String,
}

lazy_static! {
  pub static ref CLIENT_MODS: HashMap<String, ClientMod> = {
    let mut map = HashMap::new();

    map.insert(
      "Shelter".to_string(),
      ClientMod {
        script: "https://raw.githubusercontent.com/uwu/shelter-builds/main/shelter.js".to_string(),
        styles: "".to_string(),
      },
    );

    map.insert(
      "Vencord".to_string(),
      ClientMod {
        script: "https://github.com/Vendicated/Vencord/releases/download/devbuild/browser.js"
          .to_string(),
        styles: "https://github.com/Vendicated/Vencord/releases/download/devbuild/browser.css"
          .to_string(),
      },
    );

    map
  };
}

#[tauri::command]
pub fn available_mods() -> Vec<String> {
  CLIENT_MODS.keys().map(|s| s.to_string()).collect()
}

pub fn load_mods_js() -> String {
  let config = get_config();
  let mut enabled_mods = config.client_mods.unwrap_or(vec![]);

  // if enabled_mods does not include shelter, add it and save the config
  if !enabled_mods.contains(&"Shelter".to_string()) {
    let mut config = get_config();
    // add shelter to the enabled mods while keeping the others. shelter is always first
    enabled_mods.insert(0, "Shelter".to_string());
    config.client_mods = Option::from(enabled_mods.clone());

    write_config_file(serde_json::to_string(&config).unwrap());
  }

  let mut exec = String::new();

  for mod_name in enabled_mods {
    let response =
      reqwest::blocking::get(CLIENT_MODS.get(mod_name.as_str()).unwrap().script.as_str()).unwrap();

    let contents = if !response.status().is_success() {
      log(format!(
        "Failed to load client mod {}! Loading fallback.",
        mod_name
      ));

      read_fallback(mod_name.clone())
    } else {
      response.text().unwrap()
    };

    exec = format!("{};{}", exec, contents);

    write_fallback(mod_name, contents);
  }

  exec
}

#[tauri::command]
pub fn load_mods_css() -> String {
  let config = get_config();
  let enabled_mods = config.client_mods.unwrap_or(vec![]);

  let mut exec = String::new();

  for mod_name in enabled_mods {
    let styles = CLIENT_MODS.get(mod_name.as_str()).unwrap().styles.clone();

    if styles.is_empty() {
      continue;
    }

    let response = reqwest::blocking::get(styles).unwrap();

    let contents = if !response.status().is_success() {
      log(format!(
        "Failed to load client mod {}! Loading fallback.",
        mod_name
      ));

      String::new()
    } else {
      response.text().unwrap()
    };

    exec = format!("{} {}", exec, contents);
  }

  exec
}

fn get_fallback_dir(mod_name: String) -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();
  current_exe
    .parent()
    .unwrap()
    .join("injection")
    .join(format!("{}.js", mod_name))
}

fn write_fallback(mod_name: String, contents: String) {
  fs::write(get_fallback_dir(mod_name), contents).unwrap();
}

fn read_fallback(mod_name: String) -> String {
  fs::read_to_string(get_fallback_dir(mod_name)).unwrap()
}
