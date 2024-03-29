use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

use crate::config;
use crate::log;

// Globally store the PTT keys
static PTT_KEYS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static PTT_ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PTTEvent {
  pub state: bool,
}

#[cfg(target_os = "macos")]
pub fn start_hotkey_watcher(_win: tauri::Window) {}

#[cfg(not(target_os = "macos"))]
pub fn start_hotkey_watcher(win: tauri::Window) {
  use device_query::{DeviceQuery, DeviceState, Keycode};
  use std::{thread, time::Duration};

  let config = config::get_config();
  let mut ptt_state = false;

  // Set global PTT keys
  *PTT_KEYS.lock().unwrap() = config.push_to_talk_keys.unwrap_or_default();
  PTT_ENABLED.store(config.push_to_talk.unwrap_or_default(), Ordering::Relaxed);

  thread::spawn(move || {
    let device_state = DeviceState::new();
    loop {
      if !PTT_ENABLED.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
        continue;
      }

      let ptt_keys = PTT_KEYS.lock().unwrap().clone();
      let keys: Vec<Keycode> = device_state.get_keys();

      // Recreate keys as a string vector
      let mut keys_str: Vec<String> = Vec::new();
      for key in keys {
        if key.to_string() == "LControl" || key.to_string() == "RControl" {
          keys_str.push("Control".to_string());
          continue;
        }

        if key.to_string() == "LShift" || key.to_string() == "RShift" {
          keys_str.push("Shift".to_string());
          continue;
        }

        if key.to_string() == "LAlt" || key.to_string() == "RAlt" {
          keys_str.push("Alt".to_string());
          continue;
        }

        keys_str.push(key.to_string());
      }

      // Check if held keys matches all PTT keys
      let mut ptt_held = true;

      for key in ptt_keys {
        if !keys_str.contains(&key) {
          ptt_held = false;
          break;
        }
      }

      if ptt_held && !ptt_state {
        // Do PTT
        win
          .emit("ptt_toggle", PTTEvent { state: true })
          .unwrap_or_else(|_| log!("Error sending PTT event!"));
        ptt_state = true;
      } else if ptt_state && !ptt_held {
        // Stop PTT
        win
          .emit("ptt_toggle", PTTEvent { state: false })
          .unwrap_or_else(|_| log!("Error sending PTT toggle event!"));
        ptt_state = false;
      }

      // Small delay to reduce CPU usage
      thread::sleep(Duration::from_millis(10));
    }
  });
}

#[tauri::command]
pub fn save_ptt_keys(keys: Vec<String>) -> Result<(), String> {
  let config = config::read_config_file();
  let mut parsed =
    serde_json::from_str(config.as_str()).unwrap_or_else(|_| config::default_config());

  parsed.push_to_talk_keys = Option::from(keys.clone());

  let new_config = serde_json::to_string(&parsed);

  match new_config {
    Ok(new_config) => {
      config::write_config_file(new_config);

      *PTT_KEYS.lock().unwrap() = keys;
      Ok(())
    }
    Err(e) => Err(e.to_string()),
  }
}

#[tauri::command]
pub fn toggle_ptt(state: bool) -> Result<(), String> {
  let config = config::read_config_file();
  let mut parsed =
    serde_json::from_str(config.as_str()).unwrap_or_else(|_| config::default_config());

  parsed.push_to_talk = Option::from(state);

  let new_config = serde_json::to_string(&parsed);

  log!("PTT set to: {}", state);

  match new_config {
    Ok(new_config) => {
      config::write_config_file(new_config);

      PTT_ENABLED.store(state, Ordering::Relaxed);

      Ok(())
    }
    Err(e) => Err(e.to_string()),
  }
}
