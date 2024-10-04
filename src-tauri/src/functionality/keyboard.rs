#[cfg(feature = "hotkeys")]
use device_query::keymap::Keycode;
use serde::{Deserialize, Serialize};

#[cfg(feature = "hotkeys")]
#[derive(Debug)]
pub struct KeyComboState {
  keys: Vec<Keycode>,
  pressed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeybindChangedEvent {
  keys: Vec<KeyStruct>,
  key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyStruct {
  name: String,
  code: String,
}

// this sucks
#[cfg(feature = "hotkeys")]
pub fn js_keycode_to_key(keycode: String) -> Option<Keycode> {
  // Might have to make a PR not gonna lie
  match keycode.as_str() {
    "KeyA" => Some(Keycode::A),
    "KeyB" => Some(Keycode::B),
    "KeyC" => Some(Keycode::C),
    "KeyD" => Some(Keycode::D),
    "KeyE" => Some(Keycode::E),
    "KeyF" => Some(Keycode::F),
    "KeyG" => Some(Keycode::G),
    "KeyH" => Some(Keycode::H),
    "KeyI" => Some(Keycode::I),
    "KeyJ" => Some(Keycode::J),
    "KeyK" => Some(Keycode::K),
    "KeyL" => Some(Keycode::L),
    "KeyM" => Some(Keycode::M),
    "KeyN" => Some(Keycode::N),
    "KeyO" => Some(Keycode::O),
    "KeyP" => Some(Keycode::P),
    "KeyQ" => Some(Keycode::Q),
    "KeyR" => Some(Keycode::R),
    "KeyS" => Some(Keycode::S),
    "KeyT" => Some(Keycode::T),
    "KeyU" => Some(Keycode::U),
    "KeyV" => Some(Keycode::V),
    "KeyW" => Some(Keycode::W),
    "KeyX" => Some(Keycode::X),
    "KeyY" => Some(Keycode::Y),
    "KeyZ" => Some(Keycode::Z),

    "Digit0" => Some(Keycode::Key0),
    "Digit1" => Some(Keycode::Key1),
    "Digit2" => Some(Keycode::Key2),
    "Digit3" => Some(Keycode::Key3),
    "Digit4" => Some(Keycode::Key4),
    "Digit5" => Some(Keycode::Key5),
    "Digit6" => Some(Keycode::Key6),
    "Digit7" => Some(Keycode::Key7),
    "Digit8" => Some(Keycode::Key8),
    "Digit9" => Some(Keycode::Key9),

    "F1" => Some(Keycode::F1),
    "F2" => Some(Keycode::F2),
    "F3" => Some(Keycode::F3),
    "F4" => Some(Keycode::F4),
    "F5" => Some(Keycode::F5),
    "F6" => Some(Keycode::F6),
    "F7" => Some(Keycode::F7),
    "F8" => Some(Keycode::F8),
    "F9" => Some(Keycode::F9),
    "F10" => Some(Keycode::F10),
    "F11" => Some(Keycode::F11),
    "F12" => Some(Keycode::F12),

    "Backquote" => Some(Keycode::Grave),
    "Minus" => Some(Keycode::Minus),
    "Equal" => Some(Keycode::Equal),
    "Backspace" => Some(Keycode::Backspace),
    "Tab" => Some(Keycode::Tab),
    "BracketLeft" => Some(Keycode::LeftBracket),
    "BracketRight" => Some(Keycode::RightBracket),
    "Enter" => Some(Keycode::Enter),
    "Semicolon" => Some(Keycode::Semicolon),
    "Quote" => Some(Keycode::Apostrophe),
    "Backslash" => Some(Keycode::BackSlash),
    "Comma" => Some(Keycode::Comma),
    "Period" => Some(Keycode::Dot),
    "Slash" => Some(Keycode::Slash),

    "Space" => Some(Keycode::Space),
    "CapsLock" => Some(Keycode::CapsLock),
    "Escape" => Some(Keycode::Escape),
    //"PrintScreen" => Some(Keycode::PrintScreen),
    //"ScrollLock" => Some(Keycode::ScrollLock),
    //"Pause" => Some(Keycode::Pause),
    "Insert" => Some(Keycode::Insert),
    "Home" => Some(Keycode::Home),
    "PageUp" => Some(Keycode::PageUp),
    "Delete" => Some(Keycode::Delete),
    "End" => Some(Keycode::End),
    "PageDown" => Some(Keycode::PageDown),

    "ArrowUp" => Some(Keycode::Up),
    "ArrowDown" => Some(Keycode::Down),
    "ArrowLeft" => Some(Keycode::Left),
    "ArrowRight" => Some(Keycode::Right),

    // "NumLock" => Some(Keycode::NumLock),
    "NumpadDivide" => Some(Keycode::NumpadDivide),
    "NumpadMultiply" => Some(Keycode::NumpadMultiply),
    "NumpadSubtract" => Some(Keycode::NumpadSubtract),
    "NumpadAdd" => Some(Keycode::NumpadAdd),
    "NumpadEnter" => Some(Keycode::NumpadEnter),
    "Numpad1" => Some(Keycode::Numpad1),
    "Numpad2" => Some(Keycode::Numpad2),
    "Numpad3" => Some(Keycode::Numpad3),
    "Numpad4" => Some(Keycode::Numpad4),
    "Numpad5" => Some(Keycode::Numpad5),
    "Numpad6" => Some(Keycode::Numpad6),
    "Numpad7" => Some(Keycode::Numpad7),
    "Numpad8" => Some(Keycode::Numpad8),
    "Numpad9" => Some(Keycode::Numpad9),
    "Numpad0" => Some(Keycode::Numpad0),
    "NumpadDecimal" => Some(Keycode::NumpadDecimal),

    "ShiftLeft" => Some(Keycode::LShift),
    "ShiftRight" => Some(Keycode::RShift),
    "ControlLeft" => Some(Keycode::LControl),
    "ControlRight" => Some(Keycode::RControl),
    "AltLeft" => Some(Keycode::LAlt),
    "AltRight" => Some(Keycode::RAlt),

    // "ContextMenu" => Some(Keycode::Application),
    "MetaLeft" => Some(Keycode::LMeta),
    "MetaRight" => Some(Keycode::RMeta),

    // TODO fix for PTT since it uses a slightly different system that doesn't differentiate
    "Control" => Some(Keycode::LControl),
    "Shift" => Some(Keycode::LShift),
    "Alt" => Some(Keycode::LAlt),
    "Meta" => Some(Keycode::LMeta),
    _ => None,
  }
}
