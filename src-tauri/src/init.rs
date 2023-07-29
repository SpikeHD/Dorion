use std::time::Duration;
use crate::injection;

// Global "is injected" var
static mut IS_READY: bool = false;

pub fn inject_routine(win: tauri::Window) {
  win.listen("initial_inject", move |_| {
    unsafe {
      IS_READY = true;
    }
  });

  std::thread::spawn(move || {
    loop {
      let win_cln = win.clone();

      // Check if ready, if so, run injection
      unsafe {
        if IS_READY {
          println!("JS context ready!");
          injection::do_injection(win_cln);
          break;
        }
      }

      println!("JS context not ready...");

      // Send javascript that sends the "initial_inject" event
      //
      // If it succeeds, that means the web context is ready
      win_cln.eval("window.__TAURI__.event.emit('initial_inject')").unwrap();

      std::thread::sleep(Duration::from_millis(100));
    }
  });
}