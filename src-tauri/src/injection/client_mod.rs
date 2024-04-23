use include_flate::flate;
use phf::phf_map;

use crate::{
  config::{get_config, write_config_file},
  log,
};

flate!(pub static FALLBACK: str from "./injection/shelter.js");

pub struct ClientMod {
  script: &'static str,
  styles: &'static str,
}

pub static CLIENT_MODS: phf::Map<&'static str, ClientMod> = phf_map! {
  "Shelter" => ClientMod {
      script: "https://raw.githubusercontent.com/uwu/shelter-builds/main/shelter.js",
      styles: "",
  },
  "Vencord" => ClientMod {
      script: "https://github.com/Vendicated/Vencord/releases/download/devbuild/browser.js",
      styles: "https://github.com/Vendicated/Vencord/releases/download/devbuild/browser.css",
  },
  "Equicord" => ClientMod {
      script: "https://github.com/Equicord/Equicord/releases/download/latest/browser.js",
      styles: "https://github.com/Equicord/Equicord/releases/download/latest/browser.css",
  },
};

#[tauri::command]
pub fn available_mods() -> Vec<String> {
  CLIENT_MODS.keys().map(|s| s.to_string()).collect()
}

pub fn load_mods_js() -> String {
  let config = get_config();
  let mut enabled_mods = config.client_mods.unwrap_or_default();

  // if enabled_mods does not include shelter, add it and save the config
  if !enabled_mods.contains(&"Shelter".to_string()) {
    log!("Shelter not detected as client mod: adding to config!");
    let mut config = get_config();
    // add shelter to the enabled mods while keeping the others. shelter is always first
    enabled_mods.insert(0, "Shelter".to_string());
    config.client_mods = Option::from(enabled_mods.clone());

    write_config_file(serde_json::to_string(&config).unwrap());
  }

  let mut exec = String::new();
  let mut tasks = Vec::new();

  for mod_name in enabled_mods {
    let script_url = CLIENT_MODS.get(mod_name.as_str()).unwrap_or(
      // Prevent panics
      &ClientMod {
        script: "",
        styles: "",
      },
    ).script;

    tasks.push(std::thread::spawn(move || {
      let response = match reqwest::blocking::get(script_url) {
        Ok(r) => r,
        Err(_) => {
          log!("Failed to load client mod JS for {}.", mod_name);

          if mod_name == "Shelter" {
            log!("Shelter detected: loading fallback!");
            return FALLBACK.clone();
          }

          return String::new();
        }
      };

      let status = response.status();

      if status != 200 {
        log!("Failed to load client mod JS for {}.", mod_name);

        if mod_name == "Shelter" {
          log!("Shelter detected: loading fallback!");
          return FALLBACK.clone();
        }

        return String::new();
      }

      response.text().expect("Failed to parse client mod JS!")
    }));
  }

  for task in tasks {
    let result = match task.join() {
      Ok(r) => r,
      Err(e) => {
        log!("Error joining thread: {:?}", e);
        continue;
      }
    };

    log!("Joining (load_mods_js)...");

    if result.is_empty() {
      continue;
    }

    exec = format!("{};{}", exec, result);
  }

  exec
}

#[tauri::command]
pub fn load_mods_css() -> String {
  let config = get_config();
  let enabled_mods = config.client_mods.unwrap_or_default();
  let mut exec = String::new();

  let mut tasks = Vec::new();

  for mod_name in enabled_mods {
    let styles_url = CLIENT_MODS.get(mod_name.as_str()).unwrap_or(
      // Prevent panics
      &ClientMod {
        script: "",
        styles: "",
      },
    ).styles;

    if styles_url.is_empty() {
      continue;
    }

    tasks.push(std::thread::spawn(move || {
      let response = match reqwest::blocking::get(styles_url) {
        Ok(r) => r,
        Err(_) => {
          log!("Failed to load client mod CSS for {}.", mod_name);
          return String::new();
        }
      };

      let status = response.status();

      if status != 200 {
        log!("Failed to load client mod CSS for {}.", mod_name);
        return String::new();
      }

      response.text().expect("Failed to parse client mod CSS!")
    }));
  }

  for task in tasks {
    let result = match task.join() {
      Ok(r) => r,
      Err(e) => {
        log!("Error joining thread: {:?}", e);
        continue;
      }
    };

    log!("Joining (load_mods_css)...");

    if result.is_empty() {
      continue;
    }

    exec = format!("{} {}", exec, result);
  }

  exec
}
