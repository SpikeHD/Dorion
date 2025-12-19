import { applyExtraCSS } from './shared/ui'
import { waitForDom, waitForElmEx } from './shared/util';
import { initWindowsKeybinds } from './shared/windows_keybinds'

(async () => {
  await waitForElmEx([['>div[class*=-content]', '>div[class*=content_]']]);

  console.log('Discord is loaded!')

  // Ensure top bar exists if we want it
  if (window.__DORION_CONFIG__.use_native_titlebar)
    window.__TAURI__.core.invoke('set_decorations', { enable: true }).catch(_e => { }) // This is allowed to fail

  // Load up our extra css
  applyExtraCSS()
  initWindowsKeybinds()

  // The comment ahead is read by tauri and used to insert theme injection code

  /*! __THEMES__ */
})();
