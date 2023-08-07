use std::fs;

use crate::paths::get_theme_dir;

#[tauri::command]
pub fn get_theme(name: String) -> String {
  let theme_file = get_theme_dir().join(name);

  if !theme_file.is_dir() {
    return fs::read_to_string(theme_file).unwrap_or_else(|_| "".to_string());
  }

  // Find the first CSS file in the dir
  let mut css_file = String::new();

  for file in fs::read_dir(&theme_file).unwrap() {
    let filename = file.unwrap().file_name();
    let name_string = filename.clone().to_str().unwrap().to_string();

    if name_string.ends_with(".css") {
      css_file = name_string;
      break;
    }
  }

  fs::read_to_string(theme_file.join(css_file)).unwrap_or_else(|_| "".to_string())
}

#[tauri::command]
pub fn get_theme_names() -> Vec<String> {
  let themes_dir = get_theme_dir();
  let theme_folders = fs::read_dir(themes_dir).unwrap();
  let mut names = vec![] as Vec<String>;

  for path in theme_folders {
    if let Err(_path) = path {
      continue;
    }

    let folder = path.unwrap().file_name().clone();
    names.push(format!("{:?}", folder.clone()));
  }

  names
}

#[tauri::command]
pub fn theme_from_link(link: String) {
  let mut theme_name = link.split("/").last().unwrap().to_string();
  
  if theme_name.is_empty() {
    return;
  }

  if !theme_name.ends_with(".css") {
    theme_name.push_str(".css");
  }

  let theme = reqwest::blocking::get(&link).unwrap().text().unwrap();
  
  let path = get_theme_dir().join(theme_name);

  println!("Writing to: {:?}", path);

  fs::write(path, theme).unwrap();
}