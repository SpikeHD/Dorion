use std::{fs, path::PathBuf, fmt::Display};
use reqwest::blocking;
use serde_json::Value;

#[derive(Debug)]
pub struct ReleaseData {
  pub tag_name: String,
  pub release_names: Vec<String>,
}

pub fn get_release(user: impl AsRef<str>, repo: impl AsRef<str>) -> Result<ReleaseData, String> {
  let url = format!(
      "https://api.github.com/repos/{}/{}/releases/latest",
      user.as_ref(),
      repo.as_ref()
  );

  let client = blocking::Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "Dorion")
    .send()
    .map_err(|e| format!("Failed to get latest release from GitHub: {}", e))?;

  let text = response.text().map_err(|e| format!("Failed to read response text: {}", e))?;

  let release: Value = serde_json::from_str(&text)
    .map_err(|e| format!("Failed to parse JSON: {}", e))?;

  let asset_array = release["assets"]
    .as_array();

  if asset_array.is_none() {
    return Err("Failed to parse JSON: assets is not an array".to_string());
  }

  let release_names: Vec<String> = asset_array
    .unwrap()
    .iter()
    .map(|asset| asset["name"].as_str().unwrap().to_string())
    .collect();

  Ok(ReleaseData {
    tag_name: release["tag_name"].as_str().unwrap().to_string(),
    release_names,
  })
}

pub fn download_release(
  user: impl AsRef<str> + Display,
  repo: impl AsRef<str> + Display,
  tag_name: impl AsRef<str> + Display,
  release_name: impl AsRef<str> + Display,
  path: PathBuf,
) -> PathBuf {
  let url = format!(
    "https://github.com/{}/{}/releases/download/{}/{}",
    user,
    repo,
    tag_name,
    release_name
  );

  let client = reqwest::blocking::Client::new();

  let response = client
    .get(url)
    .header("User-Agent", "Dorion")
    .send()
    .unwrap();

  let mut file_path = path.clone();
  file_path.push(release_name.as_ref());

  println!("Writing to {:?}", file_path);

  // Create_dir_all if needed
  if !file_path.parent().unwrap().exists() {
    fs::create_dir_all(file_path.parent().unwrap())
      .expect("Failed to create directory");
  }

  // Write the file
  fs::write(
    &file_path,
    response.bytes().expect("Failed to read response bytes")
  ).expect("Failed to write file");

  // Return the path of the file
  file_path
}
