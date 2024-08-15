use std::fs;
use std::fs::File;
use fs4::fs_std::FileExt;

use crate::log;

use super::paths::get_config_dir;

static mut LOCK: Option<File> = None;

pub fn process_already_exists() -> bool {
  // We store a .running file in the config directory
  // if the process is running
  let config_dir = get_config_dir();
  let running_file = config_dir.join(".running");

  if !running_file.exists() {
    // Create the file
    match fs::File::create(running_file) {
      Ok(_) => {}
      Err(e) => {
        log!("Error creating file: {:?}", e);
      }
    }
  }

  // Check if we can get a lock on the file with file_lock
  let file = File::create(running_file);

  match file {
    Ok(f) => {
      match f.try_lock_exclusive() {
        Ok(_) => {}
        Err(e) => {
          log!("Error locking file: {:?}", e);
          return true
        }
      }

      unsafe {
        LOCK = Some(f.try_clone().expect("Error cloning file handle"));
      }
    }
    Err(e) => {
      log!("Error getting file: {:?}", e);
      return true
    }
  }

  false
}
