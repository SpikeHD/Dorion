use std::path::PathBuf;

use tauri::{Icon};

pub fn set_notif_icon(window: tauri::Window, amount: u16) {
  let icon_num = if amount > 9 { 9 } else { amount };

  // We do not have a zero icon, set back to regular icon
  if icon_num < 1 {
    let mut icon_path = PathBuf::from("icons/icon");
    icon_path.set_extension("png");
    
    window.set_icon(Icon::File(icon_path)).unwrap_or(());
    return;
  }

  let icon_name = format!("icon_{}", icon_num);
  let mut icon_path = PathBuf::from("icons/").join(icon_name);
  icon_path.set_extension("png");

  window.set_icon(Icon::File(icon_path)).unwrap_or(());
}

#[tauri::command]
pub fn notif_count(window: tauri::Window, amount: u16) {
  println!("Setting notification count: {}", amount);
  set_notif_icon(window, amount);
}