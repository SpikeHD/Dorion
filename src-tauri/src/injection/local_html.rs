use include_flate::flate;

use crate::util::color::get_os_accent;

flate!(static TOP_BAR: str from "./html/top.html");
flate!(static SPLASH: str from "./html/index.html");
flate!(static EXTRA_CSS: str from "./html/extra.css");

#[tauri::command]
pub fn get_index() -> String {
  SPLASH.to_string()
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn get_top_bar() -> String {
  TOP_BAR.to_string()
}

// Top bar is broken for MacOS currently
#[cfg(target_os = "macos")]
#[tauri::command]
pub fn get_top_bar() -> String {
  String::new()
}

#[tauri::command]
pub fn get_extra_css() -> String {
  let os_accent = get_os_accent();
  let css = EXTRA_CSS.to_string();
  let css = css.replace("\"{{os_accent}}\"", &os_accent);

  css
}
