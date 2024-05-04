#[cfg(target_os = "windows")]
pub fn get_os_accent() -> String {
  use windows::Win32::{Foundation, Graphics};

  let mut colorization: u32 = 0;
  let mut transparency = Foundation::BOOL(0);

  unsafe {
    Graphics::Dwm::DwmGetColorizationColor(&mut colorization, &mut transparency).unwrap_or_default();
  }

  // This returns in AARRGGBB format, so we need to convert to RRGGBB
  let color = format!("#{:06X}", colorization & 0xFFFFFF);

  color
}

#[cfg(target_os = "macos")]
pub fn get_os_accent() -> String {
  // TODO
  String::from("#000000")
}

#[cfg(target_os = "linux")]
pub fn get_os_accent() -> String {
  // TODO
  String::from("#000000")
}