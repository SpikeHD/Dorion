use include_flate::flate;
use std::{
  collections::HashMap,
  sync::atomic::{AtomicBool, Ordering},
};
use tauri::regex::Regex;

use crate::{log, processors::js_preprocess::eval_js_imports};

use super::plugin::get_plugin_list;

static TAURI_INJECTED: AtomicBool = AtomicBool::new(false);

flate!(pub static INJECTION: str from "./injection/postinject_min.js");
flate!(pub static PREINJECT: str from "./injection/preinject_min.js");

#[tauri::command]
pub fn is_injected() {
  TAURI_INJECTED.store(true, Ordering::Relaxed);
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
      log!("Skipping plugin {} (is preload)", name);
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
  TAURI_INJECTED.store(false, Ordering::Relaxed);

  // Eval contents
  window.eval(contents.as_str()).unwrap_or(());

  load_plugins(&window, plugins);

  is_injected();
}
