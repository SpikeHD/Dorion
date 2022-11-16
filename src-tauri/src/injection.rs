use std::{fs, path::PathBuf, time::Duration};
use tauri::{regex::Regex};

use crate::js_preprocess::eval_js_imports;

#[tauri::command]
pub async fn get_injection_js(plugin_js: &str, theme_js: &str, origin: &str) -> Result<String, ()> {
  let plugin_rxg = Regex::new(r"/\* __PLUGINS__ \*/").unwrap();
  let theme_rxg = Regex::new(r"/\* __THEMES__ \*/").unwrap();
  let origin_rxg = Regex::new(r"/\* __ORIGIN__ \*/").unwrap();
  let injection_js = match fs::read_to_string(PathBuf::from("injection/injection_min.js")) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read injection JS in local dir: {}", e);
      println!("Checking usr/lib");

      match fs::read_to_string(PathBuf::from("/usr/lib/dorion/injection/injection_min.js")) {
        Ok(f) => f,
        Err(e) => {
          println!("Failed to read injection JS: {}", e);

          String::new()
        }
      }
    }
  };

  let rewritten_just_plugins = plugin_rxg
    .replace_all(injection_js.as_str(), plugin_js)
    .to_string();
  let rewritten_with_origin = origin_rxg
    .replace_all(rewritten_just_plugins.as_str(), origin)
    .to_string();
  let rewritten_all = theme_rxg
    .replace_all(rewritten_with_origin.as_str(), theme_js)
    .to_string();

  Ok(rewritten_all)
}

#[tauri::command]
pub fn load_injection_js(window: tauri::Window, imports: Vec<String>, contents: String) {
  eval_js_imports(&window, imports);
  window.eval(contents.as_str()).unwrap();
  
  periodic_injection_check(window, contents);
}

#[tauri::command]
pub fn is_injected() {
  std::env::set_var("TAURI_INJECTED", "1");
}

fn periodic_injection_check(window: tauri::Window, injection_code: String) {
  std::thread::spawn(move || {
    loop {
      std::thread::sleep(Duration::from_secs(2));

      let is_injected = std::env::var("TAURI_INJECTED").unwrap_or("0".to_string());

      if is_injected.eq("1") {
        break;
      }

      // Check if window.dorion exists
      window
        .eval(format!("!window.dorion && (() => {{
          // Ensure we don't fire more than we have to
          window.ipc.postMessage(JSON.stringify({{
            cmd: \"is_injected\",
            callback: 0,
            error: 0,
            inner: {{}}
          }}));
          {}
        }})()", injection_code).as_str())
        .unwrap();
    }
  });
}
