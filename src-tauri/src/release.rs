use tauri::Manager;

use crate::util::logger::log;
use crate::util::paths::{config_is_local, updater_dir};

#[tauri::command]
pub async fn update_check(win: tauri::Window) -> Vec<String> {
  let mut to_update = vec![];

  log("Checking for updates...".to_string());

  let main_rel = maybe_latest_main_release(&win).await;

  if main_rel.is_ok() && main_rel.unwrap() {
    log("Available update for Dorion!".to_string());
    to_update.push("dorion".to_string());
  }

  to_update
}

#[tauri::command]
pub async fn do_update(win: tauri::Window, to_update: Vec<String>) {
  let updater_path = updater_dir(&win);
  let mut updater = std::process::Command::new(updater_path);

  #[cfg(not(target_os = "linux"))]
  if to_update.contains(&"dorion".to_string()) {
    log("Updating Dorion...".to_string());

    updater.arg(String::from("--main"));
    updater.arg(String::from("true"));
  }

  // If we have a local config, we are a portable install, so pass that too
  if config_is_local() {
    updater.arg("--local");
    updater.arg("true");
  }

  let mut process = match updater.spawn() {
    Ok(p) => p,
    Err(e) => {
      log(format!("Failed to spawn updater process: {}", e));
      return;
    }
  };

  // Wait for the updater to finish
  match process.wait() {
    Ok(_) => (),
    Err(e) => {
      log(format!("Failed to wait for updater process: {}", e));
      return;
    }
  }

  win.emit("update_complete", ()).unwrap_or_default();
}

pub async fn maybe_latest_main_release(
  win: &tauri::Window,
) -> Result<bool, Box<dyn std::error::Error + Sync + Send>> {
  let url = "https://api.github.com/repos/SpikeHD/Dorion/releases/latest";
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "Dorion")
    .send()
    .await?;
  let text = response.text().await?;

  // Parse "tag_name" from JSON
  let json: serde_json::Value = serde_json::from_str(&text)?;
  let tag_name = json["tag_name"]
    .as_str()
    .ok_or("Failed to extract 'tag_name' from JSON")?;

  let handle = win.app_handle();
  let app_version = &handle.package_info().version;
  let version_str = format!(
    "v{}.{}.{}",
    app_version.major, app_version.minor, app_version.patch
  );

  Ok(tag_name != version_str)
}
