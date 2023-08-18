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
pub fn open_scheme(path: PathBuf) {
  Command::new("xdg-open").arg(path).spawn().unwrap();
}
