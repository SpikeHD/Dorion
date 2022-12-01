use sysinfo::{PidExt, ProcessExt, SystemExt};

pub fn process_already_exists() -> bool {
  let mut exists = false;
  let mut system = sysinfo::System::new_all();
  system.refresh_all();

  for p in system.processes_by_name("Dorion") {
    if std::process::id() != p.pid().as_u32() {
      exists = true;
      break;
    }
  }

  exists
}
