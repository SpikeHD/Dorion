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

#[cfg(target_os = "windows")]
pub fn open_scheme(scheme: String) {
  Command::new("start").arg(scheme).spawn().unwrap();
}

#[cfg(target_os = "macos")]
pub fn open_scheme(scheme: String) {
  Command::new("open").arg(scheme).spawn().unwrap();
}

#[cfg(target_os = "linux")]
pub fn open_scheme(path: String) {
  Command::new("xdg-open").arg(path).spawn().unwrap();
}

pub fn move_injection_scripts(win: &tauri::Window, with_ven: bool) {
  #[cfg(target_os = "windows")]
  let appdata = tauri::api::path::data_dir().unwrap();

  #[cfg(not(target_os = "windows"))]
  let appdata = tauri::api::path::config_dir().unwrap();

  let injection_dir = appdata.join("dorion").join("injection");
  
  let packaged_injection_dir = win
    .app_handle()
    .path_resolver()
    .resolve_resource(PathBuf::from("injection"))
    .unwrap();

  let mut injection_folder = injection_dir.clone();
  injection_folder.push("injection");

  let mut copy_to = injection_dir.clone();
  copy_to.pop();

  // If the injection folder doesn't exist, create it and re-run with everything
  if std::fs::metadata(&injection_folder).is_err() {
    std::fs::create_dir_all(&injection_folder).unwrap();

    move_injection_scripts(win, true);
    return;
  }

  // If with_ven is true, we can just copy EVERYTHING
  if with_ven {
    fs_extra::dir::copy(
      packaged_injection_dir,
      copy_to,
      &fs_extra::dir::CopyOptions::new()
    ).unwrap();
    return;
  }

  // Otherwise, we need to only grab preinject_min.js and injection_min.js
  let mut preinject_path = packaged_injection_dir.clone();
  preinject_path.push("preinject_min.js");

  let mut inject_path = packaged_injection_dir.clone();
  inject_path.push("injection_min.js");

  std::fs::copy(preinject_path, injection_dir.join("preinject_min.js")).unwrap();
  std::fs::copy(inject_path, injection_dir.join("injection_min.js")).unwrap();
}