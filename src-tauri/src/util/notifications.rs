use crate::util::logger::log;
use tauri::{api::notification, Manager};

#[tauri::command]
pub fn send_notification(win: tauri::Window, title: String, body: String, icon: String) {
  // Write the result of the icon
  let client = reqwest::blocking::Client::new();
  let mut res = match client.get(icon).send() {
    Ok(res) => res,
    Err(_) => {
      send_notification_internal(win, title, body, String::new());
      return;
    }
  };

  // Then write it to a temp file
  let mut tmp_file = std::env::temp_dir();
  tmp_file.push("dorion_notif_icon.png");

  let file = match std::fs::File::create(&tmp_file) {
    Ok(file) => file,
    Err(_) => {
      send_notification_internal(win, title, body, String::new());
      return;
    }
  };

  // Write the file
  match std::io::copy(&mut res, &mut std::io::BufWriter::new(file)) {
    Ok(_) => {}
    Err(_) => {
      send_notification_internal(win, title, body, String::new());
      return;
    }
  }

  // Create file:// uri
  #[cfg(target_os = "windows")]
  let mut icon_path = String::from("file:///");

  #[cfg(not(target_os = "windows"))]
  let mut icon_path = String::from("file://");

  icon_path.push_str(&tmp_file.to_str().unwrap_or_default().replace('\\', "/"));

  send_notification_internal(win, title, body, icon_path);
}

fn send_notification_internal(win: tauri::Window, title: String, body: String, icon: String) {
  let app = win.app_handle();

  notification::Notification::new(&app.config().tauri.bundle.identifier)
    .title(title)
    .body(body)
    .icon(icon)
    .notify(&app)
    .unwrap_or_default();
}

#[cfg(not(target_os = "macos"))]
pub fn set_notif_icon(_window: &tauri::Window, _amount: u16) {
  // This doesn't work right now womp womp

  // let icon_num = if amount > 9 { 9 } else { amount };

  // // We do not have a zero icon, set back to regular icon
  // if icon_num < 1 {
  //   let mut icon_path = PathBuf::from("icons/icon");
  //   icon_path.set_extension("ico");

  //   window
  //     .set_icon(Icon::File(
  //       window
  //         .app_handle()
  //         .path_resolver()
  //         .resolve_resource(icon_path)
  //         .unwrap(),
  //     ))
  //     .unwrap_or(());
  //   return;
  // }

  // let icon_name = format!("icon_{}", icon_num);
  // let mut icon_path = PathBuf::from("icons/").join(icon_name);
  // icon_path.set_extension("ico");

  // window
  //   .set_icon(Icon::File(
  //     window
  //       .app_handle()
  //       .path_resolver()
  //       .resolve_resource(icon_path)
  //       .unwrap(),
  //   ))
  //   .unwrap_or(());
}

// https://github.com/tauri-apps/tauri/issues/4489#issuecomment-1170050529
#[cfg(target_os = "macos")]
pub unsafe fn set_notif_icon(_window: &tauri::Window, amount: u16) {
  use cocoa::{appkit::NSApp, base::nil, foundation::NSString};
  use objc::{msg_send, sel, sel_impl};

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
  log(format!("Setting notification count: {}", amount));

  #[cfg(target_os = "macos")]
  unsafe {
    set_notif_icon(&window, amount);
  }

  #[cfg(not(target_os = "macos"))]
  set_notif_icon(&window, amount);
}
