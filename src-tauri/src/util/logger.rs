use chrono::Local;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::Write;
use std::ptr::addr_of_mut;

static mut LOG_FILE: Option<File> = None;

pub fn init(with_file: bool) {
  if with_file {
    let path = crate::util::paths::log_file_path();

    fs::create_dir_all(path.parent().unwrap_or(&path)).unwrap_or_default();

    let file = File::create(path).unwrap();

    unsafe {
      LOG_FILE = Some(file);
    }
  }
}

pub fn log(s: impl AsRef<str> + Display) {
  println!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), s);

  unsafe {
    let file = addr_of_mut!(LOG_FILE);
    let file = match file.as_mut() {
      Some(file) => file,
      None => return,
    };

    if let Some(f) = file {
      f.write_all(format!("[{}] {}\n", Local::now().format("%Y-%m-%d %H:%M:%S"), s).as_bytes())
        .unwrap()
    }
  }
}

#[macro_export]
macro_rules! log {
  ($($arg:tt)*) => {
    $crate::log(format!($($arg)*))
  };
}
