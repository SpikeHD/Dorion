use tauri::Manager;

use crate::injection::injection_runner;
use std::{sync::Arc, time::Duration};

// Global "is injected" var
static mut IS_READY: bool = false;

#[tauri::command]
pub fn inject_routine(win: tauri::Window) {
  let app = Arc::new(win.app_handle());
  let evt_app = app.clone();

  // If IS_READY is already true, we should set it to false since we probably just called this from the frontend
  unsafe {
    if IS_READY {
      IS_READY = false;
    }
  }

  win.once("initial_inject", move |_| unsafe {
    IS_READY = true;
    println!("JS context ready!");

    let win = evt_app.get_window("main");

    // Set window.dorion to true in the window
    if let Some(win) = win {
      win.eval("window.dorion = true").unwrap_or_default();

      injection_runner::do_injection(win);
    }
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
      let win = app.get_window("main");

      if let Some(win) = win {
        win
          .eval("!window.dorion && window.__TAURI__.event.emit('initial_inject')")
          .unwrap_or_default();
      }

      std::thread::sleep(Duration::from_millis(5));
    }
  });
}
