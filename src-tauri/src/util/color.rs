#[cfg(target_os = "windows")]
pub fn get_os_accent() -> String {
  use windows::Win32::{Foundation, Graphics};

  let mut colorization: u32 = 0;
  let mut transparency = Foundation::BOOL(0);

  unsafe {
    Graphics::Dwm::DwmGetColorizationColor(&mut colorization, &mut transparency)
      .unwrap_or_default();
  }

  // This returns in AARRGGBB format, so we need to convert to RRGGBB
  format!("#{:06X}", colorization & 0xFFFFFF)
}

#[cfg(target_os = "macos")]
pub fn get_os_accent() -> String {
  use objc2_foundation::{NSString, NSUserDefaults};

  // From https://github.com/tauri-apps/tao/pull/589
  unsafe {
    let key = NSString::from_str("AppleAccentColor");
    let defaults = NSUserDefaults::standardUserDefaults();
    let color = defaults.objectForKey(&key);

    if color.is_none() {
      return "#000000".to_string();
    }

    let color_int: isize = defaults.integerForKey(&key);
    let result_str = match color_int {
      // These are NOT in the order they appear in the settings because why would they be???
      // I love Apple
      // Also if anyone knows how to get these programmatically, PRs are open
      -1 => "#8c8b8c",
      0 => "#62ba46",
      1 => "#f7821b",
      2 => "#ffc600",
      3 => "#60ba46",
      4 => "#007aff",
      5 => "#a550a7",
      6 => "#f74f9f",
      _ => "#8c8b8c",
    };

    result_str.to_string()
  }
}

#[cfg(target_os = "linux")]
pub fn get_os_accent() -> String {
  // TODO
  String::from("#000000")
}
