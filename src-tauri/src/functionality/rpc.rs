use rsrpc::{
  detection::{DetectableActivity, Executable},
  RPCConfig, RPCServer,
};
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, Mutex,
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tauri::{Emitter, Listener};
use window_titles::ConnectionTrait;

use crate::{config::get_config, log, util::paths::custom_detectables_path};

// We keep track of this A) To not spam enable and B) to allow for the user to manually disable without it being re-enabled automatically
static OBS_OPEN: AtomicBool = AtomicBool::new(false);

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

pub fn start_rpc_server(win: tauri::WebviewWindow) {
  #[cfg(debug_assertions)]
  std::env::set_var("RSRPC_LOGS_ENABLED", "1");

  let detectable = reqwest::blocking::get("https://discord.com/api/v9/applications/detectable")
    .expect("Request for detectable.json failed")
    .text()
    .expect("Failed to get text from response");

  let config = get_config();
  let rpc_config = RPCConfig {
    enable_process_scanner: config.rpc_process_scanner.unwrap_or(true),
    enable_ipc_connector: config.rpc_ipc_connector.unwrap_or(true),
    enable_websocket_connector: config.rpc_websocket_connector.unwrap_or(true),
    enable_secondary_events: config.rpc_secondary_events.unwrap_or(true),
  };
  let server = Arc::new(Mutex::new(
    RPCServer::from_json_str(detectable, rpc_config).expect("Failed to start RPC server"),
  ));
  let add_server = server.clone();
  let remove_server = server.clone();

  // When the "add_detectable" event is emitted, add the detectable to the server
  win.listen("add_detectable", move |event| {
    let payload: Payload = serde_json::from_str(event.payload()).unwrap_or(Payload {
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

    detectable.name.clone_from(&payload.name);

    // Save the detectable to the local file
    append_to_local(vec![detectable.clone()]);

    add_server
      .lock()
      .unwrap()
      .append_detectables(vec![detectable]);
    add_server.lock().unwrap().scan_for_processes();
  });

  win.listen("remove_detectable", move |event| {
    let payload: Payload = serde_json::from_str(event.payload()).unwrap_or(Payload {
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

  if get_config().streamer_mode_detection.unwrap_or(false) {
    server
      .lock()
      .unwrap()
      .on_process_scan_complete(move |state| {
        log!("OBS state: {:?}", state.obs_open);

        if !OBS_OPEN.load(Ordering::Relaxed) && state.obs_open {
          OBS_OPEN.store(true, Ordering::Relaxed);
          win.emit("streamer_mode_toggle", true).unwrap_or_default();
        }

        if OBS_OPEN.load(Ordering::Relaxed) && !state.obs_open {
          OBS_OPEN.store(false, Ordering::Relaxed);
          win.emit("streamer_mode_toggle", false).unwrap_or_default();
        }
      });
  }

  server.lock().unwrap().start();

  // Add any local custom detectables
  server
    .lock()
    .unwrap()
    .append_detectables(get_local_detectables());

  loop {
    std::thread::sleep(std::time::Duration::from_secs(1));
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
  let system =
    System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));

  let windows: Vec<Window> = conn
    .window_titles()
    .unwrap_or_default()
    .into_iter()
    .map(|w| {
      let proc = system.process(sysinfo::Pid::from_u32(w.pid));
      let process_name = if let Some(proc) = proc {
        proc.name().to_string_lossy().to_string()
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
