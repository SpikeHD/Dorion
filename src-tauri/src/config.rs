use std::fs;

pub fn init() {
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let config_file = exe_dir.join("config.json");

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      config_file,
      r#"{ "theme": "none", "zoom": 100, "client_type": "default" }"#,
    )
    .unwrap();
  }
}

#[tauri::command]
pub fn read_config_file() -> String {
  init();

  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let config_file = exe_dir.join("config.json");

  fs::read_to_string(config_file).unwrap()
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  init();

  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let config_file = exe_dir.join("config.json");

  fs::write(config_file, contents).unwrap()
}
