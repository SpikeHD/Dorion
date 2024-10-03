use tauri::menu::{AboutMetadata, CheckMenuItemBuilder, MenuBuilder, SubmenuBuilder};

use crate::config::{get_config, set_config};

use super::tray::get_tray;

pub fn create_menubar(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
  let enable_tray_icon = get_config().tray_icon_enabled.unwrap_or(true);

  // Initial disable/enable
  let tray = get_tray(app);
  if let Some(tray) = tray {
    tray.set_visible(enable_tray_icon).unwrap_or_default();
  }

  let tray_icon_toggle = CheckMenuItemBuilder::with_id("tray_icon_toggle", "Toggle tray icon")
    .checked(enable_tray_icon)
    .build(app)?;

  let submenu = SubmenuBuilder::new(app, "App")
    .about(Some(AboutMetadata {
      name: Some("Dorion".to_string()),
      ..Default::default()
    }))
    .separator()
    .item(&tray_icon_toggle)
    .separator()
    .hide()
    .hide_others()
    .quit()
    .build()?;

  let file_submenu = SubmenuBuilder::new(app, "File")
    .select_all()
    .copy()
    .cut()
    .paste()
    .fullscreen()
    .quit()
    .build()?;

  let menu = MenuBuilder::new(app)
    .items(&[&submenu, &file_submenu])
    .build()?;

  app.set_menu(menu).unwrap_or_default();

  app.on_menu_event(move |app, event| {
    if event.id() == tray_icon_toggle.id() {
      if let Some(tray) = get_tray(app) {
        let mut config = get_config();
        let enable_tray_icon = config.tray_icon_enabled.unwrap_or(true);
        tray.set_visible(!enable_tray_icon).unwrap_or_default();

        config.tray_icon_enabled = Option::from(!enable_tray_icon);
        set_config(config);
      }
    }
  });

  Ok(())
}
