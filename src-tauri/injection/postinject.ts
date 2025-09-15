import { applyExtraCSS } from './shared/ui'

let loaded = false

/**
 * Observer that lets us know when Discord is loaded
 */
const observer = new MutationObserver(() => {
  const content = document.querySelector('div[class*="content_"]')

  if (content && !loaded) {
    console.log('Discord is loaded!')

    loaded = true

    // Ensure top bar exists if we want it
    if (window.__DORION_CONFIG__.use_native_titlebar)
      window.__TAURI__.core.invoke('set_decorations', { enable: true }).catch(_e => {}) // This is allowed to fail

    onClientLoad()

    // The comment ahead is read by tauri and used to insert theme injection code

    /*! __THEMES__ */
  } else {
    console.log('Discord not loaded...')
  }
})

observer.observe(document, {
  childList: true,
  subtree: true,
})

/**
 * Run when the client is "loaded"
 */
function onClientLoad() {
  observer.disconnect()

  // Load up our extra css
  applyExtraCSS()
}
