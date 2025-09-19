#[cfg(not(target_os = "macos"))]
#[cfg(feature = "hotkeys")]
use livesplit_hotkey::Hotkey;
#[cfg(feature = "hotkeys")]
#[cfg(not(target_os = "macos"))]
use livesplit_hotkey::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeybindChangedEvent {
  pub keys: Vec<KeyStruct>,
  pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyStruct {
  pub name: String,
  pub code: String,
}

#[cfg(feature = "hotkeys")]
#[cfg(not(target_os = "macos"))]
pub fn js_keycode_to_key(keycode: String) -> Option<KeyCode> {
  match keycode.as_str() {
    // TODO fix for PTT since it uses a slightly different system that doesn't differentiate
    "Control" => Some(KeyCode::ControlLeft),
    "Shift" => Some(KeyCode::ShiftLeft),
    "Alt" => Some(KeyCode::AltLeft),
    "Meta" => Some(KeyCode::MetaLeft),
    _ => {
      use std::str::FromStr;

      KeyCode::from_str(&keycode).ok()
    }
  }
}

#[cfg(feature = "hotkeys")]
#[cfg(not(target_os = "macos"))]
pub fn keystructs_to_hotkey(keys: &[KeyStruct]) -> Option<Hotkey> {
  use livesplit_hotkey::Modifiers;

  let key = keys.last().and_then(|k| js_keycode_to_key(k.code.clone()));
  let mut modifiers = Modifiers::empty();

  if let Some(key_code) = key {
    // Everything from before the last key should be a modifier
    for k in keys.iter().take(keys.len() - 1) {
      if let Some(modifier) = js_keycode_to_key(k.code.clone()) {
        match modifier {
          KeyCode::ControlLeft => modifiers.insert(Modifiers::CONTROL),
          KeyCode::ControlRight => modifiers.insert(Modifiers::CONTROL),
          KeyCode::ShiftLeft => modifiers.insert(Modifiers::SHIFT),
          KeyCode::ShiftRight => modifiers.insert(Modifiers::SHIFT),
          KeyCode::AltLeft => modifiers.insert(Modifiers::ALT),
          KeyCode::AltRight => modifiers.insert(Modifiers::ALT),
          KeyCode::MetaLeft => modifiers.insert(Modifiers::META),
          KeyCode::MetaRight => modifiers.insert(Modifiers::META),
          _ => continue,
        }
      }
    }

    return Some(Hotkey {
      key_code,
      modifiers,
    });
  }

  None
}
