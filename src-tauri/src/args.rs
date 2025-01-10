use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Args {

  #[options(help = "enable safemode, which disable all additional client mods and features")]
  pub safemode: bool,

  #[options(help = "start the app in startup mode. this isn't really for users to use")]
  pub startup: bool,

  #[cfg(target_os = "windows")]
  #[options(help = "additional arguments to pass to the webview process")]
  pub webview_args: String,
}

impl Args {
  pub fn parse() -> Self {
    Args::parse_args_default_or_exit()
  }
}

pub fn is_safemode() -> bool {
  Args::parse().safemode
}

pub fn is_startup() -> bool {
  Args::parse().startup
}

pub fn should_disable_plugins() -> bool {
  Args::parse().safemode
}

#[cfg(target_os = "windows")]
pub fn get_webview_args() -> String {
  Args::parse().webview_args
}
