use std::{fs, path::PathBuf};

pub struct ReleaseData {
  pub tag_name: String,
  pub release_names: Vec<String>,
}

pub fn get_release(user: impl AsRef<str>, repo: impl AsRef<str>) -> ReleaseData {
  let url = format!(
    "https://api.github.com/repos/{}/{}/releases/latest",
    user.as_ref(),
    repo.as_ref()
  );
  let client = reqwest::blocking::Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "Dorion")
    .send()
    .unwrap();
  let text = response.text().unwrap();

  // Parse "tag_name" from JSON
  let json: serde_json::Value = serde_json::from_str(&text).unwrap();
  let tag_name = json["tag_name"].as_str().unwrap();
  let releases = json["assets"].as_array().unwrap();

  let mut release_names = Vec::new();

  for release in releases {
    let name = release["name"].as_str().unwrap();
    release_names.push(name.to_string());
  }

  ReleaseData {
    tag_name: tag_name.to_string(),
    release_names,
  }
}

pub fn download_release(
  user: impl AsRef<str>,
  repo: impl AsRef<str>,
  tag_name: impl AsRef<str>,
  release_name: impl AsRef<str>,
  path: PathBuf,
) -> PathBuf {
  let url = format!(
    "https://github.com/{}/{}/releases/download/{}/{}",
    user.as_ref(),
    repo.as_ref(),
    tag_name.as_ref(),
    release_name.as_ref()
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
    fs::create_dir_all(file_path.parent().unwrap()).unwrap();
  }

  // Write the file
  fs::write(&file_path, response.bytes().unwrap()).unwrap();

  // Return the path of the file
  file_path
}
