use std::sync::atomic::Ordering;

use crate::{
  functionality::tray::{set_tray_icon, TrayIcon, TRAY_STATE},
  log,
};
use tauri::Manager;

#[cfg(target_os = "windows")]
use super::helpers::is_windows_7;

#[derive(serde::Deserialize, Debug)]
pub struct AdditionalData {
  guild_id: Option<String>,
  channel_id: Option<String>,
  message_id: Option<String>,
}

#[tauri::command]
pub fn send_notification(
  win: tauri::WebviewWindow,
  title: String,
  body: String,
  icon: String,
  additional_data: Option<AdditionalData>,
) {
  // Write the result of the icon
  let app = win.app_handle();
  let client = reqwest::blocking::Client::new();
  let mut res = match client.get(icon).send() {
    Ok(res) => res,
    Err(e) => {
      log!("Failed to fetch notification icon: {:?}", e);
      send_notification_internal(app, title, body, String::new(), additional_data);
      return;
    }
  };

  // Then write it to a temp file
  let mut tmp_file = std::env::temp_dir();
  tmp_file.push("dorion_notif_icon.png");

  let file = match std::fs::File::create(&tmp_file) {
    Ok(file) => file,
    Err(_) => {
      log!("Failed to create temp file for notification icon");
      send_notification_internal(app, title, body, String::new(), additional_data);
      return;
    }
  };

  // Write the file
  match std::io::copy(&mut res, &mut std::io::BufWriter::new(file)) {
    Ok(_) => {}
    Err(_) => {
      send_notification_internal(app, title, body, String::new(), additional_data);
      return;
    }
  }

  #[cfg(target_os = "windows")]
  let mut icon_path = String::new();

  // Create file:// uri
  #[cfg(not(target_os = "windows"))]
  let mut icon_path = String::from("file://");

  icon_path.push_str(&tmp_file.to_str().unwrap_or_default().replace('\\', "/"));

  send_notification_internal(app, title, body, icon_path.clone(), additional_data);
}

fn send_notification_internal(
  app: &tauri::AppHandle,
  title: String,
  body: String,
  icon_path: String,
  additional_data: Option<AdditionalData>,
) {
  #[cfg(target_os = "windows")]
  {
    use crate::config::get_config;

    if !is_windows_7() && !get_config().win7_style_notifications.unwrap_or(false) {
      send_notification_internal_windows(app, title, body, icon_path, additional_data)
    } else {
      send_notification_internal_windows7(app, title, body, icon_path, additional_data)
    }
  }

  #[cfg(not(target_os = "windows"))]
  send_notification_internal_other(app, title, body, icon_path, additional_data)
}

#[cfg(not(target_os = "windows"))]
fn send_notification_internal_other(
  _app: &tauri::AppHandle,
  title: String,
  body: String,
  _icon: String,
  _additional_data: Option<AdditionalData>,
) {
  use notify_rust::{Notification, Timeout};

  match Notification::new()
    .summary(&title)
    .body(&body)
    .icon("dorion")
    .timeout(Timeout::Milliseconds(5000))
    .show()
  {
    Ok(_) => {}
    Err(e) => log!("Failed to send notification: {:?}", e),
  };
}

#[cfg(target_os = "windows")]
fn send_notification_internal_windows(
  app: &tauri::AppHandle,
  title: String,
  body: String,
  icon: String,
  additional_data: Option<AdditionalData>,
) {
  use crate::util::url::{get_url_for_channel, get_url_for_guild, get_url_for_message};
  use crate::util::window_helpers::ultrashow;
  use std::path::Path;
  use tauri_winrt_notification::{IconCrop, Toast};

  let win = app.get_webview_window("main");

  Toast::new(&app.config().identifier)
    .icon(Path::new(&icon), IconCrop::Circular, "")
    .title(title.as_str())
    .text2(body.as_str())
    .sound(None)
    .on_activated(move |_s| {
      if let Some(win) = &win {
        ultrashow(win.clone());

        // Navigate to the guild/channel/message if provided
        if let Some(data) = &additional_data {
          if let Some(guild_id) = &data.guild_id {
            let channel_id = data.channel_id.as_deref().unwrap_or_default();
            let message_id = data.message_id.as_deref().unwrap_or_default();

            let url = if !guild_id.is_empty() && !channel_id.is_empty() && !message_id.is_empty() {
              get_url_for_message(
                guild_id.clone(),
                channel_id.to_string(),
                message_id.to_string(),
              )
            } else if !guild_id.is_empty() && !channel_id.is_empty() {
              get_url_for_channel(guild_id.clone(), channel_id.to_string())
            } else if !channel_id.is_empty() && !message_id.is_empty() && guild_id.is_empty() {
              get_url_for_channel(channel_id.clone(), message_id.clone())
            } else if !guild_id.is_empty() {
              get_url_for_guild(guild_id.clone())
            } else {
              String::new()
            };

            if !url.is_empty() {
              win.navigate(&url).unwrap_or_default();
            }
          }
        }
      }

      Ok(())
    })
    .show()
    .unwrap_or_else(|e| log!("Failed to send notification: {:?}", e));
}

