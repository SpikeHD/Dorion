use include_flate::flate;
use std::collections::HashMap;
use std::fs;
use tauri::regex::Regex;

use crate::config::get_client_mods_config;
use crate::{processors::js_preprocess::eval_js_imports, util::logger};

use super::plugin::get_plugin_list;

static mut TAURI_INJECTED: bool = false;

flate!(pub static INJECTION: str from "./injection/postinject_min.js");
flate!(pub static PREINJECT: str from "./injection/preinject_min.js");

#[tauri::command]
pub fn is_injected() {
  unsafe {
    TAURI_INJECTED = true;
  }
}

#[tauri::command]
pub async fn get_injection_js(theme_js: &str) -> Result<String, ()> {
  let theme_rxg = Regex::new(r"/\*! __THEMES__ \*/").unwrap();
  let injection_js = INJECTION.clone();
  let rewritten_all = theme_rxg
    .replace_all(injection_js.as_str(), theme_js)
    .to_string();

  Ok(rewritten_all)
}

fn load_plugins(win: &tauri::Window, plugins: HashMap<String, String>) {
  let plugin_list = get_plugin_list();

  // Eval plugin imports
  for script in plugins.values() {
    let imports = crate::injection::plugin::get_js_imports(script);

    eval_js_imports(win, imports);
  }

  // Eval plugin scripts
  for (name, script) in &plugins {
    // Ignore preload plguins
    if plugin_list.contains_key(name) {
      logger::log(format!("Skipping plugin {} (is preload)", name).as_str());
      continue;
    }

    // Scuffed logging solution.
    win
      .eval(format!("console.log('Executing plugin: {}')", name).as_str())
      .unwrap_or(());

    // Execute the plugin in a try/catch, so we can capture whatever error occurs
    win
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
}

#[tauri::command]
pub fn load_injection_js(
  window: tauri::Window,
  contents: String,
  plugins: HashMap<String, String>,
) {
  // Tauri is always not injected when we call this
  unsafe {
    TAURI_INJECTED = false;
  }

  // Eval contents
  window.eval(contents.as_str()).unwrap_or(());

  load_plugins(&window, plugins);

  is_injected();
}

pub fn get_client_mods() -> String {
  let client_mods = get_client_mods_config()
    .into_iter()
    .filter(|x| x.enabled)
    .collect::<Vec<_>>();

  let mut client_mods_js = String::new();

  for client_mod in client_mods {
    let req = reqwest::blocking::get(client_mod.script.as_str());

    let mut use_fallback = false;

    let resp = match req {
      Ok(r) => r,
      Err(e) => {
        println!(
          "Failed to read {} in resource dir, using fallback: {}",
          client_mod.name, e
        );

        if client_mod.fallback != "" {
          use_fallback = true;
        }

        // Send error instead
        return format!("console.error('Failed to load {}')", client_mod.name);
      }
    };

    if use_fallback {
      let fallback = fs::read_to_string(client_mod.fallback.as_str()).unwrap_or_default();

      client_mods_js += fallback.as_str();
    } else {
      client_mods_js += resp.text().unwrap_or_default().as_str();
    }

    client_mods_js += ";";
  }

  client_mods_js
}
