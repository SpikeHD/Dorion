use std::fs;

#[tauri::command]
pub fn get_theme(name: String) -> String {
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let theme_file = exe_dir.join("themes").join(name);

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

  fs::read_to_string(theme_file.join(&css_file)).unwrap_or_else(|_| "".to_string())
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

        return vec![String::new()];
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
