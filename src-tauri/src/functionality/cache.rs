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

// TODO: I'm not conviced this works yet, I think the window is destroyed before this stuff runs, so it's not clearing the cache
#[cfg(target_os = "macos")]
pub fn clear_cache() {
  use block2::StackBlock;
  use objc2_foundation::{NSAutoreleasePool, NSDate, NSSet, NSString};
  use objc2_web_kit::WKWebViewConfiguration;

  unsafe {
    let pool = NSAutoreleasePool::new();
    let configuration = WKWebViewConfiguration::new();
    let store = configuration.websiteDataStore();

    // Specify cache so as to not clear login stuff
    let data_types = vec![
      NSString::from_str("WKWebsiteDataTypeDiskCache"),
      NSString::from_str("WKWebsiteDataTypeMemoryCache"),
      NSString::from_str("WKWebsiteDataTypeOfflineWebApplicationCache"),
    ];

    let data_types = NSSet::from_vec(data_types);

    let date = NSDate::dateWithTimeIntervalSince1970(0.0);

    store.removeDataOfTypes_modifiedSince_completionHandler(
      &data_types,
      &date,
      &StackBlock::new(|| {}),
    );
    pool.drain();
  }
}

#[cfg(target_os = "linux")]
pub fn clear_cache() {}

pub fn maybe_clear_cache() {
  if get_config().auto_clear_cache.unwrap_or(false) {
    clear_cache();
  }
}
