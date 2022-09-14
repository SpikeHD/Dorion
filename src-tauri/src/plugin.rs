
use std::{fs};

#[tauri::command]
pub fn load_plugins() -> String {
  let mut contents = String::new();
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let plugins_dir = exe_dir.join("plugins");

  if fs::metadata(&plugins_dir).is_err() {
    match fs::create_dir_all(&plugins_dir) {
      Ok(()) => (),
      Err(e) => {
        println!("Error creating plugins dir: {}", e);

        return String::new();
      }
    };
  }

  let plugin_folders = fs::read_dir(&plugins_dir).unwrap();

  for path in plugin_folders {
    if let Err(_path) = path {
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
