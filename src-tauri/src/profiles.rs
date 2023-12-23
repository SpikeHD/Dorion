use std::path::PathBuf;

use crate::{
  config::{get_config, Config},
  util::logger::log,
  util::paths::profiles_dir,
};

pub fn init_profiles_folders() {
  // Create %appdata%/dorion/profiles/default
  let default_profile_folder = profiles_dir().join("default");

  if !default_profile_folder.exists() {
    std::fs::create_dir_all(default_profile_folder).expect("Failed to create profile folder!");
  }
}

#[tauri::command]
pub fn get_profile_list() -> Vec<String> {
  let mut profiles: Vec<String> = vec![];

  let profiles_folder = profiles_dir();

  if profiles_folder.exists() {
    let paths = std::fs::read_dir(profiles_folder).expect("Unable to read profiles folder!");

    for path in paths {
      if path.is_err() {
        continue;
      }

      let path = path.unwrap().path();

      if path.is_dir() {
        if let Some(file_name) = path.file_name() {
          if let Some(profile_name) = file_name.to_str() {
            profiles.push(profile_name.to_string());
          } else {
            log("Failed to convert file name to a valid UTF-8 string".to_string());
          }
        } else {
          log("Failed to retrieve file name".to_string());
        }
      } else {
        log("Path is not a directory".to_string());
      }
    }
  }

  profiles
}

#[tauri::command]
pub fn get_current_profile_folder() -> PathBuf {
  let profiles_folder = profiles_dir();
  let current_profile = get_config().profile.unwrap_or("default".to_string());

  let profile_folder = profiles_folder.join(current_profile);

  // If it doesn't exist, just use the default path
  if !profile_folder.exists() {
    return profiles_folder.join("default");
  }

  profile_folder
}

#[tauri::command]
pub fn create_profile(name: String) {
  let profiles_folder = profiles_dir();

  let new_profile_folder = profiles_folder.join(name);

  if !new_profile_folder.exists() {
    std::fs::create_dir_all(new_profile_folder).unwrap_or_else(|_| {
      log("Failed to create profile folder!".to_string());
    });
  }
}

#[tauri::command]
pub fn delete_profile(name: String) {
  if name == "default" {
    return;
  }

  let profiles_folder = profiles_dir();

  let profile_folder = profiles_folder.join(name);

  if profile_folder.exists() {
    std::fs::remove_dir_all(profile_folder).unwrap_or_else(|_| {
      log("Failed to delete profile folder!".to_string());
    });
  }

  // Set config to "default"
  let mut config: Config =
    serde_json::from_str(&crate::config::read_config_file()).expect("Failed to read config file!");

  config.profile = Some("default".to_string());

  crate::config::write_config_file(
    serde_json::to_string(&config).expect("Failed to convert config to string!"),
  );
}
