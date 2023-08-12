use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use tauri::regex::Regex;

use crate::util::paths::get_plugin_dir;

#[derive(Serialize, Deserialize)]
pub struct Plugin {
  name: String,
  disabled: bool,
  preload: bool,
}

#[tauri::command]
pub fn get_js_imports(js: &str) -> Vec<String> {
  let reg = Regex::new(r"//[ ]?URL_IMPORT (.*)").unwrap();
  let mut imports: Vec<String> = vec![];

  let captures = reg.captures_iter(js).next();

  if captures.is_none() {
    return imports;
  }

  if let Some(capture) = captures.unwrap().get(1) {
    let first_match = capture.as_str();

    imports.push(first_match.to_string());
  }

  imports
}

#[tauri::command]
pub fn load_plugins(preload_only: Option<bool>) -> HashMap<String, String> {
  let pl_only = preload_only.unwrap_or(false);
  let mut plugin_list = HashMap::new();
  let plugins_dir = get_plugin_dir();
  let plugin_folders = match fs::read_dir(plugins_dir) {
    Ok(f) => f,
    Err(e) => {
      println!("Error: {}", e);

      return HashMap::new();
    }
  };

  for path in plugin_folders {
    let full_path = path.unwrap();
    let meta = full_path.metadata().unwrap();
    let name = full_path.file_name();

    // If it's just the file, load just the file contens
    if meta.is_dir() {
      // No more folder plugins. Ew yuck gross.
      continue;
    }

    // Disabled
    if name.to_str().unwrap().starts_with('_') {
      continue;
    }

    // Preload-only
    if !name.to_str().unwrap().starts_with("PRELOAD_") && pl_only {
      continue;
    }

    let contents = fs::read_to_string(full_path.path()).unwrap();
    plugin_list.insert(format!("{:?}", name), contents);
  }

  plugin_list
}

#[tauri::command]
pub fn get_plugin_import_urls(plugin_js: String) -> Vec<String> {
  let mut script_imports: Vec<String> = vec![];
  let url_imports = get_js_imports(&plugin_js);

  for s in &url_imports {
    script_imports.push(s.to_string());
  }

  script_imports
}

#[tauri::command]
pub fn get_plugin_list() -> Vec<Plugin> {
  let plugins_dir = get_plugin_dir();
  let mut plugin_list: Vec<Plugin> = Vec::new();
  let plugin_folders = match fs::read_dir(plugins_dir) {
    Ok(f) => f,
    Err(e) => {
      println!("Error: {}", e);

      return plugin_list;
    }
  };

  for path in plugin_folders {
    let full_path = path.unwrap();
    let meta = full_path.metadata().unwrap();
    let name = full_path.file_name();

    // If it's just the file, load just the file contens
    if meta.is_dir() {
      // No more plugin folders grr
      continue;
    }

    let disabled = name.to_str().unwrap_or("").starts_with('_');
    let preload = name.to_str().unwrap_or("").contains("PRELOAD");

    let mut plugin_name = name.to_str().unwrap_or("").to_string();

    if disabled {
      plugin_name = plugin_name.replacen('_', "", 1);
    }

    if preload {
      plugin_name = plugin_name.replace("PRELOAD_", "");
    }

    plugin_name = plugin_name.replace(".js", "");

    if fs::metadata(full_path.path()).is_ok() {
      plugin_list.push(Plugin {
        name: plugin_name,
        disabled,
        preload,
      });
    }
  }

  plugin_list
}

#[tauri::command]
pub fn toggle_plugin(name: String) -> bool {
  let plugins_dir = get_plugin_dir();
  let folders = fs::read_dir(&plugins_dir).unwrap();

  for path in folders {
    let full_path = path.unwrap();
    let meta = full_path.metadata().unwrap();
    let file_name_os = full_path.file_name();
    let file_name = file_name_os.to_str().unwrap();

    // If it's just the file, load just the file contens
    if meta.is_dir() {
      // No more plugin folders grr
      continue;
    }

    let mut plugin_name = String::from(&name);

    // Use this name to ensure that, if a name with _ is provided, we remove that before comparison
    if plugin_name.starts_with('_') {
      plugin_name = file_name.replacen('_', "", 1);
    }

    if file_name.contains(&plugin_name) {
      let mut new_name = String::from('_') + file_name;

      if file_name.starts_with('_') {
        new_name = file_name.replacen('_', "", 1);
      }

      // Disable the folder
      fs::rename(plugins_dir.join(file_name), plugins_dir.join(new_name)).unwrap();

      return file_name.starts_with('_');
    }
  }

  false
}

#[tauri::command]
pub fn toggle_preload(name: String) -> bool {
  let plugins_dir = get_plugin_dir();
  let folders = fs::read_dir(&plugins_dir).unwrap();

  for path in folders {
    let full_path = path.unwrap();
    let meta = full_path.metadata().unwrap();
    let file_name_os = full_path.file_name();
    let file_name = file_name_os.to_str().unwrap();

    // If it's just the file, load just the file contens
    if meta.is_dir() {
      // No more plugin folders grr
      continue;
    }

    let mut plugin_name = String::from(&name);
    let disabled = file_name.starts_with('_');
    let preloaded = file_name.contains("PRELOAD");

    // Use this name to ensure that, if a name with PRELOAD is provided, we remove that before comparison
    if plugin_name.contains("PRELOAD") {
      plugin_name = file_name.replace("PRELOAD_", "").replacen('_', "", 1);
    }

    if file_name.contains(&plugin_name) {
      let mut new_name = plugin_name;

      // Disable if enabled, otherwise enable if disabled
      if preloaded {
        new_name = new_name.replace("PRELOAD_", "");
      } else {
        new_name = String::from("PRELOAD_") + &new_name;
      }

      // Ensure we keep disabled state
      if disabled {
        new_name = String::from("_") + &new_name;
      }

      new_name += ".js";

      // Disable/enable preload
      fs::rename(plugins_dir.join(file_name), plugins_dir.join(new_name)).unwrap();

      return !preloaded;
    }
  }

  false
}
