use std::sync::{Arc, Mutex};
use rsrpc::{detection::{DetectableActivity, Executable}, RPCServer};

#[derive(Clone, serde::Deserialize)]
struct Payload {
  name: String,
  exe: String,
}

pub fn start_rpc_server(win: tauri::Window) {
  let detectable = reqwest::blocking::get(
    "https://gist.githubusercontent.com/SpikeHD/209bd9b17c97f45dc5be4803c748726f/raw/ddf8ed33621933b4e3c58cf1113e1679ab9fd9b5/dorion_detectable.json",
  )
  .expect("Request for detectable.json failed")
  .text()
  .expect("Failed to get text from response");

  // This accepts both a `&str` or a `String`
  let server = Arc::new(Mutex::new(RPCServer::from_json_str(detectable).expect("Failed to start RPC server")));
  let evt_server = server.clone();

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

      arguments: None
    }]);

    detectable.name = payload.name.clone();

    evt_server.lock().unwrap().append_detectables(vec![detectable]);
  });

  server.lock().unwrap().start();

  loop {}
}

fn blank_activity() -> DetectableActivity {
  DetectableActivity {
    bot_public: None,
    bot_require_code_grant: None,
    cover_image: None,
    description: None,
    developers: None,
    executables: None,
    flags: None,
    guild_id: None,
    hook: false,
    icon: None,
    id: "null".to_string(),
    name: "".to_string(),
    publishers: vec![],
    rpc_origins: vec![],
    splash: None,
    summary: "".to_string(),
    third_party_skus: vec![],
    type_field: None,
    verify_key: "".to_string(),
    primary_sku_id: None,
    slug: None,
    aliases: vec![],
    overlay: None,
    overlay_compatibility_hook: None,
    privacy_policy_url: None,
    terms_of_service_url: None,
    eula_id: None,
    deeplink_uri: None,
    tags: vec![],
    pid: None,
    timestamp: None,
  }
}
