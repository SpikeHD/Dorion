use include_flate::flate;
use std::collections::HashMap;
use tauri::regex::Regex;

use crate::{processors::js_preprocess::eval_js_imports, util::logger::log};

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
      log(format!("Skipping plugin {} (is preload)", name).as_str());
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
