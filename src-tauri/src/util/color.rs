use mundy::{self, AccentColor, Interest, Preferences};
use tauri::Emitter;

use crate::log;

pub fn start_os_accent_subscriber(win: &tauri::WebviewWindow) {
  let win = win.clone();
  Preferences::subscribe(Interest::AccentColor, move |prefs| {
    let accent = prefs.accent_color.0;

    if let Some(accent) = accent {
      // Convert from SRGBA to RGBA
      let r = accent.red * 255.;
      let b = accent.blue * 255.;
      let g = accent.green * 255.;
      let a = accent.alpha * 255.;

      win.emit("os_accent_update", format!("rgba({:.0}, {:.0}, {:.0}, {:.0})", r, g, b, a)).unwrap_or_else(|e| {
        log!("Error emitting os_accent_update event: {}", e);
      });
    }
  });
}
