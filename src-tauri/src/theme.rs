use std::fs;

#[tauri::command]
pub fn get_theme(name: String) -> String {
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let config_file = exe_dir.join("themes").join(name).join("index.css");

  fs::read_to_string(config_file).unwrap_or_else(|_| "".to_string())
}

#[tauri::command]
pub fn get_theme_names() -> Vec<String> {
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let themes_dir = exe_dir.join("themes");

  if fs::metadata(&themes_dir).is_err() {
    match fs::create_dir_all(&themes_dir) {
      Ok(()) => (),
      Err(e) => {
        println!("Error creating themes dir: {}", e);

        return vec![String::new()]
      }
    };
  }

  let theme_folders = fs::read_dir(&themes_dir).unwrap();
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
