use rsrpc::{
  detection::{DetectableActivity, Executable},
  RPCServer,
};
use std::sync::{Arc, Mutex};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use window_titles::ConnectionTrait;

use crate::util::paths::custom_detectables_path;

#[derive(Clone, serde::Deserialize)]
struct Payload {
  name: String,
  exe: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Window {
  title: String,
  process_name: String,
  pid: u32,
}

#[tauri::command]
pub fn get_local_detectables() -> Vec<DetectableActivity> {
  let path = custom_detectables_path();

  // Make if doesn't exist
  if !path.exists() {
    std::fs::write(path, "[]").unwrap_or_default();
    return vec![];
  }

  let contents = std::fs::read_to_string(path).unwrap_or_default();
  let detectables: Vec<DetectableActivity> = serde_json::from_str(&contents).unwrap_or_default();

  detectables
}

pub fn append_to_local(detectables: Vec<DetectableActivity>) {
  let mut local_detectables = get_local_detectables();

  local_detectables.extend(detectables);

  let path = custom_detectables_path();

  // Write back to file
  std::fs::write(
    path,
    serde_json::to_string(&local_detectables).unwrap_or_default(),
  )
  .unwrap_or_default();
}

pub fn start_rpc_server(win: tauri::Window) {
  let detectable = reqwest::blocking::get(
    "https://gist.githubusercontent.com/SpikeHD/209bd9b17c97f45dc5be4803c748726f/raw/ddf8ed33621933b4e3c58cf1113e1679ab9fd9b5/dorion_detectable.json",
  )
  .expect("Request for detectable.json failed")
  .text()
  .expect("Failed to get text from response");

  // This accepts both a `&str` or a `String`
  let server = Arc::new(Mutex::new(
    RPCServer::from_json_str(detectable).expect("Failed to start RPC server"),
  ));
  let add_server = server.clone();
  let remove_server = server.clone();

  // When the "add_detectable" event is emitted, add the detectable to the server
  win.listen("add_detectable", move |event| {
    let payload: Payload = serde_json::from_str(event.payload().unwrap()).unwrap_or(Payload {
      name: String::from(""),
      exe: String::from(""),
    });

    if payload.name.is_empty() || payload.exe.is_empty() {
      return;
    }

    let mut detectable = blank_activity();

    // Set the executable
    detectable.executables = Some(vec![Executable {
      name: payload.exe.clone(),
      is_launcher: false,

      #[cfg(target_os = "windows")]
      os: "win32".to_string(),

      #[cfg(target_os = "linux")]
      os: "linux".to_string(),

      #[cfg(target_os = "macos")]
      os: "darwin".to_string(),

      arguments: None,
    }]);

    detectable.name = payload.name.clone();

    // Save the detectable to the local file
    append_to_local(vec![detectable.clone()]);

    add_server
      .lock()
      .unwrap()
      .append_detectables(vec![detectable]);
    add_server.lock().unwrap().scan_for_processes();
  });

  win.listen("remove_detectable", move |event| {
    let payload: Payload = serde_json::from_str(event.payload().unwrap()).unwrap_or(Payload {
      name: String::from(""),
      exe: String::from(""),
    });

    // We only care about the name
    if payload.name.is_empty() {
      return;
    }

    // Remove from rsRPC instance
    remove_server
      .lock()
      .unwrap()
      .remove_detectable_by_name(payload.name.clone());

    let local_detectables = get_local_detectables();

    // Remove the detectable from the local file
    let new_detectables: Vec<DetectableActivity> = local_detectables
      .into_iter()
      .filter(|d| d.name != payload.name)
      .collect();

    // Write back to file
    let path = custom_detectables_path();

    std::fs::write(
      path,
      serde_json::to_string(&new_detectables).unwrap_or_default(),
    )
    .unwrap_or_default();
  });

  server.lock().unwrap().start();

  // Add any local custom detectables
  server
    .lock()
    .unwrap()
    .append_detectables(get_local_detectables());

  loop {
    std::thread::sleep(std::time::Duration::from_millis(10));
  }
}

fn blank_activity() -> DetectableActivity {
  serde_json::from_str::<DetectableActivity>(
    r#"
  {
    "bot_public": true,
    "bot_require_code_grant": false,
    "description": "",
    "executables": [],
    "name": "",
    "flags": 0,
    "hook": true,
    "id": "1337",
    "summary": "",
    "type": 1
  }
  "#,
  )
  .unwrap()
}

#[tauri::command(async)]
pub fn get_windows() -> Vec<Window> {
  let conn = window_titles::Connection::new().expect("Failed to connect to window titles");
  let mut system = System::new_all();

  system.refresh_processes();

  let windows: Vec<Window> = conn
    .window_titles()
    .unwrap_or_default()
    .into_iter()
    .map(|w| {
      let proc = system.process(sysinfo::Pid::from_u32(w.pid));
      let process_name = if let Some(proc) = proc {
        proc.name().to_string()
      } else {
        format!("Unknown ({})", w.pid)
      };

      Window {
        title: w.title,
        pid: w.pid,
        process_name,
      }
    })
    .collect();

  windows
}
