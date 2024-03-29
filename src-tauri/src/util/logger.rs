use std::fmt::Display;
use std::fs::{self, File};
use std::io::Write;
use chrono::Local;

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
    if let Some(file) = &mut LOG_FILE {
      file.write_all(format!("{}\n", s).as_bytes()).unwrap()
    }
  }
}

#[macro_export]
macro_rules! log {
  ($($arg:tt)*) => {
    crate::log(format!($($arg)*))
  };
}
