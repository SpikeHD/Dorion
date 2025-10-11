use include_flate::flate;
use regex::Regex;
use std::collections::HashMap;

use crate::{log, processors::js_preprocess::eval_js_imports};

use super::plugin::get_plugin_list;

flate!(pub static INJECTION: str from "./injection/postinject_min.js");
flate!(pub static PREINJECT: str from "./injection/preinject_min.js");

#[tauri::command]
pub async fn get_injection_js(theme_js: &str) -> Result<String, ()> {
  let theme_rxg = Regex::new(r"/\*! __THEMES__ \*/").unwrap();
  let injection_js = INJECTION.clone();
  let rewritten_all = theme_rxg
    .replace_all(injection_js.as_str(), theme_js)
    .to_string();

  Ok(rewritten_all)
}

pub fn load_plugins(win: &tauri::WebviewWindow, plugins: HashMap<String, String>) {
  let plugin_list = get_plugin_list();

  // Eval plugin imports
  for script in plugins.values() {
    let imports = crate::injection::plugin::get_js_imports(script);

    eval_js_imports(win, imports);
  }

  // Eval plugin scripts
  for (name, script) in &plugins {
    // Ignore preload plguins
    if let Some(plugin) = plugin_list.get(name)
      && plugin.preload
    {
      log!("Skipping plugin {} (is preload)", name);
      continue;
    }

    // Execute the plugin in a try/catch, so we can capture whatever error occurs
    win
      .eval(
        format!(
          "
        console.log('Executing plugin: {name}')
        try {{
          {script}
        }} catch(e) {{
          console.error(`Plugin {name} returned error: ${{e}}`)
          console.log('The plugin could still work! Just don\\'t expect it to.')
        }}"
        )
        .as_str(),
      )
      .unwrap_or(());
  }
}
