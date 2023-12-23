use crate::config::get_config;

#[cfg(target_os = "windows")]
pub fn clear_cache() {
  use crate::util::paths::profiles_dir;
  use std::fs;

  let profiles_dir = profiles_dir();
  let profile = get_config().profile.unwrap_or("default".to_string());

  // Paths from profiles will be webdata/EBWebView/Default/[Cache|Code Cache]
  let main_cache = profiles_dir
    .join(&profile)
    .join("webdata")
    .join("EBWebView")
    .join("Default")
    .join("Cache");
  let code_cache = profiles_dir
    .join(profile)
    .join("webdata")
    .join("EBWebView")
    .join("Default")
    .join("Code Cache");

  // Attempt to delete
  if main_cache.exists() {
    fs::remove_dir_all(main_cache).unwrap_or_default();
  }

  if code_cache.exists() {
    fs::remove_dir_all(code_cache).unwrap_or_default();
  }
}

#[cfg(not(target_os = "windows"))]
pub fn clear_cache() {}

pub fn maybe_clear_cache() {
  if get_config().auto_clear_cache.unwrap_or(false) {
    clear_cache();
  }
}
