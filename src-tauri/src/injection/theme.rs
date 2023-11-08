use std::fs;

use crate::util::paths::get_theme_dir;

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
pub fn _theme_from_link(link: String) -> Result<(), String> {
  let mut theme_name = link.split('/').last().unwrap().to_string();

  if theme_name.is_empty() {
    return Ok(());
  }

  if !theme_name.ends_with(".css") {
    theme_name.push_str(".css");
  }

  let theme = reqwest::blocking::get(&link)
    .map_err(|e| format!("Error fetching theme from link: {}", e))?
    .text()
    .map_err(|e| format!("Error reading theme from response: {}", e))?;

  let path = get_theme_dir().join(&theme_name);

  fs::write(path, theme).map_err(|e| format!("Error writing theme to file: {}", e))?;

  Ok(())
}
