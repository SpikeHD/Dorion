use system_idle_time::get_idle_time;

pub fn start_idle_watcher(win: &tauri::WebviewWindow) {
  let win = win.clone();

  std::thread::spawn(move || {
    // Every 10 seconds, check idle time.
    //
    // If it's been 10 mins or more, emit idle true. If not, emit idle false.
    loop {
      let idle_time = get_idle_time().unwrap_or_default();

      // Emitting these events like this should be fine, as they do not override forced idle.
      if idle_time >= std::time::Duration::from_secs(600) {
        win.eval("shelter.flux.dispatcher.dispatch({ type: 'IDLE', idle: true })").unwrap_or_default();
      } else {
        win.eval("shelter.flux.dispatcher.dispatch({ type: 'IDLE', idle: false })").unwrap_or_default();
      }
      std::thread::sleep(std::time::Duration::from_secs(10));
    }
  });
}
