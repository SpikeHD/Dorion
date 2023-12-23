use std::fs;

use async_recursion::async_recursion;
use tauri::regex::Regex;

use crate::config::get_config;
use crate::util::logger::log;
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

#[tauri::command]
#[async_recursion]
pub async fn localize_imports(win: tauri::Window, css: String, name: String) -> String {
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
      log(format!("Using cached CSS for {}", name));

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
      log(format!("Getting: {}", &url));

      let response = match reqwest::blocking::get(format!("https://{}", &url)) {
        Ok(r) => r,
        Err(e) => {
          log(format!("Request failed: {}", e));
          log(format!("URL: {}", &url));

          return Some((full_import.to_owned(), String::new()));
        }
      };

      let status = response.status();

      if status != 200 {
        log(format!("Request failed: {}", status));
        log(format!("URL: {}", &url));

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
        log(format!("Error joining thread: {:?}", e));
        continue;
      }
    };

    log("Joining...".to_string());

    if result.is_none() {
      continue;
    }

    let (url, processed) = result.unwrap();

    println!(
      "Replacing URL: {} with CSS that is {} characters long",
      url,
      processed.len()
    );

    new_css = new_css.replace(url.as_str(), processed.as_str());
  }

  // If any of this css still contains imports, we need to re-process it
  if reg.is_match(new_css.as_str()) {
    log("Re-processing CSS imports...".to_string());
    new_css = localize_imports(win.clone(), new_css, name.clone()).await;
  }

  win
    .emit(
      "loading_log",
      format!("Finished processing {} CSS imports", seen_urls.len()),
    )
    .unwrap_or_default();

  // Now localize images to base64 data representations
  new_css = localize_images(win.clone(), new_css).await;
  new_css = localize_fonts(win.clone(), new_css).await;

  // If we need to cache css, do that
  if get_config().cache_css.unwrap_or(true) {
    let cache_path = get_theme_dir().join("cache");
    let cache_file = cache_path.join(format!("{}_cache.css", name));

    fs::write(cache_file, new_css.clone()).expect("Failed to write cache file!");
  }

  new_css
}

pub async fn localize_images(win: tauri::Window, css: String) -> String {
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
      || url.contains("fonts.gstatic.com")
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
      log(format!("Getting: {}", &url));

      let response = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(e) => {
          log(format!("Request failed: {}", e));
          log(format!("URL: {}", &url));

          win_clone
            .emit("loading_log", "An image failed to import...".to_string())
            .unwrap();

          return None;
        }
      };
      let bytes = response.bytes().unwrap();
      let b64 = base64::encode(&bytes);

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
        log(format!("Error joining thread: {:?}", e));
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

async fn localize_fonts(win: tauri::Window, css: String) -> String {
  let font_reg = Regex::new(
    r#"@font-face.{0,1}\{(?:.|\n)+?src:.{0,1}url\((?:'|"|)(http.+?)\.([a-zA-Z0-9]{0,5})(?:'|"|)\)"#,
  )
  .unwrap();
  let mut new_css = css.clone();
  let matches = font_reg.captures_iter(Box::leak(css.clone().into_boxed_str()));

  // This could be pretty computationally expensive for just a count, so I should change this sometime
  let count = font_reg
    .captures_iter(Box::leak(css.into_boxed_str()))
    .count();

  let mut tasks = Vec::new();

  // Check if the matches iter is more than 50
  // If it is, we should just skip it
  if count > 50 {
    win
      .emit(
        "loading_log",
        format!("Too many fonts to process ({}), skipping...", count),
      )
      .unwrap_or_default();
    return new_css;
  }

  for groups in matches {
    let url = groups.get(1).unwrap().as_str();
    let filetype = groups.get(2).unwrap().as_str();
    let full_url = format!("{}.{}", url, filetype);

    // CORS allows discord media
    if url.is_empty()
      || url.contains("cdn.discordapp")
      || url.contains("discord.com/assets")
      || url.contains("fonts.gstatic.com")
    {
      continue;
    }

    let win_clone = win.clone(); // Clone the Window handle for use in the async block

    tasks.push(std::thread::spawn(move || {
      log(format!("Getting: {}", &full_url));

      let response = match reqwest::blocking::get(&full_url) {
        Ok(r) => r,
        Err(e) => {
          log(format!("Request failed: {}", e));
          log(format!("URL: {}", &full_url));

          win_clone
            .emit("loading_log", "A font failed to import...".to_string())
            .unwrap_or_default();

          return None;
        }
      };
      let bytes = response.bytes().unwrap();
      let b64 = base64::encode(&bytes);

      win_clone
        .emit("loading_log", format!("Processed font import: {}", &url))
        .unwrap_or_default();

      Some((
        full_url.to_owned(),
        format!("data:font/{};charset=utf-8;base64,{}", filetype, b64),
      ))
    }));
  }

  for task in tasks {
    let result = match task.join() {
      Ok(r) => r,
      Err(e) => {
        log(format!("Error joining thread: {:?}", e));
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
