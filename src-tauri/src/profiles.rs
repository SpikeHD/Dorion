use std::path::PathBuf;

use crate::{
  config::{get_config, Config},
  util::paths::profiles_dir,
};

pub fn init_profiles_folders() {
  // Create %appdata%/dorion/profiles/default
  let default_profile_folder = profiles_dir().join("default");

  if !default_profile_folder.exists() {
    std::fs::create_dir_all(default_profile_folder).unwrap();
  }
}

#[tauri::command]
pub fn get_profile_list() -> Vec<String> {
  let mut profiles: Vec<String> = vec![];

  let profiles_folder = profiles_dir();

  if profiles_folder.exists() {
    let paths = std::fs::read_dir(profiles_folder).unwrap();

    for path in paths {
      let path = path.unwrap().path();

      if path.is_dir() {
        let profile_name = path.file_name().unwrap().to_str().unwrap().to_string();

        profiles.push(profile_name);
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
    std::fs::create_dir_all(new_profile_folder).unwrap();
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
    std::fs::remove_dir_all(profile_folder).unwrap();
  }

  // Set config to "default"
  let mut config: Config = serde_json::from_str(&crate::config::read_config_file()).unwrap();

  config.profile = Some("default".to_string());

  crate::config::write_config_file(serde_json::to_string(&config).unwrap());
}
