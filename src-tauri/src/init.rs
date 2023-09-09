use crate::injection::injection_runner;
use std::time::Duration;

// Global "is injected" var
static mut IS_READY: bool = false;

pub fn inject_routine(win: tauri::Window) {
  let win_cln = win.clone();

  win.once("initial_inject", move |_| unsafe {
    IS_READY = true;
    println!("JS context ready!");
    injection_runner::do_injection(win_cln);
  });

  std::thread::spawn(move || {
    loop {
      unsafe {
        if IS_READY {
          break;
        }
      }

      println!("JS context not ready...");

      // Send javascript that sends the "initial_inject" event
      //
      // If it succeeds, that means the web context is ready
      win
        .eval("window.__TAURI__.event.emit('initial_inject')")
        .unwrap();

      #[cfg(target_os = "macos")]
      std::thread::sleep(Duration::from_millis(10));

      #[cfg(not(target_os = "macos"))]
      std::thread::sleep(Duration::from_millis(100));
    }
  });
}
