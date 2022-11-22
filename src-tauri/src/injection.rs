use std::{collections::HashMap, env, fs, path::PathBuf, thread, time::Duration};
use tauri::regex::Regex;

use crate::js_preprocess::eval_js_imports;

#[tauri::command]
pub async fn get_injection_js(theme_js: &str) -> Result<String, ()> {
  let theme_rxg = Regex::new(r"/\* __THEMES__ \*/").unwrap();
  let injection_js = match fs::read_to_string(PathBuf::from("injection/injection_min.js")) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read injection JS in local dir: {}", e);
      println!("Checking usr/lib");

      // This is where the .deb installer throws it.
      match fs::read_to_string(PathBuf::from("/usr/lib/dorion/injection/injection_min.js")) {
        Ok(f) => f,
        Err(e) => {
          println!("Failed to read injection JS: {}", e);

          String::new()
        }
      }
    }
  };
  let rewritten_all = theme_rxg
    .replace_all(injection_js.as_str(), theme_js)
    .to_string();

  Ok(rewritten_all)
}

pub fn preinject(window: &tauri::Window) {
  let injection_js = match fs::read_to_string(PathBuf::from("injection/preinject_min.js")) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read preinject JS in local dir: {}", e);
      println!("Checking usr/lib");

      // This is where the .deb installer throws it.
      match fs::read_to_string(PathBuf::from("/usr/lib/dorion/injection/injection_min.js")) {
        Ok(f) => f,
        Err(e) => {
          println!("Failed to read preinject JS: {}", e);

          String::new()
        }
      }
    }
  };

  window.eval(injection_js.as_str()).unwrap_or(())
}

#[tauri::command]
pub fn load_injection_js(
  window: tauri::Window,
  imports: Vec<String>,
  contents: String,
  plugins: HashMap<String, String>,
) {
  eval_js_imports(&window, imports);
  window.eval(contents.as_str()).unwrap_or(());

  periodic_injection_check(window, contents, plugins);
}

#[tauri::command]
pub fn is_injected() {
  env::set_var("TAURI_INJECTED", "1");
}

fn periodic_injection_check(
  window: tauri::Window,
  injection_code: String,
  plugins: HashMap<String, String>,
) {
  std::thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_secs(1));

      let is_injected = env::var("TAURI_INJECTED").unwrap_or_else(|_| "0".to_string());

      if is_injected.eq("1") {
        // After running our injection code, we can iterate through the plugins and load them as well
        for (name, script) in &plugins {
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

      // Check if window.dorion exists
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
