use tauri::Manager;

use super::paths::*;
use std::path::*;
use std::process::Command;

#[tauri::command]
pub fn open_plugins() {
  let plugin_folder = get_plugin_dir();

  open_folder(plugin_folder)
}

#[tauri::command]
pub fn open_themes() {
  let theme_folder = get_theme_dir();

  open_folder(theme_folder)
}

fn open_folder(path: PathBuf) {
  #[cfg(target_os = "windows")]
  Command::new("explorer").arg(path).spawn().unwrap();

  #[cfg(target_os = "macos")]
  Command::new("open").arg(path).spawn().unwrap();

  #[cfg(target_os = "linux")]
  Command::new("xdg-open").arg(path).spawn().unwrap();
}

pub fn open_scheme(scheme: String) {
  #[cfg(target_os = "windows")]
  Command::new("start").arg(scheme).spawn().unwrap();

  #[cfg(target_os = "macos")]
  Command::new("open").arg(scheme).spawn().unwrap();

  #[cfg(target_os = "linux")]
  Command::new("xdg-open").arg(scheme).spawn().unwrap();
}

#[tauri::command]
pub fn get_platform() -> &'static str {
  #[cfg(target_os = "windows")]
  return "windows";

  #[cfg(target_os = "macos")]
  return "macos";

  #[cfg(target_os = "linux")]
  "linux"
}

pub fn move_injection_scripts(win: &tauri::Window, with_mod: bool) {
  let injection_dir = get_injection_dir(Some(win));

  let packaged_injection_dir = win
    .app_handle()
    .path_resolver()
    .resolve_resource(PathBuf::from("injection"))
    .unwrap();

  let mut copy_to = injection_dir.clone();
  copy_to.pop();

  // If the injection folder doesn't exist, create it and re-run with everything
  if std::fs::metadata(&injection_dir).is_err() {
    std::fs::create_dir_all(&injection_dir).unwrap();

    move_injection_scripts(win, true);
    return;
  }

  // If true, we can just copy EVERYTHING
  if with_mod {
    // Move shelter to injection folder
    std::fs::copy(
      packaged_injection_dir.join("shelter.js"),
      injection_dir.join("shelter.js"),
    )
    .unwrap();
  }
}