#[cfg(target_os = "windows")]
fn send_notification_internal_windows7(
  app: &tauri::AppHandle,
  title: String,
  body: String,
  icon: String,
  _additional_data: Option<AdditionalData>,
) {
  use std::path::Path;
  use win7_notifications::Notification;

  let icon = tauri::image::Image::from_path(Path::new(&icon));
  let mut notification = Notification::new();

  notification
    .appname(&app.package_info().name)
    .summary(&title)
    .body(&body);

  if let Ok(icon) = icon {
    notification.icon(icon.rgba().to_vec(), icon.width(), icon.height());
  }

  notification
    .show()
    .unwrap_or_else(|e| log!("Failed to send notification: {:?}", e));
}

#[tauri::command]
pub fn notification_count(window: tauri::WebviewWindow, amount: i64) {
  log!("Setting notification count: {}", amount);

  notification_count_inner(&window, amount);

  // If the tray state is unread or default,
  if TrayIcon::from_usize(TRAY_STATE.load(Ordering::Relaxed)).is_overwrite() {
    let state = if amount == 0 { "default" } else { "unread" };
    set_tray_icon(window.app_handle().to_owned(), state.to_string());
  }
}

#[cfg(target_os = "linux")]
fn notification_count_inner(window: &tauri::WebviewWindow, amount: i64) {
  window
    .set_badge_count(if amount <= 0 { None } else { Some(amount) })
    .unwrap_or_default();
}

#[cfg(target_os = "windows")]
fn notification_count_inner(window: &tauri::WebviewWindow, amount: i64) {
  if amount == 0 {
    window.set_overlay_icon(None).unwrap_or_default();
  } else {
    use include_flate::flate;
    use tauri::image::Image;

    // Include icons
    flate!(static ICO_SOME: [u8] from "./icons/notifications/some.png");
    flate!(static ICO_1: [u8] from "./icons/notifications/1.png");
    flate!(static ICO_2: [u8] from "./icons/notifications/2.png");
    flate!(static ICO_3: [u8] from "./icons/notifications/3.png");
    flate!(static ICO_4: [u8] from "./icons/notifications/4.png");
    flate!(static ICO_5: [u8] from "./icons/notifications/5.png");
    flate!(static ICO_6: [u8] from "./icons/notifications/6.png");
    flate!(static ICO_7: [u8] from "./icons/notifications/7.png");
    flate!(static ICO_8: [u8] from "./icons/notifications/8.png");
    flate!(static ICO_9: [u8] from "./icons/notifications/9.png");

    let ico = match amount {
      -1 => ICO_SOME.as_ref(),
      1 => ICO_1.as_ref(),
      2 => ICO_2.as_ref(),
      3 => ICO_3.as_ref(),
      4 => ICO_4.as_ref(),
      5 => ICO_5.as_ref(),
      6 => ICO_6.as_ref(),
      7 => ICO_7.as_ref(),
      8 => ICO_8.as_ref(),
      9 => ICO_9.as_ref(),
      // more than 9, just stay at 9
      _ => ICO_9.as_ref(),
    };

    let converted = Image::from_bytes(ico);

    if let Ok(converted) = converted {
      window.set_overlay_icon(Some(converted)).unwrap_or_default();
    } else {
      log!("Failed to convert notification icon: {:?}", converted.err());
      window.set_overlay_icon(None).unwrap_or_default();
    }
  }
}

#[cfg(target_os = "macos")]
fn notification_count_inner(_window: &tauri::WebviewWindow, amount: i64) {
  use objc2_app_kit::NSApp;
  use objc2_foundation::{MainThreadMarker, NSString};

  let label = if amount > 0 {
    Some(NSString::from_str(&format!("{amount}")))
  } else if amount == -1 {
    Some(NSString::from_str("●"))
  } else {
    None
  };

  if let Some(thread) = MainThreadMarker::new() {
    unsafe {
      let app = NSApp(thread);
      let dock_tile = app.dockTile();
      dock_tile.setBadgeLabel(label.as_deref());
      dock_tile.display();
    }
  } else {
    log!("Failed to mark main thread!");
  }
}
