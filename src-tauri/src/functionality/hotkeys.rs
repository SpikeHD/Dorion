use livesplit_hotkey::{ConsumePreference, Hook};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
};
use tauri::{Emitter, Listener};

use crate::{
  config::{get_config, set_config},
  functionality::keyboard::keystructs_to_hotkey,
  log,
};

use super::keyboard::{KeyStruct, KeybindChangedEvent};

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
  PTT_ENABLED.store(
    get_config().push_to_talk.unwrap_or(false),
    std::sync::atomic::Ordering::Relaxed,
  );
  let win_hook = win.clone();
  let hook = match new_hook(win_hook.clone()) {
    Ok(hook) => hook,
    Err(e) => {
      log!("Failed to create new keybind hook: {}", e);
      return;
    }
  };
  let hook = Mutex::new(hook);

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

    // Drop and recreate the hook to apply new keybinds
    *hook.lock().unwrap() = match new_hook(win_hook.clone()) {
      Ok(new_hook) => new_hook,
      Err(e) => {
        log!("Failed to recreate keybind hook: {}", e);
        return;
      }
    };

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
    
    let mut config = get_config();
    config.push_to_talk = Some(state.state);
    set_config(config);
  });
}

fn new_hook(win: tauri::WebviewWindow) -> Result<Arc<Hook>, Box<dyn std::error::Error>> {
  let hook = Arc::new(Hook::with_consume_preference(
    ConsumePreference::PreferNoConsume,
  )?);
  // Register keybinds
  register_all_keybinds(&win, &hook, &get_keybinds());
  Ok(hook)
}

fn register_all_keybinds(
  win: &tauri::WebviewWindow,
  hook: &Arc<Hook>,
  keybinds: &HashMap<String, Vec<KeyStruct>>,
) {
  for (action, keys) in keybinds {
    let win = win.clone();
    let hotkey = match keystructs_to_hotkey(keys) {
      Some(hotkey) => hotkey,
      None => {
        log!("Invalid keybind for action {}: {:?}", action, keys);
        continue;
      }
    };

    let action_clone = action.clone();

    if action.starts_with("PUSH") {
      let callback = move |pressed| {
        log!("Keybind triggered: {} | Pressed: {}", action_clone, pressed);
        if !PTT_ENABLED.load(Ordering::Relaxed) && action_clone == "PUSH_TO_TALK" {
          return;
        }

        if pressed {
          win
            .emit("keybind_pressed", action_clone.clone())
            .expect("Failed to emit keybind_pressed event");
        } else {
          win
            .emit("keybind_released", action_clone.clone())
            .expect("Failed to emit keybind_released event");
        }
      };

      match hook.register_specific(hotkey, callback) {
        Ok(_) => {
          log!("Registered PTT keybind: {:?}", hotkey);
        }
        Err(e) => {
          log!("Failed to register PTT keybind: {}: {}", hotkey, e);
        }
      }
    } else {
      let callback = move || {
        log!("Keybind triggered: {}", action_clone);

        win
          .emit("keybind_pressed", action_clone.clone())
          .expect("Failed to emit keybind_pressed event");
      };

      match hook.register(hotkey, callback) {
        Ok(_) => {
          log!("Registered keybind: {:?}", hotkey);
        }
        Err(e) => {
          log!("Failed to register keybind: {}: {}", hotkey, e);
        }
      };
    }
  }
}
