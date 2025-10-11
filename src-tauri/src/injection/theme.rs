use std::fs;

use crate::{config::get_config, util::paths::get_theme_dir};

fn theme_is_enabled(name: String) -> bool {
  let config = get_config();
  config.themes.unwrap_or_default().contains(&name)
}

#[tauri::command]
pub fn get_themes() -> Result<String, String> {
  let themes = get_theme_dir();
  let mut all_contents = String::new();

  for entry in fs::read_dir(themes).map_err(|e| format!("Error reading theme directory: {e}"))? {
    let entry = entry.map_err(|e| format!("Error reading theme directory: {e}"))?;
    let file_name = entry
      .file_name()
      .to_str()
      .map(|name| name.to_string())
      .filter(|name| name != "cache" && name != ".ds_store")
      .unwrap_or_default();

    if file_name.ends_with(".css") && theme_is_enabled(file_name) {
      all_contents.push_str(
        fs::read_to_string(entry.path())
          .unwrap_or_default()
          .as_str(),
      );
    }
  }

  Ok(all_contents)
}

#[tauri::command]
pub fn get_theme_names() -> Result<Vec<String>, String> {
  let themes_dir = get_theme_dir();
  let theme_folders =
    fs::read_dir(themes_dir).map_err(|e| format!("Error reading theme directory: {e}"))?;
  let names = theme_folders
    .filter_map(|entry| {
      entry.ok().and_then(|file| {
        file
          .file_name()
          .to_str()
          .map(|name| name.to_string())
          .filter(|name| {
            let lowercase = name.to_lowercase();
            lowercase != "cache" && lowercase != ".ds_store"
          })
      })
    })
    .map(|folder_name| format!("{folder_name:?}"))
    .collect();

  Ok(names)
}

#[tauri::command]
pub fn get_enabled_themes() -> Result<Vec<String>, String> {
  let config = get_config();
  Ok(config.themes.unwrap_or_default())
}

#[tauri::command]
pub fn theme_from_link(link: String, filename: Option<String>) -> String {
  let theme_name = filename
    .unwrap_or(link.split('/').next_back().unwrap_or("unnamed").to_string())
    .split('.')
    .next()
    .unwrap_or("unnamed")
    .to_string();

  let mut filename = theme_name.clone();

  if theme_name.is_empty() {
    return String::new();
  }

  if !filename.ends_with(".css") {
    filename.push_str(".css");
  }

  let resp = reqwest::blocking::get(&link);

  if resp.is_err() {
    return String::new();
  }

  let theme = resp.unwrap().text().unwrap_or_default();

  let path = get_theme_dir().join(&filename);

  if fs::write(path, theme).is_err() {
    return String::new();
  }

  filename
}
