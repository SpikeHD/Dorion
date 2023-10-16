use std::io::BufRead;
use tauri::Manager;

use crate::util::paths::{get_injection_dir, updater_dir};

#[tauri::command]
pub async fn update_check(win: tauri::Window) -> Vec<String> {
  let mut to_update = vec![];

  println!("Checking for updates...");

  if maybe_latest_injection_release().await {
    println!("Available update for Vencordorion!");
    to_update.push("vencordorion".to_string());
  }

  if maybe_latest_main_release(&win).await {
    println!("Available update for Dorion!");
    to_update.push("dorion".to_string());
  }
  
  to_update
}

#[tauri::command]
pub async fn do_update(win: tauri::Window, to_update: Vec<String>) {
  let updater_path = updater_dir(&win);
  let mut updater = std::process::Command::new(updater_path);

  if to_update.contains(&"vencordorion".to_string()) {
    let injection_path = get_injection_dir(Some(&win));
    println!("Updating Vencordorion...");

    updater.arg(String::from("--vencord"));
    updater.arg(
      injection_path
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('\\', "/"),
    );
  }

  if to_update.contains(&"dorion".to_string()) {
    println!("Updating Dorion...");

    updater.arg(String::from("--main"));
    updater.arg(String::from("true"));
  }

  let mut process = updater.spawn().unwrap();

  // Wait for the updater to finish
  process.wait().unwrap();

  win.emit("update_complete", ()).unwrap();
}

#[tauri::command]
pub async fn maybe_latest_injection_release() -> bool {
  // See if there is a new release in Vencordorion
  let url = "https://api.github.com/repos/SpikeHD/Vencordorion/releases/latest";
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "Dorion")
    .send()
    .await
    .unwrap();
  let text = response.text().await.unwrap();

  // Parse "tag_name" from JSON
  let json: serde_json::Value = serde_json::from_str(&text).unwrap();
  let tag_name = json["tag_name"].as_str().unwrap();

  // Read previous version from vencord.version (located in binary folder)
  let mut path = get_injection_dir(None);
  path.push("vencord.version");

  let mut previous_version = String::new();
  if let Ok(file) = std::fs::File::open(&path) {
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
      previous_version = line.unwrap();
    }
  }

  if tag_name == previous_version {
    return false;
  }

  true
}

pub async fn maybe_latest_main_release(win: &tauri::Window) -> bool {
  let url = "https://api.github.com/repos/SpikeHD/Dorion/releases/latest";
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "Dorion")
    .send()
    .await
    .unwrap();
  let text = response.text().await.unwrap();

  // Parse "tag_name" from JSON
  let json: serde_json::Value = serde_json::from_str(&text).unwrap();
  let tag_name = json["tag_name"].as_str().unwrap();

  let handle = win.app_handle();
  let app_version = &handle.package_info().version;
  let version_str = format!("v{}.{}.{}", app_version.major, app_version.minor, app_version.patch);

  println!("Comparing {} to {}", tag_name, version_str);

  if tag_name == version_str {
    return false;
  }

  true
}
