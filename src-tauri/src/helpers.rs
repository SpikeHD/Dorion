use std::path::*;
use std::process::Command;

#[tauri::command]
pub fn open_plugins() {
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let plugin_folder = exe_dir.join("plugins");

  open_folder(plugin_folder)
}

#[tauri::command]
pub fn open_themes() {
  let mut exe_dir = std::env::current_exe().unwrap();
  exe_dir.pop();

  let theme_folder = exe_dir.join("themes");

  open_folder(theme_folder)
}

#[cfg(target_os = "windows")]
fn open_folder(path: PathBuf) {
  Command::new("explorer").arg(path).spawn().unwrap();
}

#[cfg(target_os = "macos")]
fn open_folder(path: PathBuf) {
  Command::new("open").arg(path).spawn().unwrap();
}

#[cfg(target_os = "linux")]
fn open_folder(path: PathBuf) {
  Command::new("xdg-open").arg(path).spawn().unwrap();
}
