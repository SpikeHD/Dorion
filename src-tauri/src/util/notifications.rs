use crate::log;
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

#[tauri::command]
pub fn notif_count(window: tauri::Window, amount: i32) {
  log!("Setting notification count: {}", amount);

  #[cfg(not(target_os = "linux"))]
  unsafe {
    set_notif_icon(&window, amount);
  }

  #[cfg(target_os = "linux")]
  set_notif_icon(&window, amount);
}

#[cfg(target_os = "windows")]
pub unsafe fn set_notif_icon(window: &tauri::Window, amount: i32) {
  use include_flate::flate;
  use windows::Win32::{
    System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL},
    UI::{
      Shell::{ITaskbarList3, TaskbarList},
      WindowsAndMessaging::{CreateIconFromResourceEx, LR_DEFAULTCOLOR},
    },
  };

  // Include icons
  flate!(static ICO_SOME: [u8] from "./icons/notifications/some_48.png");
  flate!(static ICO_1: [u8] from "./icons/notifications/1_48.png");
  flate!(static ICO_2: [u8] from "./icons/notifications/2_48.png");
  flate!(static ICO_3: [u8] from "./icons/notifications/3_48.png");
  flate!(static ICO_4: [u8] from "./icons/notifications/4_48.png");
  flate!(static ICO_5: [u8] from "./icons/notifications/5_48.png");
  flate!(static ICO_6: [u8] from "./icons/notifications/6_48.png");
  flate!(static ICO_7: [u8] from "./icons/notifications/7_48.png");
  flate!(static ICO_8: [u8] from "./icons/notifications/8_48.png");
  flate!(static ICO_9: [u8] from "./icons/notifications/9_48.png");

  CoInitialize(std::ptr::null()).unwrap_or_default();

  let hwnd = window.hwnd();

  if hwnd.is_err() {
    log!("Failed to get window handle: {:?}", hwnd.err());
    return;
  }

  let hwnd = hwnd.unwrap();
  let ico = match amount {
    -1 => (ICO_SOME.as_ptr(), ICO_SOME.len()),
    1 => (ICO_1.as_ptr(), ICO_1.len()),
    2 => (ICO_2.as_ptr(), ICO_2.len()),
    3 => (ICO_3.as_ptr(), ICO_3.len()),
    4 => (ICO_4.as_ptr(), ICO_4.len()),
    5 => (ICO_5.as_ptr(), ICO_5.len()),
    6 => (ICO_6.as_ptr(), ICO_6.len()),
    7 => (ICO_7.as_ptr(), ICO_7.len()),
    8 => (ICO_8.as_ptr(), ICO_8.len()),
    9 => (ICO_9.as_ptr(), ICO_9.len()),
    // more than 9, just stay at 9
    _ => (ICO_9.as_ptr(), ICO_9.len()),
  };

  // set the icon
  let taskbar_list: Result<ITaskbarList3, windows::core::Error> = CoCreateInstance(
    // For about an hour, I was trying to use ITaskbarList3, but it turns out that the GUID is wrong. I hate Windows.
    &TaskbarList,
    None,
    CLSCTX_ALL,
  );

  // check
  if taskbar_list.is_err() {
    log(format!(
      "Failed to get taskbar list: {:?}",
      taskbar_list.err()
    ));
    return;
  }

  let taskbar_list = taskbar_list.unwrap();
  taskbar_list.HrInit().unwrap_or_default();

  let hicon = CreateIconFromResourceEx(ico.0, ico.1 as u32, true, 0x30000, 32, 32, LR_DEFAULTCOLOR);

  // Apparently things can fail with a success message, lol: https://github.com/microsoft/windows-rs/issues/2108
  if hicon.is_err() || amount == 0 {
    log!("Failed to create icon: {:?}", hicon.err());
    // create null icon
    taskbar_list
      .SetOverlayIcon(hwnd, None, None)
      .unwrap_or_default();
    return;
  }

  let hicon = hicon.unwrap();

  taskbar_list
    .SetOverlayIcon(hwnd, hicon, None)
    .unwrap_or_default();

  CoUninitialize();
}

// https://github.com/tauri-apps/tauri/issues/4489#issuecomment-1170050529
#[cfg(target_os = "macos")]
pub unsafe fn set_notif_icon(_window: &tauri::Window, amount: i32) {
  use cocoa::{appkit::NSApp, base::nil, foundation::NSString};
  use objc::{msg_send, sel, sel_impl};

  let label = if amount <= 0 {
    nil
  } else {
    NSString::alloc(nil).init_str(&format!("{}", amount))
  };
  let dock_tile: cocoa::base::id = msg_send![NSApp(), dockTile];
  let _: cocoa::base::id = msg_send![dock_tile, setBadgeLabel: label];
}

#[cfg(target_os = "linux")]
pub fn set_notif_icon(_window: &tauri::Window, _amount: i32) {}
