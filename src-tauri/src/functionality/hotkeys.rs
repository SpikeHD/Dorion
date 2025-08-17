use livesplit_hotkey::{ConsumePreference, Hook, Hotkey, KeyCode};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  sync::{atomic::AtomicBool, Arc, Mutex},
};
use tauri::{Emitter, Listener};

use crate::{
  config::{get_config, set_config},
  functionality::keyboard::{js_keycode_to_key, keystructs_to_hotkey},
  log,
};

use super::keyboard::{KeyComboState, KeyStruct, KeybindChangedEvent};

pub static KEYBINDS_CHANGED: AtomicBool = AtomicBool::new(false);
pub static PTT_ENABLED: AtomicBool = AtomicBool::new(false);

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

pub fn start_keybind_watcher(win: &tauri::WebviewWindow) {
  let hook = match Hook::with_consume_preference(ConsumePreference::PreferNoConsume) {
    Ok(hook) => hook,
    Err(e) => {
      log!("Failed to create keybind hook: {}", e);
      return;
    }
  };
  let hook = Mutex::new(Arc::new(hook));

  register_all_keybinds(&*hook.lock().unwrap(), &get_keybinds());

  win.listen("keybinds_changed", move |evt| {
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
}

fn register_all_keybinds(hook: &Arc<Hook>, keybinds: &HashMap<String, Vec<KeyStruct>>) {
  for (action, keys) in keybinds {
    let hotkey = match keystructs_to_hotkey(&keys) {
      Some(hotkey) => hotkey,
      None => {
        log!("Invalid keybind for action {}: {:?}", action, keys);
        continue;
      }
    };

    let action_clone = action.clone();
    let callback = move |pressed| {
      log!("Keybind triggered: {} | Pressed: {}", action_clone, pressed);
      // TODO handle
    };

    if let Err(e) = hook.register_specific(hotkey, callback) {
      log!("Failed to register keybind for {}: {}", action, e);
    } else {
      log!("Registered keybind for {}: {:?}", action, hotkey);
    }
  }
}
