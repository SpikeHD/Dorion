use std::io::BufRead;

use crate::util::paths::{updater_dir, get_injection_dir};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Release {
  pub tag_name: String,
  pub link: String,
}

#[tauri::command]
pub async fn get_latest_release() -> Release {
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

  // Parse "html_url"
  let link = json["html_url"].as_str().unwrap();

  Release {
    tag_name: tag_name.to_string(),
    link: link.to_string(),
  }
}

#[tauri::command]
pub async fn update_check() -> Vec<String> {
  let mut to_update = vec![];

  println!("Checking for updates...");

  if maybe_latest_injection_release().await {
    println!("Available update for Vencordorion!");
    to_update.push("vencordorion".to_string());
  }

  // TODO: Dorion autoupdate check
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
        .replace("\\", "/")
    );
  }

  let mut process = updater.spawn().unwrap();

  // Wait for the updater to finish
  process.wait().unwrap();

  win.emit("update_complete", {}).unwrap();
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
