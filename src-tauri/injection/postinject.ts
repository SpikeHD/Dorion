import { applyExtraCSS } from './shared/ui'
import { initWindowsKeybinds } from './shared/windows_keybinds'

(async () => {
  console.log('Discord is loaded!')

  // Ensure top bar exists if we want it
  if (window.__DORION_CONFIG__.use_native_titlebar)
    window.__TAURI__.core.invoke('set_decorations', { enable: true }).catch(_e => { }) // This is allowed to fail

  initWindowsKeybinds()
  // Load up our extra css
  applyExtraCSS()
  if (document.documentElement.dataset.dorionPlatform === "macos") {
    const setupDragRegion = () => {
      // Find title_ element at top of window (the actual titlebar, not username panel)
      const titleElements = document.querySelectorAll('[class*="title_"]');
      for (const el of Array.from(titleElements)) {
        const rect = el.getBoundingClientRect();
        if (rect.top < 10 && rect.height > 20 && rect.height < 50) {
          // Found the titlebar - set drag attribute on it and all non-interactive children
          el.setAttribute("data-tauri-drag-region", "");
          el.querySelectorAll("*").forEach((child) => {
            if (
              !child.matches('button, a, input, [role="button"], svg, path')
            ) {
              child.setAttribute("data-tauri-drag-region", "");
            }
          });
          return true;
        }
      }
      return false;
    };

    // Retry until Discord UI loads
    const interval = setInterval(() => {
      if (setupDragRegion()) clearInterval(interval);
    }, 100);
  };


  // The comment ahead is read by tauri and used to insert theme injection code

  /*! __THEMES__ */
})()
