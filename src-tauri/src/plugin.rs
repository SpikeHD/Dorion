use serde::{Deserialize, Serialize};
use std::fs;

use crate::js_preprocess::get_js_imports;

#[derive(Serialize, Deserialize)]
pub struct Plugin {
  name: String,
  disabled: bool,
}

fn get_plugin_dir() -> std::path::PathBuf {
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

#[tauri::command]
pub async fn load_plugins() -> String {
  let mut contents = String::new();
  let plugins_dir = get_plugin_dir();
  let plugin_folders = match fs::read_dir(&plugins_dir) {
    Ok(f) => f,
    Err(e) => {
      println!("Error: {}", e);

      return String::new();
    }
  };

  for path in plugin_folders {
    if path.is_err() {
      continue;
    }

    let folder = path.unwrap().file_name().clone();
    let plugin_dir = plugins_dir.join(&folder);
    let index_file = plugin_dir.join("index.js");

    if folder.to_str().unwrap_or("").starts_with('_') {
      continue;
    }

    if fs::metadata(&index_file).is_ok() {
      let plugin_contents = fs::read_to_string(&index_file).unwrap();

      contents = format!("{};(() => {{ {} }})()", contents, plugin_contents);
    }
  }

  contents
}

#[tauri::command]
pub async fn get_plugin_import_urls(plugin_js: String) -> Vec<String> {
  let mut script_imports: Vec<String> = vec![];
  let url_imports = get_js_imports(&plugin_js).await;

  for s in &url_imports {
    script_imports.push(s.to_string());
  }

  script_imports
}

#[tauri::command]
pub fn get_plugin_list() -> Vec<Plugin> {
  let plugins_dir = get_plugin_dir();
  let mut plugin_list: Vec<Plugin> = Vec::new();
  let plugin_folders = match fs::read_dir(&plugins_dir) {
    Ok(f) => f,
    Err(e) => {
      println!("Error: {}", e);

      return plugin_list;
    }
  };

  for path in plugin_folders {
    if path.is_err() {
      continue;
    }

    let folder = path.unwrap().file_name().clone();
    let plugin_dir = plugins_dir.join(&folder);
    let index_file = plugin_dir.join("index.js");
    let disabled = folder.to_str().unwrap_or("").starts_with('_');

    let mut plugin_name = folder.into_string().unwrap();

    if plugin_name.starts_with('_') {
      plugin_name = plugin_name.replacen('_', "", 1);
    }

    if fs::metadata(&index_file).is_ok() {
      plugin_list.push(Plugin {
        name: plugin_name,
        disabled,
      });
    }
  }

  plugin_list
}

#[tauri::command]
pub fn toggle_plugin(name: String) {
  let plugins_dir = get_plugin_dir();
  let folders = fs::read_dir(&plugins_dir).unwrap();

  for path in folders {
    if path.is_err() {
      continue;
    }

    let folder = path.unwrap().file_name().clone();
    let folder_name = folder.to_str().unwrap();
    let mut plugin_name = String::from(&name);

    // Use this name to ensure that, if a name with _ is provided, we remove that before comparison
    if plugin_name.starts_with('_') {
      plugin_name = folder_name.replacen('_', "", 1);
    }

    if folder_name.contains(&plugin_name) {
      let mut new_name = String::from('_') + folder_name;

      if folder_name.starts_with('_') {
        new_name = folder_name.replacen('_', "", 1);
      }

      // Disable the folder
      fs::rename(plugins_dir.join(folder_name), plugins_dir.join(new_name)).unwrap();
    }
  }
}
