use mundy::{self, Interest, Preferences};
use std::sync::Mutex;
use tauri::Emitter;

use crate::log;

static ACCENT_COLOR: Mutex<Option<(u8, u8, u8, u8)>> = Mutex::new(None);

pub fn set_accent_color(r: u8, g: u8, b: u8, a: u8) {
  ACCENT_COLOR.lock().unwrap().replace((r, g, b, a));
}

pub fn start_os_accent_subscriber(win: &tauri::WebviewWindow) {
  let win = win.clone();
  Preferences::subscribe(Interest::All, move |prefs| {
    let accent = prefs.accent_color.0;

    if let Some(accent) = accent {
      // Convert from SRGBA to RGBA
      let r = accent.red * 255.;
      let b = accent.blue * 255.;
      let g = accent.green * 255.;
      let a = accent.alpha * 255.;

      win
        .emit(
          "os_accent_update",
          format!("rgba({:.0}, {:.0}, {:.0}, {:.0})", r, g, b, a),
        )
        .unwrap_or_else(|e| {
          log!("Error emitting os_accent_update event: {}", e);
        });

      set_accent_color(r as u8, g as u8, b as u8, a as u8);
    }
  });
}

pub fn _get_os_accent() -> String {
  let accent = ACCENT_COLOR.lock().unwrap();

  if let Some(accent) = accent.as_ref() {
    return format!(
      "rgba({}, {}, {}, {})",
      accent.0, accent.1, accent.2, accent.3
    );
  }

  "".to_string()
}
