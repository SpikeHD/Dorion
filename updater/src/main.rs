use clap::{command, Parser};
use std::path::PathBuf;

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

  if args.main.is_some() {
    update_main();
  }

  if args.vencord.is_some() {
    if needs_to_elevate(PathBuf::from(args.vencord.clone().unwrap())) {
      println!("Elevating process...");
      elevate();
      return;
    }

    update_vencordorion(PathBuf::from(args.vencord.unwrap()));
  }
}

pub fn elevate() {
  // This should always be run by Dorion itself, which means it will likely not have admin perms, so we request them before anything else
  #[cfg(target_os = "windows")]
  reopen_as_elevated();

  #[cfg(not(target_os = "windows"))]
  sudo::escalate_if_needed().expect("Failed to escalate as root");
}

/**
 * Check if we can already access the folder before elevating
 */
pub fn needs_to_elevate(path: PathBuf) -> bool {
  // Write a test file to the injection folder to see if we have perms
  let mut test_file = path;
  test_file.push("test");

  let write_perms = match std::fs::write(&test_file, "") {
    Ok(()) => {
      // Delete the test file
      std::fs::remove_file(test_file).unwrap();

      true
    }
    Err(e) => {
      println!("Error writing test file: {}", e);
      false
    }
  };

  !write_perms
}

#[cfg(target_os = "windows")]
pub fn reopen_as_elevated() {
  let install = std::env::current_exe().unwrap();

  let mut binding = std::process::Command::new("powershell.exe");
  let cmd = binding.arg("-command").arg(format!(
    "Start-Process -filepath '{}' -verb runas -ArgumentList @({})",
    install.into_os_string().into_string().unwrap(),
    // get program args (without first one) and join by ,
    std::env::args()
      .skip(1)
      .map(|arg| format!("'\"{}\"'", arg))
      .collect::<Vec<String>>()
      .join(",")
  ));

  println!("Executing: {:?}", cmd);

  let mut process = cmd.spawn().unwrap();

  // Wait for the updater to finish
  process.wait().unwrap();

  std::process::exit(0);
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

  println!("Latest Vencordorion release: {}", tag_name);

  // Download browser.css and browser.js from the assets
  let css_url = format!(
    "https://github.com/SpikeHD/Vencordorion/releases/download/{}/browser.css",
    tag_name
  );

  let js_url = format!(
    "https://github.com/SpikeHD/Vencordorion/releases/download/{}/browser.js",
    tag_name
  );

  println!("JS URL: {}", js_url);
  println!("CSS URL: {}", css_url);

  // Fetch both
  let css_response = client
    .get(&css_url)
    .header("User-Agent", "Dorion")
    .send()
    .unwrap();

  println!("Got CSS response");

  let js_response = client
    .get(&js_url)
    .header("User-Agent", "Dorion")
    .send()
    .unwrap();

  println!("Got JS response");

  println!("Writing files to disk...");

  // Write both to disk
  let mut css_path = path.clone();
  css_path.push("browser.css");

  let mut js_path = path.clone();
  js_path.push("browser.js");

  std::fs::write(css_path, css_response.text().unwrap()).unwrap();
  std::fs::write(js_path, js_response.text().unwrap()).unwrap();

  // If this succeeds, write the new version to vencord.version
  let mut ven_path = path.clone();
  ven_path.push("vencord.version");

  std::fs::write(ven_path, tag_name).unwrap();
}

pub fn update_main() {}
