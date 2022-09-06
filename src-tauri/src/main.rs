#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{time::Duration, fs};

#[tauri::command]
fn eval(window: tauri::Window, contents: String) {
  std::thread::spawn(move || {
    std::thread::sleep(Duration::from_millis(1000));
    window.eval(contents.as_str()).unwrap();
  });
}

#[tauri::command]
fn load_plugins() -> String {
  let mut contents = "".to_string();
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let plugins_dir = exe_dir.join("plugins");

  if !fs::metadata(&plugins_dir).is_ok() {
    fs::create_dir_all(&plugins_dir).unwrap();
  }

  let plugin_folders = fs::read_dir(&plugins_dir).unwrap();

  for path in plugin_folders {
    if let Err(_path) = path {
        continue;
    }

    let folder = path.unwrap().file_name().clone();
    let plugin_dir = plugins_dir.join(&folder);
    let index_file = plugin_dir.join("index.js");

    if folder.to_str().unwrap_or("").starts_with("_") {
        continue;
    }

    if fs::metadata(&index_file).is_ok() {
        let plugin_contents = fs::read_to_string(&index_file).unwrap();
        
        contents = format!("{};(() => {{ {} }})()", contents, plugin_contents);
    }
  }

  contents
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        eval,
        load_plugins
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
