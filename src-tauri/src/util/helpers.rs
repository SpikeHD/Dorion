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
    fs_extra::dir::copy(
      packaged_injection_dir,
      copy_to,
      &fs_extra::dir::CopyOptions::new(),
    )
    .unwrap();
    return;
  }
}
