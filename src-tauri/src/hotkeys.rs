use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time::Duration};

use crate::config;

// Globally store the PTT keys
static mut PTT_KEYS: Vec<String> = Vec::new();

pub fn start_hotkey_watcher() {
  // Get ptt enabled and ptt keys
  let ptt_enabled = crate::config::get_ptt();
  let mut ptt_state = false;

  // Set global PTT keys
  unsafe {
    PTT_KEYS = crate::config::get_ptt_keys();
  }

  if !ptt_enabled {
    return;
  }

  thread::spawn(move || {
    let device_state = DeviceState::new();
    loop {
      let ptt_keys = unsafe { PTT_KEYS.clone() };
      let keys: Vec<Keycode> = device_state.get_keys();

      // Recreate keys as a strin vector
      let mut keys_str: Vec<String> = Vec::new();
      for key in keys {
        keys_str.push(key.to_string());
      }

      // Check if held keys matches all PTT keys
      let mut ptt_held = true;

      for key in &ptt_keys {
        // if the key is "Control" or "Shift", match both L and R version
        if key == "Control" {
          if !keys_str.contains(&"LControl".to_string()) && !keys_str.contains(&"RControl".to_string()) {
            ptt_held = false;
          }
        } else if key == "Shift" {
          if !keys_str.contains(&"LShift".to_string()) && !keys_str.contains(&"RShift".to_string()) {
            ptt_held = false;
          }
        }

        // If the key is a single character, make sure we are comparing an uppercase version of ptt_key
        if key.len() == 1 {
          if !keys_str.contains(&key.to_uppercase()) {
            ptt_held = false;
          }
        }
      }

      if ptt_held && ptt_state == false {
        // Do PTT
        println!("PTT is held!");
        ptt_state = true;
      } else if ptt_state == true && !ptt_held {
        // Stop PTT
        println!("PTT is not held!");

        ptt_state = false;
      }

      // Small delay to reduce CPU usage
      thread::sleep(Duration::from_millis(5));
    }
  });
}

#[tauri::command]
pub fn save_ptt_keys(keys: Vec<String>) {
  let config = config::read_config_file();
  let mut parsed = serde_json::from_str(config.as_str()).unwrap_or_else(|_| config::default_config());

  parsed.push_to_talk_keys = Option::from(keys.clone());

  let new_config = serde_json::to_string(&parsed).unwrap();

  config::write_config_file(new_config);

  // Also set the global PTT keys
  unsafe {
    PTT_KEYS = keys.clone();
  }
}
