use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time::Duration};

pub fn start_hotkey_watcher() {
  // Get ptt enabled and ptt keys
  let ptt_enabled = crate::config::get_ptt();
  let ptt_keys = crate::config::get_ptt_keys();

  if !ptt_enabled {
    return;
  }

  thread::spawn(move || {
    let device_state = DeviceState::new();
    loop {
      let keys: Vec<Keycode> = device_state.get_keys();

      // Recreate keys as a strin vector
      let mut keys_str: Vec<String> = Vec::new();
      for key in keys {
        keys_str.push(key.to_string());
      }

      // Check if held keys matches all PTT keys
      let mut ptt_held = true;

      for key in &ptt_keys {
        // If the key is a single character, make sure we are comparing an uppercase version of ptt_key
        if key.len() == 1 {
          if !keys_str.contains(&key.to_uppercase()) {
            ptt_held = false;
          }
        } else if !keys_str.contains(&key.to_string()) {
          ptt_held = false;
        }
      }

      if ptt_held {
        // Do PTT
      } else {
        // Stop PTT
      }

      // Small delay to reduce CPU usage
      thread::sleep(Duration::from_millis(5));
    }
  });
}
