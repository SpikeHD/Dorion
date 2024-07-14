use device_query::{keymap::Keycode, DeviceState};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::atomic::AtomicBool};
use tauri::{Emitter, Listener};

use crate::{
  config::{get_config, set_config},
  functionality::keyboard::js_keycode_to_key,
  log,
};

pub static KEYBINDS_CHANGED: AtomicBool = AtomicBool::new(false);
pub static PTT_ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
struct KeyComboState {
  keys: Vec<Keycode>,
  pressed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct KeybindChangedEvent {
  keys: Vec<KeyStruct>,
  key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyStruct {
  name: String,
  code: String,
}

#[tauri::command]
pub fn get_keybinds() -> HashMap<String, Vec<KeyStruct>> {
  let config = get_config();
  config.keybinds.unwrap_or_default()
}

#[tauri::command]
pub fn set_keybinds(keybinds: HashMap<String, Vec<KeyStruct>>) {
  let mut config = get_config();
  config.keybinds = Some(keybinds);

  set_config(config);

  KEYBINDS_CHANGED.store(true, std::sync::atomic::Ordering::Relaxed);
}

#[tauri::command]
pub fn set_keybind(action: String, keys: Vec<KeyStruct>) {
  let mut keybinds = get_keybinds();
  keybinds.insert(action, keys);

  set_keybinds(keybinds);
}

#[cfg(target_os = "macos")]
pub fn start_keybind_watcher(_win: &tauri::WebviewWindow) {
  log!("Keybinds are not supported on macOS yet.");
}

#[cfg(not(target_os = "macos"))]
pub fn start_keybind_watcher(win: &tauri::WebviewWindow) {
  win.listen("keybinds_changed", |evt| {
    let payload = evt.payload();
    if payload.is_empty() {
      return;
    }

    let keybinds: Vec<KeybindChangedEvent> = serde_json::from_str(payload).unwrap();
    let mut keybinds_map = HashMap::new();

    for keybind in keybinds {
      keybinds_map.insert(keybind.key, keybind.keys);
    }

    set_keybinds(keybinds_map);

    KEYBINDS_CHANGED.store(true, std::sync::atomic::Ordering::Relaxed);
  });

  win.listen("ptt_toggled", |evt| {
    #[derive(Serialize, Deserialize)]
    struct PTTPayload {
      state: bool,
    }

    let payload = evt.payload();

    log!("PTT enabled: {:?}", payload);

    if payload.is_empty() {
      return;
    }

    let state = serde_json::from_str::<PTTPayload>(payload).unwrap();
    PTT_ENABLED.store(state.state, std::sync::atomic::Ordering::Relaxed);
  });

  let win_thrd = win.clone();

  std::thread::spawn(move || loop {
    let keybinds = get_keybinds();
    let mut registered_combos = keybinds
      .iter()
      .map(|(action, keys)| {
        let keycodes = keys
          .iter()
          .map(|key| {
            js_keycode_to_key(key.code.clone())
              .unwrap_or_else(|| {
                log!("Error converting key: {:?}", key);
                Keycode::Key0
              })
              .clone()
          })
          .collect::<Vec<Keycode>>();

        (
          action.clone(),
          KeyComboState {
            keys: keycodes,
            pressed: false,
          },
        )
      })
      .collect::<HashMap<String, KeyComboState>>();

    loop {
      std::thread::sleep(std::time::Duration::from_millis(100));

      if KEYBINDS_CHANGED.load(std::sync::atomic::Ordering::Relaxed) {
        KEYBINDS_CHANGED.store(false, std::sync::atomic::Ordering::Relaxed);

        log!("Keybinds changed, restarting keybind watcher...");
        break;
      }

      // emit keybind_pressed event when pressed, and keybind_released when released
      // TODO maybe consider using event listeners
      for (action, combo) in registered_combos.iter_mut() {
        let mut all_pressed = true;
        let key_state = DeviceState::new().query_keymap();

        for key in &combo.keys {
          if !key_state.contains(key) {
            all_pressed = false;
            break;
          }
        }

        // Special consideration for PUSH_TO_TALK, where we should ask if PTT is enabled first
        // also check for all_pressed so we aren't spam-checking this when not all keys for it are pressed
        if action == "PUSH_TO_TALK" && all_pressed {
          if !PTT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            all_pressed = false;
          }
        }

        if all_pressed && !combo.pressed {
          win_thrd
            .emit("keybind_pressed", Some(action.clone()))
            .unwrap_or_default();
          combo.pressed = true;
        } else if !all_pressed && combo.pressed {
          win_thrd
            .emit("keybind_released", Some(action.clone()))
            .unwrap_or_default();
          combo.pressed = false;
        }
      }
    }
  });
}
