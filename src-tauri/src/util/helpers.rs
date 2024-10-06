use super::paths::*;
use base64::{engine::general_purpose, Engine as _};
use std::path::*;
use std::process::Command;

#[tauri::command]
pub async fn fetch_image(url: String) -> Option<String> {
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "Dorion")
    .send()
    .await
    .unwrap();

  // extract the content type
  let content_type = response
    .headers()
    .get("content-type")
    .and_then(|value| value.to_str().ok())
    .map(|s| s.to_owned())
    .unwrap_or_else(|| {
      eprintln!("Error: Unable to get content type");
      String::new()
    });

  if !content_type.starts_with("image") {
    return None;
  }

  let bytes = response.bytes().await.unwrap();
  let base64 = general_purpose::STANDARD.encode(bytes);
  let image = format!("data:{content_type};base64,{base64}");

  Some(image)
}

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

#[tauri::command]
pub fn open_extensions() {
  let extension_folder = get_extensions_dir();

  open_folder(extension_folder).unwrap_or_default()
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

#[tauri::command]
pub fn get_platform() -> &'static str {
  #[cfg(target_os = "windows")]
  return "windows";

  #[cfg(target_os = "macos")]
  return "macos";

  #[cfg(target_os = "linux")]
  "linux"
}
