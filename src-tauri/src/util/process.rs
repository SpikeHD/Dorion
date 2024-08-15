use std::fs;
use file_lock::{FileLock, FileOptions};

use crate::log;

use super::paths::get_config_dir;

static mut LOCK: Option<FileLock> = None;

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

    return false;
  }

  // Check if we can get a lock on the file with file_lock
  let options = FileOptions::new()
    .write(true)
    .read(true)
    .create(true);
  let file = FileLock::lock(running_file, true, options);

  match file {
    Ok(file) => {
      unsafe {
        LOCK = Some(file);
      }
    }
    Err(e) => {
      log!("Error locking file: {:?}", e);
      return true
    }
  }

   false
}
