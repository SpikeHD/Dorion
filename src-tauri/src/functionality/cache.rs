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
  use std::os::raw::c_char;

  extern "C" {
    // Global Obj-C variables
    static WKWebsiteDataTypeDiskCache: *const c_char;
    static WKWebsiteDataTypeMemoryCache: *const c_char;
    static WKWebsiteDataTypeOfflineWebApplicationCache: *const c_char;
  }

  use cocoa::base::id;
  use cocoa::foundation::NSAutoreleasePool;
  use objc::{class, msg_send, sel, sel_impl};

  unsafe {
    let pool = cocoa::foundation::NSAutoreleasePool::new(cocoa::base::nil);
    let configuration: id = msg_send![class!(WKWebViewConfiguration), new];
    let store: id = msg_send![configuration, websiteDataStore];

    // Specify cache so as to not clear login stuff
    let data_types = vec![
      WKWebsiteDataTypeDiskCache,
      WKWebsiteDataTypeMemoryCache,
      WKWebsiteDataTypeOfflineWebApplicationCache,
    ];

    let date: id = msg_send![class!(NSDate), dateWithTimeIntervalSince1970:0.0];

    // Define a completion handler using a closure
    let handler = |_: id| {};

    let _: id =
      msg_send![store, removeDataOfTypes:data_types modifiedSince:date completionHandler:handler];
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
