#[cfg(target_os = "linux")]
pub fn disable_dma() {
  // Disable DMA rendering on Linux + NVIDIA systems
  // see: https://github.com/SpikeHD/Dorion/issues/237 and https://github.com/tauri-apps/tauri/issues/9304
  use crate::log;
  use wgpu::{
    BackendOptions, Backends, DeviceType, GlBackendOptions, Instance, InstanceDescriptor,
    InstanceFlags,
  };

  let instance = Instance::new(&InstanceDescriptor {
    flags: InstanceFlags::empty(),
    backends: Backends::GL | Backends::VULKAN,
    backend_options: BackendOptions {
      gl: GlBackendOptions::default(),
      dx12: Default::default(),
    },
  });

  for adapter in instance.enumerate_adapters(Backends::all()) {
    let info = adapter.get_info();

    match info.device_type {
      DeviceType::DiscreteGpu | DeviceType::IntegratedGpu | DeviceType::VirtualGpu => {
        if info.name.contains("NVIDIA") {
          log!("NVIDIA GPU detected, disabling DMA");
          std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
      }
      _ => {}
    }
  }
}

#[cfg(target_os = "windows")]
pub fn disable_hardware_accel_windows() {
  let existing_args = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").unwrap_or_default();
  unsafe {
    std::env::set_var(
      "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
      format!("{existing_args} --disable-gpu"),
    )
  };
}

#[cfg(target_os = "linux")]
pub fn disable_hardware_accel_linux(window: &tauri::WebviewWindow) {
  use crate::config::get_config;
  use crate::log;
  use webkit2gtk::{HardwareAccelerationPolicy, SettingsExt, WebViewExt};

  window
    .with_webview(move |webview| {
      let config = get_config();
      let wv = webview.inner();
      let settings = WebViewExt::settings(&wv).unwrap_or_default();

      if config.disable_hardware_accel.unwrap_or(false) {
        settings.set_hardware_acceleration_policy(HardwareAccelerationPolicy::Never);
      }
    })
    .unwrap_or_else(|_| log!("Failed to set user-agent"));
}
