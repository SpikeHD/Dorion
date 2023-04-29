use std::{thread, time::Duration};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode, DeviceEvents};

pub fn start_hotkey_watcher() {
  thread::spawn(move || {    
    let device_state = DeviceState::new();    
    loop {    
        let keys: Vec<Keycode> = device_state.get_keys();    

        //println!("keys: {:?}", keys);

        // pick whatever timeout works for you in here    
        thread::sleep(Duration::from_millis(1));    
    }    
});    
}