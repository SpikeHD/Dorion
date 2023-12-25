#[tauri::command]
pub fn available_blurs() -> Vec<&'static str> {
  #[cfg(target_os = "windows")]
  {
    return vec![
      "none",
      "blur",
      "acrylic",
      "mica"
    ];
  }

  #[cfg(target_os = "macos")]
  {
    return vec![
      "none",
      "vibrancy"
    ];
  }

  #[cfg(target_os = "linux")]
  {
    // Sorry linux :/
    return vec![
      "none",
    ];
  }
}

#[tauri::command]
pub fn apply_effect(win: tauri::Window, effect: &str) {
  #[cfg(target_os = "windows")]
  {
    use window_vibrancy::{apply_blur, apply_acrylic, apply_mica};

    match effect {
      "blur" => apply_blur(win, Some((18, 18, 18, 125))).unwrap_or_default(),
      "acrylic" => apply_acrylic(win, Some((18, 18, 18, 125))).unwrap_or_default(),
      "mica" => apply_mica(win, None).unwrap_or_default(),
      _ => ()
    }
  }

  #[cfg(target_os = "macos")]
  {
    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

    match effect {
      "vibrancy" => apply_vibrancy(win, NSVisualEffectMaterial::HudWindow, None, None).unwrap_or_default(),
      _ => {}
    }
  }
}

#[tauri::command]
pub fn remove_effect(win: tauri::Window) {
  #[cfg(target_os = "windows")]
  {
    use window_vibrancy::{clear_blur, clear_acrylic, clear_mica};

    clear_blur(&win).unwrap_or_default();
    clear_acrylic(&win).unwrap_or_default();
    clear_mica(win).unwrap_or_default();
  }
}