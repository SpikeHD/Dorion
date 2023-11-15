use include_flate::flate;
use std::{collections::HashMap, fs, thread, time::Duration};
use tauri::{regex::Regex, Manager};

use super::plugin;
use crate::{
  functionality::streamer_mode::start_streamer_mode_watcher,
  processors::js_preprocess::eval_js_imports, util::paths::get_injection_dir,
};

static mut TAURI_INJECTED: bool = false;

flate!(static INJECTION: str from "./injection/injection_min.js");
flate!(static PREINJECT: str from "./injection/preinject_min.js");
flate!(static FALLBACK_MOD: str from "./injection/browser.js");

#[tauri::command]
pub async fn get_injection_js(theme_js: &str) -> Result<String, ()> {
  let theme_rxg = Regex::new(r"/\*! __THEMES__ \*/").unwrap();
  let injection_js = INJECTION.clone();
  let rewritten_all = theme_rxg
    .replace_all(injection_js.as_str(), theme_js)
    .to_string();

  Ok(rewritten_all)
}

#[tauri::command]
pub fn do_injection(window: tauri::Window) {
  let preload_plugins = match plugin::load_plugins(Option::Some(true)) {
    Ok(p) => p,
    Err(e) => {
      println!("Error loading plugins: {}", e);
      HashMap::new()
    }
  };

  // Execute preload scripts
  for script in preload_plugins.values() {
    window.eval(script).unwrap_or(());
  }

  std::thread::spawn(move || {
    let injection_js = PREINJECT.clone();

    println!("Injecting...");

    // Exec our injection js
    match window.eval(injection_js.as_str()) {
      Ok(r) => r,
      Err(e) => {
        println!("Error evaluating preinject: {}", e)
      }
    };

    // Run Shelter's preinject script
    match window.eval(&get_client_mod_js_content(&window.app_handle())) {
      Ok(r) => r,
      Err(e) => {
        println!("Error evaluating client mod preinject: {}", e)
      }
    };
  });
}

#[tauri::command]
pub fn load_injection_js(
  window: tauri::Window,
  imports: Vec<String>,
  contents: String,
  plugins: HashMap<String, String>,
) {
  // Tauri is always not injected when we call this
  unsafe {
    TAURI_INJECTED = false;
  }

  // This is a seperate call as the JS will hang otherwise
  periodic_injection_check(window, imports, contents, plugins);
}

#[tauri::command]
pub fn is_injected() {
  unsafe {
    TAURI_INJECTED = true;
  }
}

fn periodic_injection_check(
  window: tauri::Window,
  imports: Vec<String>,
  injection_code: String,
  plugins: HashMap<String, String>,
) {
  std::thread::spawn(move || unsafe {
    loop {
      thread::sleep(Duration::from_secs(1));

      let is_injected = TAURI_INJECTED;

      if is_injected {
        // We are injected! Start the streamer mode watcher
        start_streamer_mode_watcher(window.clone());

        // First we need to eval imports
        eval_js_imports(&window, imports);

        // After running our injection code, we can iterate through the plugins and load them as well
        for (name, script) in &plugins {
          // Don't load preload plugins
          if name.contains("PRELOAD_") {
            continue;
          }

          // Scuffed logging solution.
          // TODO: make not dogshit (not that it really matters)
          window
            .eval(format!("console.log('Executing plugin: {}')", name).as_str())
            .unwrap_or(());

          // Execute the plugin in a try/catch, so we can capture whatever error occurs
          window
            .eval(
              format!(
                "
            try {{
              {}
            }} catch(e) {{
              console.error(`Plugin {} returned error: ${{e}}`)
              console.log('The plugin could still work! Just don\\'t expect it to.')
            }}
            ",
                script, name
              )
              .as_str(),
            )
            .unwrap_or(());
        }

        // No longer wait for injection
        break;
      }

      window
        .eval(
          format!(
            "(() => {{
          {}
        }})()",
            injection_code
          )
          .as_str(),
        )
        .unwrap_or(());
    }
  });
}

pub fn get_client_mod_js_content(app: &tauri::AppHandle) -> String {
  let path = get_injection_dir(Some(&app.get_window("main").unwrap())).join("browser.js");

  match fs::read_to_string(path) {
    Ok(f) => f,
    Err(e) => {
      println!(
        "Failed to read browser.js in resource dir, using fallback: {}",
        e
      );

      // Send fallback instead
      FALLBACK_MOD.clone()
    }
  }
}
