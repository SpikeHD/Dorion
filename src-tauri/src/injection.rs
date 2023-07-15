use std::{collections::HashMap, env, fs, thread, time::Duration, path::PathBuf};
use tauri::{regex::Regex, Manager};

use crate::{helpers::resource_folder, js_preprocess::eval_js_imports};

#[tauri::command]
pub async fn get_injection_js(theme_js: &str) -> Result<String, ()> {
  let theme_rxg = Regex::new(r"/\* __THEMES__ \*/").unwrap();
  let injection_js = match fs::read_to_string(resource_folder().join("injection/injection_min.js"))
  {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read injection JS in local dir: {}", e);

      return Ok(String::new());
    }
  };
  let rewritten_all = theme_rxg
    .replace_all(injection_js.as_str(), theme_js)
    .to_string();

  Ok(rewritten_all)
}

pub fn preinject(window: &tauri::Window) {
  let injection_js = match fs::read_to_string(resource_folder().join("injection/preinject_min.js"))
  {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read preinject JS in local dir: {}", e);
      return;
    }
  };

  match window.eval(injection_js.as_str()) {
    Ok(r) => r,
    Err(e) => {
      println!("Error evaluating preinject: {}", e)
    }
  };

  // Run vencord's preinject script
  match window.eval(
    &get_vencord_js_content(&window.app_handle())
  ) {
    Ok(r) => r,
    Err(e) => {
      println!("Error evaluating vencord preinject: {}", e)
    }
  };

  // Inject vencords css
  match window.eval(
    format!(
      "
      const ts = document.createElement('style')

      ts.textContent = `
        {}
      `

      document.head.appendChild(ts)", get_vencord_css_content(&window.app_handle())).as_str()
  ) {
    Ok(r) => r,
    Err(e) => {
      println!("Error evaluating vencord css: {}", e)
    }
  };
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

pub fn get_vencord_js_content(app: &tauri::AppHandle) -> String {
  let path = app
    .path_resolver()
    .resolve_resource(PathBuf::from("injection/browser.js"))
    .unwrap();

  match fs::read_to_string(path) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read browser.js in resource dir: {}", e);

      String::new()
    }
  }
}

pub fn get_vencord_css_content(app: &tauri::AppHandle) -> String {
  let path = app
    .path_resolver()
    .resolve_resource(PathBuf::from("injection/browser.css"))
    .unwrap();

  match fs::read_to_string(path) {
    Ok(f) => f,
    Err(e) => {
      println!("Failed to read browser.css in resource dir: {}", e);

      String::new()
    }
  }
}