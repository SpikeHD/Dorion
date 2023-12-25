use super::paths::*;
use std::path::*;
use std::process::Command;

#[tauri::command]
pub fn open_plugins() {
  let plugin_folder = get_plugin_dir();

  open_folder(plugin_folder).unwrap_or_default()
}

#[tauri::command]
pub fn open_themes() {
  let theme_folder = get_theme_dir();

  open_folder(theme_folder).unwrap_or_default()
}

fn open_folder(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(target_os = "windows")]
  Command::new("explorer").arg(path).spawn()?;

  #[cfg(target_os = "macos")]
  Command::new("open").arg(path).spawn()?;

  #[cfg(target_os = "linux")]
  Command::new("xdg-open").arg(path).spawn()?;

  Ok(())
}

pub fn open_scheme(scheme: String) -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(target_os = "windows")]
  Command::new("start").arg(scheme).spawn()?;

  #[cfg(target_os = "macos")]
  Command::new("open").arg(scheme).spawn()?;

  #[cfg(target_os = "linux")]
  Command::new("xdg-open").arg(scheme).spawn()?;

  Ok(())
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
