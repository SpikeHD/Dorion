//! A prebuilt WebView2 target avoids creating a controller during `NewWindowRequested`.

use std::sync::{
  Mutex,
  atomic::{AtomicU64, Ordering},
};
use std::time::Duration;

use tauri::webview::NewWindowFeatures;
use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Environment;

use crate::log;

static POPOUT_COUNTER: AtomicU64 = AtomicU64::new(0);

struct UiEnv(ICoreWebView2Environment);
unsafe impl Send for UiEnv {}

enum State {
  Empty,
  Queued,
  Building,
  Standby(WebviewWindow),
  Active(String),
}

struct PopoutState(Mutex<State>);

pub fn manage(app: &AppHandle) {
  app.manage(PopoutState(Mutex::new(State::Empty)));
}

fn get_environment(main: &WebviewWindow) -> Option<ICoreWebView2Environment> {
  use std::sync::mpsc;
  use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_2;
  use windows::core::Interface;

  let (tx, rx) = mpsc::sync_channel(1);
  if main
    .with_webview(move |webview| {
      let environment = unsafe {
        webview
          .controller()
          .CoreWebView2()
          .ok()
          .and_then(|webview| webview.cast::<ICoreWebView2_2>().ok())
          .and_then(|webview| webview.Environment().ok())
      };
      let _ = tx.send(environment.map(UiEnv));
    })
    .is_err()
  {
    return None;
  }
  rx.recv().ok().flatten().map(|UiEnv(env)| env)
}

fn build(app: &AppHandle, environment: ICoreWebView2Environment) -> Option<WebviewWindow> {
  let label = format!("popout-{}", POPOUT_COUNTER.fetch_add(1, Ordering::Relaxed));
  match WebviewWindowBuilder::new(
    app,
    &label,
    tauri::WebviewUrl::External("about:blank".parse().unwrap()),
  )
  .title("Popout")
  .visible(false)
  .with_environment(environment)
  .build()
  {
    Ok(window) => {
      log!("Popout standby: primed {label}");
      Some(window)
    }
    Err(error) => {
      log!("Popout standby: build failed: {error:?}");
      None
    }
  }
}

/// Runs on the UI thread, outside new-window and destruction callbacks.
pub fn refill(app: &AppHandle) {
  let state = app.state::<PopoutState>();
  {
    let mut slot = state.0.lock().unwrap_or_else(|e| e.into_inner());
    match *slot {
      State::Empty | State::Queued => *slot = State::Building,
      _ => return,
    }
  }

  let window = app
    .get_webview_window("main")
    .and_then(|main| get_environment(&main))
    .and_then(|environment| build(app, environment));

  let mut slot = state.0.lock().unwrap_or_else(|e| e.into_inner());
  *slot = window.map_or(State::Empty, State::Standby);
}

fn schedule_refill(app: &AppHandle) {
  let state = app.state::<PopoutState>();
  {
    let mut slot = state.0.lock().unwrap_or_else(|e| e.into_inner());
    if !matches!(*slot, State::Empty) {
      return;
    }
    *slot = State::Queued;
  }

  let app = app.clone();
  std::thread::spawn(move || {
    std::thread::sleep(Duration::from_millis(150));
    let _ = app.run_on_main_thread({
      let app = app.clone();
      move || refill(&app)
    });
  });
}

pub fn on_destroyed(app: &AppHandle, label: &str) {
  let state = app.state::<PopoutState>();
  let refill = {
    let mut slot = state.0.lock().unwrap_or_else(|e| e.into_inner());
    match &*slot {
      State::Active(active) if active == label => {
        *slot = State::Empty;
        true
      }
      State::Standby(window) if window.label() == label => {
        *slot = State::Empty;
        false
      }
      _ => false,
    }
  };

  if refill {
    schedule_refill(app);
  }
}

pub fn take(app: &AppHandle, features: &NewWindowFeatures) -> Option<WebviewWindow> {
  let state = app.state::<PopoutState>();
  let window = {
    let mut slot = state.0.lock().unwrap_or_else(|e| e.into_inner());
    match std::mem::replace(&mut *slot, State::Empty) {
      State::Standby(window) => {
        *slot = State::Active(window.label().to_string());
        window
      }
      other => {
        *slot = other;
        return None;
      }
    }
  };

  if let Some(size) = features.size() {
    let _ = window.set_size(tauri::LogicalSize::new(size.width, size.height));
  }
  if let Some(position) = features.position() {
    let _ = window.set_position(tauri::LogicalPosition::new(position.x, position.y));
  }
  let _ = window.show();
  Some(window)
}
