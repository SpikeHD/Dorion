use std::fs;

use crate::{util::paths::get_theme_dir, config::get_client_mods_config};

#[tauri::command]
pub fn get_theme(name: String) -> Result<String, String> {
  let theme_file = get_theme_dir().join(name);

  if !theme_file.is_dir() {
    return fs::read_to_string(&theme_file).map_err(|e| format!("Error reading theme file: {}", e));
  }

  // Find the first CSS file in the directory
  let css_file = fs::read_dir(&theme_file)
    .map_err(|e| format!("Error reading theme directory: {}", e))?
    .find_map(|entry| {
      entry
        .ok()
        .and_then(|file| file.file_name().to_str().map(|name| name.to_string()))
        .filter(|name| name.ends_with(".css"))
    });

  if let Some(css_file) = css_file {
    return fs::read_to_string(theme_file.join(css_file))
      .map_err(|e| format!("Error reading CSS file: {}", e));
  }

  Ok("".to_string())
}

#[tauri::command]
pub fn get_client_mod_themes() -> String {
  let client_mods = get_client_mods_config()
    .into_iter()
    .filter(|x| x.enabled)
    .collect::<Vec<_>>();

  let mut client_mods_css = String::new();

  for client_mod in client_mods {
    if client_mod.styles.is_empty() {
      continue;
    }

    let req = reqwest::blocking::get(client_mod.styles.as_str());

    let resp = match req {
      Ok(r) => r,
      Err(e) => {
        println!(
          "Failed to read {} in resource dir, using fallback: {}",
          client_mod.name, e
        );
        // Send nothing instead
        return String::new();
      }
    };

    client_mods_css += resp.text().unwrap_or_default().as_str();
  }

  client_mods_css
}

#[tauri::command]
pub fn get_theme_names() -> Result<Vec<String>, String> {
  let themes_dir = get_theme_dir();
  let theme_folders =
    fs::read_dir(themes_dir).map_err(|e| format!("Error reading theme directory: {}", e))?;
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
    .map(|folder_name| format!("{:?}", folder_name))
    .collect();

  Ok(names)
}

#[tauri::command]
pub fn theme_from_link(link: String) -> String {
  let theme_name = link.split('/').last().unwrap().to_string();
  let mut file_name = theme_name.clone();
  let theme_name = theme_name.split('.').next().unwrap().to_string();

  if theme_name.is_empty() {
    return String::new();
  }

  if !file_name.ends_with(".css") {
    file_name.push_str(".css");
  }

  let resp = reqwest::blocking::get(&link);

  if resp.is_err() {
    return String::new();
  }

  let theme = resp.unwrap().text().unwrap_or_default();

  let path = get_theme_dir().join(&file_name);

  if fs::write(path, theme).is_err() {
    return String::new();
  }

  file_name
}
