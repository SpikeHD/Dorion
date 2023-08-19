use std::path::PathBuf;

use tauri::{Icon, Manager};

#[cfg(not(target_os = "macos"))]
pub fn set_notif_icon(window: &tauri::Window, amount: u16) {
  let icon_num = if amount > 9 { 9 } else { amount };

  // We do not have a zero icon, set back to regular icon
  if icon_num < 1 {
    let mut icon_path = PathBuf::from("icons/icon");
    icon_path.set_extension("ico");

    window
      .set_icon(Icon::File(
        window
          .app_handle()
          .path_resolver()
          .resolve_resource(icon_path)
          .unwrap(),
      ))
      .unwrap_or(());
    return;
  }

  let icon_name = format!("icon_{}", icon_num);
  let mut icon_path = PathBuf::from("icons/").join(icon_name);
  icon_path.set_extension("ico");

  window
    .set_icon(Icon::File(
      window
        .app_handle()
        .path_resolver()
        .resolve_resource(icon_path)
        .unwrap(),
    ))
    .unwrap_or(());
}

// https://github.com/tauri-apps/tauri/issues/4489#issuecomment-1170050529
#[cfg(target_os = "macos")]
pub unsafe fn set_notif_icon(_window: &tauri::Window, amount: u16) {
  use cocoa::{appkit::NSApp, base::nil, foundation::NSString};
  use objc::{sel, sel_impl, msg_send};

  let label = if amount == 0 {
    nil
  } else {
      NSString::alloc(nil).init_str(&format!("{}", amount))
  };
  let dock_tile: cocoa::base::id = msg_send![NSApp(), dockTile];
  let _: cocoa::base::id = msg_send![dock_tile, setBadgeLabel: label];
} 

#[tauri::command]
pub fn notif_count(window: tauri::Window, amount: u16) {
  println!("Setting notification count: {}", amount);

  unsafe {
    set_notif_icon(&window, amount);
  };
}
