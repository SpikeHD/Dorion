use pico_args::Arguments;
use std::path::PathBuf;

#[cfg(not(target_os = "linux"))]
use crate::github::{download_release, get_release};
#[cfg(not(target_os = "linux"))]
mod github;

// If you are reading this, you probably don't need to be. Dorion updates on it's own, silly!
struct UpdaterArguments {
  main: bool,
  #[cfg(target_os = "windows")]
  local: bool,
}

pub fn main() {
  let mut pargs = Arguments::from_env();
  let args = UpdaterArguments {
    main: pargs.contains("--main"),
    #[cfg(target_os = "windows")]
    local: pargs.contains("--local"),
  };

  // This should happen second
  if args.main {
    #[cfg(target_os = "windows")]
    if args.local {
      update_main_kinda();
      return;
    }

    update_main();
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
      std::fs::remove_file(test_file).expect("Failed to remove test file");
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
  let install = std::env::current_exe().unwrap_or_default();

  let mut binding = std::process::Command::new("powershell.exe");
  let cmd = binding.arg("-command").arg(format!(
    "Start-Process -filepath '{}' -verb runas -ArgumentList @({})",
    install.into_os_string().into_string().unwrap(),
    // get program args (without first one) and join by ,
    std::env::args()
      .skip(1)
      .map(|arg| format!("'\"{arg}\"'"))
      .collect::<Vec<String>>()
      .join(",")
  ));

  println!("Executing: {:?}", cmd);

  let mut process = match cmd.spawn() {
    Ok(p) => p,
    Err(e) => {
      eprintln!("Failed to spawn updater process: {}", e);
      return;
    }
  };

  // Wait for the updater to finish
  match process.wait() {
    Ok(_) => (),
    Err(e) => {
      eprintln!("Failed to wait for updater process: {}", e);
      return;
    }
  }

  std::process::exit(0);
}

/**
 * Download the MSI and install
 */
#[cfg(target_os = "windows")]
pub fn update_main() {
  let release = match get_release("SpikeHD", "Dorion") {
    Ok(release) => release,
    Err(e) => {
      println!("Failed to get release: {}", e);
      return;
    }
  };

  println!("Latest Dorion release: {}", release.tag_name);

  // Find the release that ends with ".dmg", that's the MacOS release
  let mut release_name = String::new();

  for name in release.release_names {
    if name.ends_with(".msi") {
      release_name = name;
      break;
    }
  }

  let path = std::env::temp_dir();

  println!("Downloading {}...", release_name);

  let release_path = download_release(
    "SpikeHD",
    "Dorion",
    release.tag_name.clone(),
    release_name.clone(),
    path.clone(),
  );

  // Kill Dorion BEFORE we install
  println!("Attempting to kill Dorion process...");

  let mut cmd = std::process::Command::new("taskkill");
  cmd.arg("/F");
  cmd.arg("/IM");
  cmd.arg("Dorion.exe");

  match cmd.spawn() {
    Ok(mut p) => p.wait().unwrap(),
    Err(e) => {
      println!("Failed to kill Dorion process: {}", e);
      return;
    }
  };

  println!("Installing {:?}...", release_path.clone());

  // Install from the MSI
  let mut cmd = std::process::Command::new("msiexec");
  cmd.arg("/i");
  cmd.arg(release_path.into_os_string().into_string().unwrap());

  println!("Running {:?}", cmd);

  match cmd.spawn() {
    Ok(_) => (),
    Err(e) => {
      println!("Failed to kill Dorion process: {}", e);
    }
  };

  std::process::exit(0);
}

#[cfg(target_os = "windows")]
pub fn update_main_kinda() {
  let release = match get_release("SpikeHD", "Dorion") {
    Ok(release) => release,
    Err(e) => {
      println!("Failed to get release: {}", e);
      return;
    }
  };

  println!("Latest Dorion release: {}", release.tag_name);

  let arch = if cfg!(target_arch = "aarch64") {
    "win_arm64"
  } else {
    "win64"
  };
  let mut release_name = String::new();

  for name in release.release_names {
    if name.ends_with(".zip") && name.contains(arch) {
      release_name = name;
      break;
    }
  }

  if release_name.is_empty() {
    println!("No portable package found");
    return;
  }

  println!("Downloading {}...", release_name);

  let release_path = download_release(
    "SpikeHD",
    "Dorion",
    release.tag_name.clone(),
    release_name.clone(),
    std::env::temp_dir(),
  );

  let install_dir = match std::env::current_exe()
    .ok()
    .and_then(|exe| exe.parent().map(PathBuf::from))
  {
    Some(dir) => dir,
    None => return,
  };

  println!("Stopping Dorion...");
  let _ = std::process::Command::new("taskkill")
    .args(["/F", "/IM", "Dorion.exe"])
    .status();
  std::thread::sleep(std::time::Duration::from_secs(2));

  if let Ok(exe) = std::env::current_exe() {
    let _ = std::fs::rename(&exe, exe.with_extension("old"));
  }

  println!("Extracting update...");
  let extracted = std::process::Command::new("powershell")
    .args([
      "-Command",
      &format!(
        "Expand-Archive -Force -Path '{}' -DestinationPath '{}'",
        release_path.display(),
        install_dir.display()
      ),
    ])
    .status()
    .map(|status| status.success())
    .unwrap_or(false);

  if !extracted {
    println!("Failed to extract the update");
    return;
  }

  let _ = std::process::Command::new(install_dir.join("Dorion.exe")).spawn();
}

/**
 * Download the DMG and open
 */
#[cfg(target_os = "macos")]
pub fn update_main() {
  let release = match get_release("SpikeHD", "Dorion") {
    Ok(release) => release,
    Err(e) => {
      println!("Failed to get release: {}", e);
      return;
    }
  };

  println!("Latest Dorion release: {}", release.tag_name);

  // Find the release that ends with ".dmg", that's the MacOS release
  let mut release_name = String::new();

  for name in release.release_names {
    let arch = if cfg!(target_arch = "x86_64") {
      "x64"
    } else {
      "aarch64"
    };

    if name.ends_with(".dmg") && name.contains(arch) {
      release_name = name;
      break;
    }
  }

  let path = std::env::temp_dir();

  println!("Downloading {}...", release_name);

  let release_path = download_release(
    "SpikeHD",
    "Dorion",
    release.tag_name.clone(),
    release_name.clone(),
    path.clone(),
  );

  println!("Opening {:?}...", release_path.clone());

  // Open the mounted DMG
  let mut cmd = std::process::Command::new("open");
  cmd.arg(release_path);

  match cmd.spawn() {
    Ok(_) => {}
    Err(e) => {
      println!("Failed to open DMG: {:?}", e);
      return;
    }
  }

  println!("Attempting to kill Dorion process...");

  // Also kill the main Dorion process if we can
  let mut cmd = std::process::Command::new("pkill");
  cmd.arg("-9");
  cmd.arg("Dorion");

  match cmd.spawn() {
    Ok(_) => {}
    Err(e) => {
      println!("Failed to kill Dorion: {:?}", e);
    }
  }

  std::process::exit(0);
}

/**
 * Do nothing, too hard to know where we were sourced from on Linux
 */
#[cfg(target_os = "linux")]
pub fn update_main() {}
