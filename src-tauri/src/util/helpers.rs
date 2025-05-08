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
  open::that(path)?;
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

#[cfg(target_os = "windows")]
pub fn is_windows_7() -> bool {
  use windows::{
    Wdk::System::SystemServices::RtlGetVersion, Win32::System::SystemInformation::OSVERSIONINFOW,
  };

  let mut osvi = OSVERSIONINFOW {
    dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOW>() as u32,
    ..Default::default()
  };

  unsafe {
    let _ = RtlGetVersion(&mut osvi);
  }

  osvi.dwMajorVersion == 6 && osvi.dwMinorVersion == 1
}
