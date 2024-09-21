use std::fs;

use crate::log;
use crate::util::paths::get_theme_dir;

#[tauri::command]
pub async fn clear_css_cache() {
  let cache_path = get_theme_dir().join("cache");

  if fs::metadata(&cache_path).is_ok() {
    let files = fs::read_dir(&cache_path).expect("Failed to read cache directory!");

    // Remove all files within
    for file in files.flatten() {
      fs::remove_file(file.path()).expect("Failed to remove file!");
    }
  }
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn localize_imports(win: tauri::WebviewWindow, css: String, name: String) -> String {
  use regex::Regex;
  use tauri::Emitter;
  
  use crate::config::get_config;

  let reg = Regex::new(r#"(?m)^@import url\((?:"|'|)(?:|.+?)\/\/(.+?)(?:"|'|)\);"#).unwrap();
  let mut seen_urls: Vec<String> = vec![];
  let mut new_css = css.clone();

  let matches = reg.captures_iter(Box::leak(css.into_boxed_str()));

  let mut tasks = Vec::new();

  // If we need to cache CSS, first check and use cache if it exists
  if get_config().cache_css.unwrap_or(true) {
    let cache_path = get_theme_dir().join("cache");

    let cache_file = cache_path.join(format!("{}_cache.css", name));

    if fs::metadata(&cache_file).is_ok() {
      log!("Using cached CSS for {}", name);

      // if reading to string succeeds, return that
      if let Ok(cached) = fs::read_to_string(cache_file) {
        return cached;
      }
    }
  }

  for groups in matches {
    let full_import = groups.get(0).unwrap().as_str();
    let url = groups.get(1).unwrap().as_str().replace(['\'', '\"'], "");

    if url.is_empty() {
      continue;
    }

    if seen_urls.contains(&url) {
      // Remove the import statement from the css
      new_css = new_css.replace(full_import, "");
      continue;
    }

    let win_clone = win.clone(); // For use within the thread

    seen_urls.push(url.clone());

    tasks.push(std::thread::spawn(move || {
      log!("Getting: {}", &url);

      let response = match reqwest::blocking::get(format!("https://{}", &url)) {
        Ok(r) => r,
        Err(e) => {
          log!("Request failed: {}", e);
          log!("URL: {}", &url);

          return Some((full_import.to_owned(), String::new()));
        }
      };

      let status = response.status();

      if status != 200 {
        log!("Request failed: {}", status);
        log!("URL: {}", &url);

        return Some((full_import.to_owned(), String::new()));
      }

      let text = response.text().expect("CSS import text failed to parse!");

      // Emit a loading log
      win_clone
        .emit(
          "loading_log",
          format!("Processed CSS import: {}", url.clone()),
        )
        .unwrap_or_default();

      Some((full_import.to_owned(), text))
    }));
  }

  for task in tasks {
    let result = match task.join() {
      Ok(r) => r,
      Err(e) => {
        log!("Error joining thread: {:?}", e);
        continue;
      }
    };

    log!("Joining (localize_imports)...");

    if result.is_none() {
      continue;
    }

    let (url, processed) = result.unwrap();

    log!(
      "Replacing URL: {} with CSS that is {} characters long",
      url,
      processed.len()
    );

    new_css = new_css.replace(url.as_str(), processed.as_str());
  }

  // If any of this css still contains imports, we need to re-process it
  if reg.is_match(new_css.as_str()) {
    log!("Re-processing CSS imports...");
    new_css = localize_imports(win.clone(), new_css, name.clone());
  }

  win
    .emit(
      "loading_log",
      format!("Finished processing {} CSS imports", seen_urls.len()),
    )
    .unwrap_or_default();

  // Now localize images to base64 data representations
  new_css = localize_images(win.clone(), new_css);

  // If we need to cache css, do that
  if get_config().cache_css.unwrap_or(true) {
    let cache_path = get_theme_dir().join("cache");

    // Ensure cache path exists
    if fs::metadata(&cache_path).is_err() {
      fs::create_dir(&cache_path).expect("Failed to create cache directory!");
    }

    let cache_file = cache_path.join(format!("{}_cache.css", name));

    fs::write(cache_file, new_css.clone()).expect("Failed to write cache file!");
  }

  new_css
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn localize_imports(_win: tauri::WebviewWindow, css: String, _name: String) -> String {
  log!("Windows no longer requires CSS imports to be localized");
  css
}

#[cfg(not(target_os = "windows"))]
pub fn localize_images(win: tauri::WebviewWindow, css: String) -> String {
  use base64::{engine::general_purpose, Engine as _};
  use regex::Regex;
  use tauri::Emitter;

  let img_reg = Regex::new(r#"url\((?:'|"|)(http.+?)(?:'|"|)\)"#).unwrap();
  let mut new_css = css.clone();
  let matches = img_reg.captures_iter(Box::leak(css.clone().into_boxed_str()));

  let mut seen_urls: Vec<String> = vec![];

  // This could be pretty computationally expensive for just a count, so I should change this sometime
  let count = img_reg
    .captures_iter(Box::leak(css.into_boxed_str()))
    .count();

  let mut tasks = Vec::new();

  // Check if the matches iter is more than 50
  // If it is, we should just skip it
  if count > 50 {
    win
      .emit(
        "loading_log",
        format!("Too many images to process ({}), skipping...", count),
      )
      .unwrap_or_default();
    return new_css;
  }

  for groups in matches {
    let url = groups.get(1).unwrap().as_str();
    let filetype = url.split('.').last().unwrap();

    // SVGs require the filetype to be svg+xml because they're special I guess
    let filetype = if filetype == "svg" {
      "svg+xml"
    } else {
      filetype
    };

    // CORS allows discord media
    if url.is_empty()
            || url.contains(".css")
            || url.contains("data:image")
            || url.contains("media.discordapp")
            || url.contains("cdn.discordapp")
            || url.contains("discord.com/assets")
            // Imgur is allowed(?)
            || url.contains("i.imgur.com")
    {
      continue;
    }

    if seen_urls.contains(&url.to_string()) {
      continue;
    }

    seen_urls.push((*url).to_string());

    // If there are more than 50 tasks, it's safe to say that there are probably too many images
    // to process, so we should just skip it
    if groups.len() > 50 {
      win
        .emit(
          "loading_log",
          format!("Too many images to process ({})", groups.len()),
        )
        .unwrap_or_default();
      break;
    }

    let win_clone = win.clone(); // Clone the Window handle for use in the async block

    tasks.push(std::thread::spawn(move || {
      log!("Getting: {}", &url);

      let response = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(e) => {
          log!("Request failed: {}", e);
          log!("URL: {}", &url);

          win_clone
            .emit("loading_log", "An image failed to import...".to_string())
            .unwrap();

          return None;
        }
      };
      let bytes = response.bytes().unwrap();
      let b64 = general_purpose::STANDARD.encode(&bytes);

      win_clone
        .emit("loading_log", format!("Processed image import: {}", &url))
        .unwrap_or_default();

      if url.is_empty() {
        return None;
      }

      Some((
        url.to_owned(),
        format!("data:image/{};base64,{}", filetype, b64),
      ))
    }));
  }

  for task in tasks {
    let result = match task.join() {
      Ok(r) => r,
      Err(e) => {
        log!("Error joining thread: {:?}", e);
        continue;
      }
    };

    if result.is_none() {
      continue;
    }

    let (url, b64) = result.unwrap();

    new_css = new_css.replace(url.as_str(), b64.as_str());
  }

  new_css
}
