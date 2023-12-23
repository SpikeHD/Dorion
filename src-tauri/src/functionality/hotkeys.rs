use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time::Duration};

use crate::config;
use crate::util::logger::{self, log};

// Globally store the PTT keys
static mut PTT_KEYS: Vec<String> = Vec::new();
static mut PTT_ENABLED: bool = false;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PTTEvent {
  pub state: bool,
}

pub fn start_hotkey_watcher(win: tauri::Window) {
  let mut ptt_state = false;

  // Set global PTT keys
  unsafe {
    PTT_KEYS = crate::config::get_config()
      .push_to_talk_keys
      .unwrap_or_default();
    PTT_ENABLED = crate::config::get_config().push_to_talk.unwrap_or(false);
  }

  thread::spawn(move || {
    let device_state = DeviceState::new();
    loop {
      if unsafe { !PTT_ENABLED } {
        thread::sleep(Duration::from_millis(20));
        continue;
      }

      let ptt_keys = unsafe { PTT_KEYS.clone() };
      let keys: Vec<Keycode> = device_state.get_keys();

      // Recreate keys as a string vector
      let mut keys_str: Vec<String> = Vec::new();
      for key in keys {
        keys_str.push(key.to_string());
      }

      // Check if held keys matches all PTT keys
      let mut ptt_held = true;

      for key in &ptt_keys {
        // if the key is "Control" or "Shift", match both L and R version
        if key == "Control"
          && !keys_str.contains(&"LControl".to_string())
          && !keys_str.contains(&"RControl".to_string())
        {
          ptt_held = false;
        }

        if key == "Shift"
          && !keys_str.contains(&"LShift".to_string())
          && !keys_str.contains(&"RShift".to_string())
        {
          ptt_held = false;
        }

        // If the key is a single regular character, make sure we are comparing an uppercase version of ptt_key
        if key.len() == 1 && !keys_str.contains(&key.to_uppercase()) {
          ptt_held = false;
        }
      }

      if ptt_held && !ptt_state {
        // Do PTT
        win
          .emit("ptt_toggle", PTTEvent { state: true })
          .unwrap_or_else(|_| log("Error sending PTT event!".to_string()));
        ptt_state = true;
      } else if ptt_state && !ptt_held {
        // Stop PTT
        win
          .emit("ptt_toggle", PTTEvent { state: false })
          .unwrap_or_else(|_| log("Error sending PTT toggle event!".to_string()));
        ptt_state = false;
      }

      // Small delay to reduce CPU usage
      thread::sleep(Duration::from_millis(5));
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

      unsafe {
        PTT_KEYS = keys;
      }
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

  logger::log(format!("PTT set to: {}", state));

  match new_config {
    Ok(new_config) => {
      config::write_config_file(new_config);

      unsafe {
        PTT_ENABLED = state;
      }

      Ok(())
    }
    Err(e) => Err(e.to_string()),
  }
}
