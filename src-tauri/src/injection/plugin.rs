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

  let captures = match captures {
    Some(c) => c,
    None => return imports,
  };

  if let Some(capture) = captures.get(1) {
    let first_match = capture.as_str();

    imports.push(first_match.to_string());
  }

  imports
}

#[tauri::command]
pub fn load_plugins(preload_only: Option<bool>) -> Result<HashMap<String, String>, String> {
  let pl_only = preload_only.unwrap_or(false);
  let mut plugin_list = HashMap::new();
  let plugins_dir = get_plugin_dir();

  let plugin_folders =
    fs::read_dir(plugins_dir).map_err(|e| format!("Error reading directory: {}", e))?;

  for entry in plugin_folders {
    let full_path = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
    let meta = full_path
      .metadata()
      .map_err(|e| format!("Error getting metadata: {}", e))?;

    let name = full_path.file_name();
    let name_str = name.to_str().unwrap_or_default();

    if !meta.is_dir() && name_str.ends_with(".js") && !name_str.starts_with('_') {
      if pl_only && !name_str.starts_with("PRELOAD_") {
        continue;
      }

      let contents = fs::read_to_string(full_path.path());

      if contents.is_err() {
        continue;
      }

      plugin_list.insert(format!("{:?}", name), contents.unwrap());
    }
  }

  Ok(plugin_list)
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
pub fn get_plugin_list() -> Result<Vec<Plugin>, String> {
  let plugins_dir = get_plugin_dir();
  let mut plugin_list: Vec<Plugin> = Vec::new();

  let plugin_folders =
    fs::read_dir(plugins_dir).map_err(|e| format!("Error reading directory: {}", e))?;

  for entry in plugin_folders {
    let full_path = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
    let meta = full_path
      .metadata()
      .map_err(|e| format!("Error getting metadata: {}", e))?;

    if !meta.is_dir() {
      let name = full_path.file_name();
      let name_str = name.to_str().unwrap_or("");
      let disabled = name_str.starts_with('_');
      let preload = name_str.contains("PRELOAD");

      let mut plugin_name = name_str.to_string();

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
  }

  Ok(plugin_list)
}

#[tauri::command]
pub fn toggle_plugin(name: String) -> Result<bool, String> {
  let plugins_dir = get_plugin_dir();
  let folders =
    fs::read_dir(&plugins_dir).map_err(|e| format!("Error reading directory: {}", e))?;

  for path in folders {
    let full_path = path.map_err(|e| format!("Error reading directory entry: {}", e))?;
    let meta = full_path
      .metadata()
      .map_err(|e| format!("Error getting metadata: {}", e))?;
    let file_name_os = full_path.file_name();
    let file_name = file_name_os.to_str().ok_or("Error getting file name")?;

    if meta.is_dir() {
      continue;
    }

    let mut plugin_name = String::from(&name);

    if plugin_name.starts_with('_') {
      plugin_name = file_name.replacen('_', "", 1);
    }

    if file_name.contains(&plugin_name) {
      let mut new_name = String::from('_') + file_name;

      if file_name.starts_with('_') {
        new_name = file_name.replacen('_', "", 1);
      }

      // Disable the folder
      fs::rename(plugins_dir.join(file_name), plugins_dir.join(new_name))
        .map_err(|e| format!("Error renaming file: {}", e))?;

      return Ok(file_name.starts_with('_'));
    }
  }

  Ok(false)
}

#[tauri::command]
pub fn toggle_preload(name: String) -> Result<bool, String> {
  let plugins_dir = get_plugin_dir();
  let folders =
    fs::read_dir(&plugins_dir).map_err(|e| format!("Error reading directory: {}", e))?;

  for path in folders {
    let full_path = path.map_err(|e| format!("Error reading directory entry: {}", e))?;
    let meta = full_path
      .metadata()
      .map_err(|e| format!("Error getting metadata: {}", e))?;
    let file_name_os = full_path.file_name();
    let file_name = file_name_os.to_str().ok_or("Error getting file name")?;

    if meta.is_dir() {
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

      // Ensure we keep the disabled state
      if disabled {
        new_name = String::from("_") + &new_name;
      }

      new_name += ".js";

      // Disable/enable preload
      fs::rename(plugins_dir.join(file_name), plugins_dir.join(&new_name))
        .map_err(|e| format!("Error renaming file: {}", e))?;

      return Ok(!preloaded);
    }
  }

  Ok(false)
}
