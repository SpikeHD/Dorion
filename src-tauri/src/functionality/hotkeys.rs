use device_query::{keymap::Keycode, DeviceState};
use std::{collections::HashMap, sync::atomic::AtomicBool};

use crate::config::{get_config, set_config};

pub static KEYBINDS_CHANGED: AtomicBool = AtomicBool::new(false);

struct KeyComboState {
  keys: Vec<Keycode>,
  pressed: bool,
}

#[tauri::command]
pub fn get_keybinds() -> HashMap<String, Vec<u16>> {
  let config = get_config();
  config.keybinds.unwrap_or_default()
}

#[tauri::command]
pub fn set_keybinds(keybinds: HashMap<String, Vec<u16>>) {
  let mut config = get_config();
  config.keybinds = Some(keybinds);
  
  set_config(config);
  
  KEYBINDS_CHANGED.store(true, std::sync::atomic::Ordering::Relaxed);
}

// this sucks
pub fn js_keycode_to_key(keycode: u16) -> Option<Keycode> {  
  // Might have to make a PR not gonna lie
  match keycode {
    8 => Some(Keycode::Backspace),
    9 => Some(Keycode::Tab),
    13 => Some(Keycode::Enter),
    16 => Some(Keycode::LShift),
    17 => Some(Keycode::LControl),
    18 => Some(Keycode::LAlt),
    // 19 => Some(Keycode::Pause),
    20 => Some(Keycode::CapsLock),
    27 => Some(Keycode::Escape),
    32 => Some(Keycode::Space),
    33 => Some(Keycode::PageUp),
    34 => Some(Keycode::PageDown),
    35 => Some(Keycode::End),
    36 => Some(Keycode::Home),
    37 => Some(Keycode::Left),
    38 => Some(Keycode::Up),
    39 => Some(Keycode::Right),
    40 => Some(Keycode::Down),
    45 => Some(Keycode::Insert),
    46 => Some(Keycode::Delete),
    48 => Some(Keycode::Key0),
    49 => Some(Keycode::Key1),
    50 => Some(Keycode::Key2),
    51 => Some(Keycode::Key3),
    52 => Some(Keycode::Key4),
    53 => Some(Keycode::Key5),
    54 => Some(Keycode::Key6),
    55 => Some(Keycode::Key7),
    56 => Some(Keycode::Key8),
    57 => Some(Keycode::Key9),
    65 => Some(Keycode::A),
    66 => Some(Keycode::B),
    67 => Some(Keycode::C),
    68 => Some(Keycode::D),
    69 => Some(Keycode::E),
    70 => Some(Keycode::F),
    71 => Some(Keycode::G),
    72 => Some(Keycode::H),
    73 => Some(Keycode::I),
    74 => Some(Keycode::J),
    75 => Some(Keycode::K),
    76 => Some(Keycode::L),
    77 => Some(Keycode::M),
    78 => Some(Keycode::N),
    79 => Some(Keycode::O),
    80 => Some(Keycode::P),
    81 => Some(Keycode::Q),
    82 => Some(Keycode::R),
    83 => Some(Keycode::S),
    84 => Some(Keycode::T),
    85 => Some(Keycode::U),
    86 => Some(Keycode::V),
    87 => Some(Keycode::W),
    88 => Some(Keycode::X),
    89 => Some(Keycode::Y),
    90 => Some(Keycode::Z),
    91 => Some(Keycode::LMeta),
    92 => Some(Keycode::RMeta),
    96 => Some(Keycode::Numpad0),
    97 => Some(Keycode::Numpad1),
    98 => Some(Keycode::Numpad2),
    99 => Some(Keycode::Numpad3),
    100 => Some(Keycode::Numpad4),
    101 => Some(Keycode::Numpad5),
    102 => Some(Keycode::Numpad6),
    103 => Some(Keycode::Numpad7),
    104 => Some(Keycode::Numpad8),
    105 => Some(Keycode::Numpad9),
    106 => Some(Keycode::NumpadMultiply),
    107 => Some(Keycode::NumpadAdd),
    109 => Some(Keycode::NumpadSubtract),
    110 => Some(Keycode::NumpadDecimal),
    111 => Some(Keycode::NumpadDivide),
    112 => Some(Keycode::F1),
    113 => Some(Keycode::F2),
    114 => Some(Keycode::F3),
    115 => Some(Keycode::F4),
    116 => Some(Keycode::F5),
    117 => Some(Keycode::F6),
    118 => Some(Keycode::F7),
    119 => Some(Keycode::F8),
    120 => Some(Keycode::F9),
    121 => Some(Keycode::F10),
    122 => Some(Keycode::F11),
    123 => Some(Keycode::F12),
    // 144 => Some(Keycode::NumLock),
    // 145 => Some(Keycode::ScrollLock),
    186 => Some(Keycode::Semicolon),
    187 => Some(Keycode::Equal),
    188 => Some(Keycode::Comma),
    189 => Some(Keycode::Minus),
    190 => Some(Keycode::Dot),
    191 => Some(Keycode::Slash),
    // 192 => Some(Keycode::BackQuote),
    219 => Some(Keycode::LeftBracket),
    220 => Some(Keycode::BackSlash),
    221 => Some(Keycode::RightBracket),
    222 => Some(Keycode::Apostrophe),
    _ => None,
  }
}

pub fn start_keybind_watcher(win: &tauri::Window) {
  win.listen("keybinds_changed", |_payload| {
    KEYBINDS_CHANGED.store(true, std::sync::atomic::Ordering::Relaxed);
  });

  let win_thrd = win.clone();

  std::thread::spawn(move || loop {
    let keybinds = get_keybinds();
    let mut registered_combos = keybinds
      .iter()
      .map(|(action, keys)| {
        let keycodes = keys
          .iter()
          .map(|key| js_keycode_to_key(*key).unwrap())
          .collect::<Vec<Keycode>>();

        (action.clone(), KeyComboState {
          keys: keycodes,
          pressed: false,
        })
      })
      .collect::<HashMap<String, KeyComboState>>();

    loop {
      std::thread::sleep(std::time::Duration::from_millis(250));

      if KEYBINDS_CHANGED.load(std::sync::atomic::Ordering::Relaxed) {
        KEYBINDS_CHANGED.store(false, std::sync::atomic::Ordering::Relaxed);
        break;
      }

      // emit keybind_pressed event when pressed, and keybind_released when released
      for (action, combo) in registered_combos.iter_mut() {
        let mut all_pressed = true;
        let key_state = DeviceState::new().query_keymap();

        for key in &combo.keys {
          if !key_state.contains(key) {
            all_pressed = false;
            break;
          }
        }

        if all_pressed && !combo.pressed {
          win_thrd.emit("keybind_pressed", Some(action.clone())).unwrap_or_default();
          combo.pressed = true;
        } else if !all_pressed && combo.pressed {
          win_thrd.emit("keybind_released", Some(action.clone())).unwrap_or_default();
          combo.pressed = false;
        }
      }
    }
  });
}


