use device_query::{DeviceQuery, DeviceState, MouseState, Keycode, DeviceEvents};

pub fn start_hotkey_watcher() {
  let device_state = DeviceState::new();

  let _guard = device_state.on_key_down(|key| {
    println!("Keyboard key down: {:#?}", key);
  });

  let _guard = device_state.on_key_up(|key| {
    println!("Keyboard key up: {:#?}", key);
  });
}