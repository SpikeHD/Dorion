import { applyExtraCSS, createTopBar } from './shared/ui'
import { applyNotificationCount } from './shared/window'

let loaded = false

/**
 * Observer that lets us know when Discord is loaded
 */
const observer = new MutationObserver(() => {
  const innerApp = document
    ?.querySelector('div[class*="app"]')
    ?.querySelector('div[class*="app"]')
  const loading = Array.from(innerApp?.children || []).length === 2

  if (loading && !loaded) {
    console.log('Discord is loaded!')
    
    loaded = true

    // Ensure top bar exists if we want it
    if (window.__DORION_CONFIG__.use_native_titlebar)
      window.__TAURI__.core.invoke('set_decorations', { enable: true }).catch(_e => {}) // This is allowed to fail

    handleTopBar()
    onClientLoad()

    // The comments ahead are read by tauri and used to insert plugin/theme injection code

    /*! __THEMES__ */
  } else {
    console.log('Discord not loaded...')
  }
})

observer.observe(document, {
  childList: true,
  subtree: true,
})

function handleTopBar() {
  // This needs to render after discord is loaded
  if (
    // !window.__DORION_CONFIG__.use_native_titlebar &&
    !document.querySelector('#dorion_topbar')
  ) {
    window.__TAURI__.core.invoke('set_decorations', { enable: false }).catch(_e => {})
    createTopBar()
  }
}

/**
 * Run when the client is "loaded"
 */
function onClientLoad() {
  observer.disconnect()

  // Notification watcher
  notifGetter()

  // Assign notification count
  applyNotificationCount()

  // Load up our extra css
  applyExtraCSS()
}

function notifGetter() {
  const notifObserver = new MutationObserver(applyNotificationCount)

  notifObserver.observe(document.querySelector('title') as HTMLTitleElement, {
    subtree: true,
    childList: true,
    attributes: false,
    characterData: false,
  })
}
