#[cfg(target_os = "windows")]
pub fn disable_hardware_accel_windows() {
  let existing_args = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").unwrap_or_default();
  unsafe {
    std::env::set_var(
      "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
      format!("{existing_args} --disable-gpu"),
    );
  };
}
