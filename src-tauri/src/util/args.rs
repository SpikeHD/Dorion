pub fn is_safemode() -> bool {
  std::env::args().any(|arg| arg == "--safemode")
}

pub fn is_startup() -> bool {
  std::env::args().any(|arg| arg == "--startup")
}