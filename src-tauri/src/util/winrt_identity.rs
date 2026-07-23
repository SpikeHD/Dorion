#![cfg(target_os = "windows")]

use std::{
  ffi::OsStr,
  os::windows::ffi::OsStrExt,
  path::{Path, PathBuf},
};

use windows::{
  core::{GUID, Interface, PCWSTR},
  Win32::{
    Foundation::PROPERTYKEY,
    System::Com::{
      CoCreateInstance, CoInitializeEx, CoUninitialize, IPersistFile,
      StructuredStorage::{PROPVARIANT, PropVariantClear},
      CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
    },
    System::Variant::VT_LPWSTR,
    UI::Shell::{
      PropertiesSystem::IPropertyStore, SetCurrentProcessExplicitAppUserModelID, IShellLinkW,
      SHStrDupW, ShellLink,
    },
  },
};

fn prop_variant_from_string(value: &[u16]) -> Result<PROPVARIANT, String> {
  let mut prop_variant = unsafe { std::mem::zeroed::<PROPVARIANT>() };

  let duplicated = unsafe { SHStrDupW(PCWSTR(value.as_ptr())) }
    .map_err(|error| format!("SHStrDupW failed: {error}"))?;

  // This is the implementation of the header-only
  // InitPropVariantFromString helper from propvarutil.h.
  unsafe {
    let value = &mut *prop_variant.Anonymous.Anonymous;
    value.vt = VT_LPWSTR;
    value.Anonymous.pwszVal = duplicated;
  }

  Ok(prop_variant)
}

fn wide(value: &OsStr) -> Vec<u16> {
  value.encode_wide().chain(std::iter::once(0)).collect()
}

struct ComGuard;

impl Drop for ComGuard {
  fn drop(&mut self) {
    unsafe {
      CoUninitialize();
    }
  }
}

fn create_aumid_shortcut(executable: &Path, shortcut: &Path, app_id: &str) -> Result<(), String> {
  let init_result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };

  if init_result.is_err() {
    return Err(format!("CoInitializeEx failed: {init_result:?}"));
  }

  let _com_guard = ComGuard;

  let executable_w = wide(executable.as_os_str());
  let shortcut_w = wide(shortcut.as_os_str());
  let app_id_w = wide(OsStr::new(app_id));
  let description_w = wide(OsStr::new("Dorion Discord client"));

  let working_directory = executable
    .parent()
    .ok_or_else(|| "Executable has no parent directory".to_string())?;

  let working_directory_w = wide(working_directory.as_os_str());

  let shell_link: IShellLinkW =
    unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
      .map_err(|error| format!("Could not create IShellLinkW: {error}"))?;

  unsafe {
    shell_link
      .SetPath(PCWSTR(executable_w.as_ptr()))
      .map_err(|error| format!("SetPath failed: {error}"))?;

    shell_link
      .SetWorkingDirectory(PCWSTR(working_directory_w.as_ptr()))
      .map_err(|error| format!("SetWorkingDirectory failed: {error}"))?;

    shell_link
      .SetDescription(PCWSTR(description_w.as_ptr()))
      .map_err(|error| format!("SetDescription failed: {error}"))?;

    // Index zero uses the primary icon embedded in Dorion.exe.
    shell_link
      .SetIconLocation(PCWSTR(executable_w.as_ptr()), 0)
      .map_err(|error| format!("SetIconLocation failed: {error}"))?;
  }

  let property_store: IPropertyStore = shell_link
    .cast()
    .map_err(|error| format!("Could not obtain IPropertyStore: {error}"))?;

  // PKEY_AppUserModel_ID:
  // format ID 9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3,
  // property ID 5.
  let app_user_model_id_key = PROPERTYKEY {
    fmtid: GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3),
    pid: 5,
  };

  let mut app_id_value = prop_variant_from_string(&app_id_w)?;

  let set_result =
    unsafe { property_store.SetValue(&app_user_model_id_key, &app_id_value) };

  if let Err(error) = set_result {
    unsafe {
      let _ = PropVariantClear(&mut app_id_value);
    }

    return Err(format!("Setting shortcut AUMID failed: {error}"));
  }

  let commit_result = unsafe { property_store.Commit() };

  // InitPropVariantFromString allocates the string owned by the
  // PROPVARIANT. Release it after IPropertyStore has copied it.
  unsafe {
    let _ = PropVariantClear(&mut app_id_value);
  }

  commit_result
    .map_err(|error| format!("Committing shortcut properties failed: {error}"))?;

  let persist_file: IPersistFile = shell_link
    .cast()
    .map_err(|error| format!("Could not obtain IPersistFile: {error}"))?;

  unsafe {
    persist_file
      .Save(PCWSTR(shortcut_w.as_ptr()), true)
      .map_err(|error| format!("Saving shortcut failed: {error}"))?;
  }

  Ok(())
}

/// Assigns the process AUMID and creates/updates the matching
/// per-user Start Menu shortcut.
///
/// The shortcut is rewritten every launch so portable builds remain
/// valid after Dorion.exe is moved.
pub fn register(app_id: &str) -> Result<PathBuf, String> {
  // This must happen before Dorion creates its window.
  let app_id_w = wide(OsStr::new(app_id));

  unsafe { SetCurrentProcessExplicitAppUserModelID(PCWSTR(app_id_w.as_ptr())) }.map_err(|error| {
    format!("SetCurrentProcessExplicitAppUserModelID failed: {error}")
  })?;

  let executable = std::env::current_exe()
    .map_err(|error| format!("Could not obtain current executable: {error}"))?;

  let app_data =
    std::env::var_os("APPDATA").ok_or_else(|| "APPDATA is unavailable".to_string())?;

  let shortcut = PathBuf::from(app_data)
    .join("Microsoft")
    .join("Windows")
    .join("Start Menu")
    .join("Programs")
    .join("Dorion.lnk");

  let shortcut_parent = shortcut
    .parent()
    .ok_or_else(|| "Shortcut has no parent directory".to_string())?;

  std::fs::create_dir_all(shortcut_parent)
    .map_err(|error| format!("Could not create Start Menu directory: {error}"))?;

  // Use a dedicated STA thread so this does not conflict with
  // WebView2/Tauri's COM apartment.
  let thread_executable = executable.clone();
  let thread_shortcut = shortcut.clone();
  let thread_app_id = app_id.to_string();

  let result = std::thread::spawn(move || {
    create_aumid_shortcut(&thread_executable, &thread_shortcut, &thread_app_id)
  })
  .join()
  .map_err(|_| "WinRT identity registration thread panicked".to_string())?;

  result?;

  Ok(shortcut)
}
