use std::sync::LazyLock;

use gumdrop::Options;

#[derive(Debug, Options, Clone)]
pub struct Args {
  #[options(help = "print usage information")]
  pub help: bool,

  #[options(help = "enable safemode, which disables most additional client mods and features")]
  pub safemode: bool,

  #[options(
    help = "start Dorion and tell it that we have opened on startup. this isn't really for users to use"
  )]
  pub startup: bool,

  #[options(help = "set a proxy that Dorion will use", meta = "URL")]
  pub proxy: Option<String>,

  #[cfg(target_os = "windows")]
  #[options(
    help = "(windows only) set Dorion to fallback to it's old style of fetching external resources"
  )]
  pub legacy_fetch: bool,

  #[cfg(target_os = "windows")]
  #[options(help = "(windows only) additional arguments to pass to the webview process")]
  pub webview_args: String,
}

impl Args {
  pub fn parse() -> Self {
    Args::parse_args_default_or_exit()
  }
}

// Lazy static to hold parsed args
static PARSED_ARGS: LazyLock<Args> = LazyLock::new(|| Args::parse());

pub fn is_help() -> bool {
  // Parsing will automatically print help information
  PARSED_ARGS.help
}

pub fn is_safemode() -> bool {
  PARSED_ARGS.safemode
}

pub fn is_startup() -> bool {
  PARSED_ARGS.startup
}

pub fn get_proxy() -> Option<String> {
  PARSED_ARGS.proxy.clone()
}

#[cfg(target_os = "windows")]
pub fn is_legacy_fetch() -> bool {
  PARSED_ARGS.legacy_fetch
}

#[cfg(target_os = "windows")]
pub fn get_webview_args() -> String {
  PARSED_ARGS.webview_args.clone()
}
