use std::path::PathBuf;
use clap::{command, Parser};

/// If you are reading this, you probably don't need to be. Dorion updates on it's own, silly!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// Update Dorion
  #[arg(short = 'm', long)]
  main: Option<String>,

  /// Path to injection folder
  #[arg(short = 'v', long)]
  vencord: Option<String>,
}

pub fn main() {
  let args = Args::parse();

  // This should always be run by Dorion itself, which means it will likely not have admin perms, so we request them before anything else
  #[cfg(target_os = "windows")]
  if is_elevated::is_elevated() == false {
    reopen_as_elevated();
  }

  #[cfg(not(target_os = "windows"))]
  sudo::escalate_if_needed().expect("Failed to escalate as root");
  
  if args.main.is_some() {
    update_main();
  }

  if args.vencord.is_some() {
    update_vencordorion(PathBuf::from(args.vencord.unwrap()));
  }
}

#[cfg(target_os = "windows")]
pub fn reopen_as_elevated() {
  let install = std::env::current_exe().unwrap();

  std::process::Command::new("powershell.exe")
    .arg("powershell")
    .arg(format!(
      "-command \"&{{Start-Process -filepath '{}' -verb runas -ArgumentList \"{}\"}}\"",
      install.to_str().unwrap(),
      // Grab all args except first
      std::env::args().skip(1).collect::<Vec<String>>().join(" ")
    ))
    .spawn()
    .expect("Error starting exec as admin");

  exit(0);
}

pub fn update_vencordorion(path: PathBuf) {
  let url = "https://api.github.com/repos/SpikeHD/Vencordorion/releases/latest";
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
    .unwrap();

  let js_response = client
    .get(&js_url)
    .header("User-Agent", "Dorion")
    .send()
    .unwrap();

  // Write both to disk
  let mut css_path = path.clone();
  css_path.push("browser.css");

  let mut js_path = path.clone();
  js_path.push("browser.js");

  std::fs::write(css_path, css_response.text().unwrap()).unwrap();
  std::fs::write(js_path, js_response.text().unwrap()).unwrap();

  // If this succeeds, write the new version to vencord.version
  let mut path = std::env::current_exe().unwrap();
  path.pop();
  path.push("vencord.version");

  std::fs::write(path, tag_name).unwrap();
}

pub fn update_main() {
  // Nothing for now
  return;
}