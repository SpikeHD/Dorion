use std::io::BufRead;

use crate::util::paths::get_config_dir;

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
pub async fn maybe_latest_injection_release() {
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

  // Read previous version from vencord.version (located in the same place config.json is)
  let mut path = get_config_dir();
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
    return;
  }

  println!("Found Vencordorion update! Installing...");

  // Download browser.css and browser.js from the assets
  let css_url = format!(
    "https://github.com/SpikeHD/Vencordorion/releases/download/{}/browser.css",
    tag_name
  );

  let js_url = format!(
    "https://github.com/SpikeHD/Vencordorion/releases/download/{}/browser.js",
    tag_name
  );

  // Fetch both
  let css_response = client
    .get(&css_url)
    .header("User-Agent", "Dorion")
    .send()
    .await
    .unwrap();

  let js_response = client
    .get(&js_url)
    .header("User-Agent", "Dorion")
    .send()
    .await
    .unwrap();

  // Write both to disk
  let mut css_path = std::env::current_exe().unwrap();
  css_path.pop();
  css_path.push("injection/browser.css");

  let mut js_path = std::env::current_exe().unwrap();
  js_path.pop();
  js_path.push("injection/browser.js");

  std::fs::write(css_path, css_response.text().await.unwrap()).unwrap();
  std::fs::write(js_path, js_response.text().await.unwrap()).unwrap();

  // If this succeeds, write the new version to vencord.version
  std::fs::write(path, tag_name).unwrap();
}