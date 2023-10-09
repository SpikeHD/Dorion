use std::io::BufRead;

use crate::injection::{injection_runner::injection_dir, self};

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

  return to_update;
}

#[tauri::command]
pub async fn do_update(win: tauri::Window, to_update: Vec<String>) -> () {
  let mut args = vec![];

  if to_update.contains(&"vencordorion".to_string()) {
    let injection_path = injection_dir(win);
    let injection_path = format!("{}", injection_path.to_str().unwrap());
    println!("Updating Vencordorion...");
    args.push(String::from("--vencord"));
    args.push(injection_path);
  }

  if args.len() > 0 {
    // Run the updater as a seperate process
    let mut updater_path = std::env::current_exe().unwrap();
    updater_path.pop();
    updater_path.push("updater");

    let mut updater = std::process::Command::new(updater_path);
    
    for arg in args {
      updater.arg(arg);
    }

    let mut process = updater.spawn().unwrap();
    
    // Wait for the updater to finish
    process.wait().unwrap();
  }
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
  let mut path = std::env::current_exe().unwrap();
  path.pop();
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