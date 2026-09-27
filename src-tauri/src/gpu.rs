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
    memory_budget_thresholds: Default::default(),
    backend_options: BackendOptions {
      gl: GlBackendOptions::default(),
      dx12: Default::default(),
      noop: Default::default(),
    },
  });

  for adapter in instance.enumerate_adapters(Backends::all()) {
    let info = adapter.get_info();

    match info.device_type {
      DeviceType::DiscreteGpu | DeviceType::IntegratedGpu | DeviceType::VirtualGpu
        if info.name.contains("NVIDIA") =>
      {
        log!("NVIDIA GPU detected, disabling DMA");
        unsafe { std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1") };
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
    );
  };
}
