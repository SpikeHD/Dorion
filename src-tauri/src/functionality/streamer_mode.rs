use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Emitter;

use crate::config::get_config;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

// We keep track of this A) To not spam enable and B) to allow for the user to manually disable without it being re-enabled automatically
static OBS_OPEN: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub fn start_streamer_mode_watcher(win: tauri::WebviewWindow) {
  let enabled = get_config().streamer_mode_detection.unwrap_or(false);
  let mut system =
    System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));

  if !enabled {
    return;
  }

  // TODO integrate this into rsRPC somehow
  // Check processes every couple seconds to see if OBS is open
  std::thread::spawn(move || loop {
    std::thread::sleep(std::time::Duration::from_secs(2));

    system.refresh_processes();

    let mut obs_running = false;

    // Meander through the list of processes and check if OBS or Streamlabs OBS is running
    // The delay is to prevent the loop from CRANKIN THIS HOG (the CPU)
    for process in system.processes().values() {
      std::thread::sleep(std::time::Duration::from_millis(5));

      if process.name().to_ascii_lowercase().contains("obs64")
        || process.name().to_ascii_lowercase().contains("streamlabs")
      {
        // If OBS is running, we can break out of the loop and emit the event
        obs_running = true;
        break;
      }
    }
    if obs_running {
      if !OBS_OPEN.load(Ordering::Relaxed) {
        OBS_OPEN.store(true, Ordering::Relaxed);
        win.emit("streamer_mode_toggle", true).unwrap_or_default();
      }

      continue;
    }

    if OBS_OPEN.load(Ordering::Relaxed) {
      OBS_OPEN.store(false, Ordering::Relaxed);
      win.emit("streamer_mode_toggle", false).unwrap_or_default();
    }
  });
}
