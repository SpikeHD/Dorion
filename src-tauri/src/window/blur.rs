#[tauri::command]
pub fn available_blurs() -> Vec<&'static str> {
  #[cfg(target_os = "windows")]
  {
    vec!["none", "blur", "acrylic", "mica"]
  }

  #[cfg(target_os = "macos")]
  {
    vec!["none", "vibrancy"]
  }

  #[cfg(target_os = "linux")]
  {
    // Sorry linux :/
    vec!["none"]
  }
}

#[tauri::command]
pub fn apply_effect(win: tauri::WebviewWindow, effect: &str) {
  #[cfg(target_os = "windows")]
  {
    use window_vibrancy::{apply_acrylic, apply_blur, apply_mica};

    match effect {
      "blur" => apply_blur(win, Some((18, 18, 18, 125))).unwrap_or_default(),
      "acrylic" => apply_acrylic(win, Some((18, 18, 18, 125))).unwrap_or_default(),
      "mica" => apply_mica(win, None).unwrap_or_default(),
      _ => (),
    }
  }

  #[cfg(target_os = "macos")]
  {
    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

    #[allow(clippy::single_match)]
    match effect {
      "vibrancy" => {
        apply_vibrancy(win, NSVisualEffectMaterial::HudWindow, None, None).unwrap_or_default()
      }
      _ => {}
    }
  }

  #[cfg(target_os = "linux")]
  {
    // Sorry linux :/
    let _ = win;
    let _ = effect;
  }
}

// Might use this one day, today is not that day
// #[tauri::command]
// pub fn remove_effect(win: tauri::WebviewWindow) {
//   #[cfg(target_os = "windows")]
//   {
//     use window_vibrancy::{clear_acrylic, clear_blur, clear_mica};

//     clear_blur(&win).unwrap_or_default();
//     clear_acrylic(&win).unwrap_or_default();
//     clear_mica(win).unwrap_or_default();
//   }
// }
