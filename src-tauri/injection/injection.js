let loaded = false

/**
 * Observer that lets us know when Discord is loaded
 */
let observer = new MutationObserver(() => {
  const innerApp = document
    ?.querySelector('div[class*="app"]')
    ?.querySelector('div[class*="app"]')
  const loading = Array.from(innerApp?.children || []).length === 2

  if (loading && !loaded) {
    console.log('Discord is loaded!')

    // Ensure top bar exists if we want it
    if (window.__DORION_CONFIG__.use_native_titlebar)
      window.__TAURI__.window.appWindow.setDecorations(true)

    // This needs to render after discord is loaded
    if (
      !window.__DORION_CONFIG__.use_native_titlebar &&
      !document.querySelector('#dorion_topbar')
    )
      _createTopBar()

    _onClientLoad()

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

/**
 * Sorta yoinked from https://github.com/uwu/shelter/blob/main/packages/shelter/src/index.ts
 */
async function _waitForApp() {
  // Ensure appMount exists
  const appMount = document.querySelector('#app-mount')

  if (!appMount) {
    setTimeout(_waitForApp, 100)
    return
  }

  while (appMount.childElementCount === 0)
    await new Promise((r) => setTimeout(r, 100))

  return appMount
}

/**
 * Functions for window controls
 */
function _close() {
  window.__TAURI__.invoke('close')
}

function _minimize() {
  window.__TAURI__.invoke('minimize')
}

function _toggleMaximize() {
  window.__TAURI__.invoke('toggle_maximize')
}

async function _createTopBar() {
  const topbar = document.createElement('div')
  const content = await window.__TAURI__
    .invoke('get_top_bar')
    .catch((e) => console.error('Error reading top bar: ', e))

  // If the top bar failed to load, stick to the default
  if (!content) return

  topbar.innerHTML = content
  topbar.id = 'dorion_topbar'

  const appMount = await _waitForApp()

  if (!appMount || document.querySelector('#dorion_topbar')) return

  appMount.prepend(topbar)

  window.__TAURI__.event.listen(
    window.__TAURI__.event.TauriEvent.WINDOW_RESIZED,
    _setMaximizeIcon
  )
  _setMaximizeIcon()

  // Set version displayed in top bar
  window.dorionVersion = await window.__TAURI__.app.getVersion()
  const versionElm = document.querySelector('#dorion_version')
  if (versionElm) versionElm.innerHTML = `Dorion - v${window.dorionVersion}`

  // Once done, remove original top bar
  window.__TAURI__.invoke('remove_top_bar')

  _initTopBarEvents()
}

/**
 * Run when the client is "loaded"
 */
function _onClientLoad() {
  observer.disconnect()

  // Notifcation watcher
  _notifGetter()

  // Assign notification count
  _applyNotificationCount()

  // Load up our extra css
  _applyExtraCSS()

  // Ensure Dorion-related plugins are installed
  // It's kinda stupid to have to wait but we have to make sure Shelter loaded fully
  _waitForApp().then(() => _ensurePlugins())
}

/**
 * Give events to the top bar buttons
 */
function _initTopBarEvents() {
  document.querySelector('#topclose').onclick = _close
  document.querySelector('#topmin').onclick = _minimize
  document.querySelector('#topmax').onclick = _toggleMaximize
}

function _applyNotificationCount() {
  const { invoke } = window.__TAURI__
  const title = document.querySelector('title')
  const notifs = title.innerHTML.match(/\((.*)\)/)

  if (!notifs) {
    invoke('notif_count', {
      amount: 0,
    })

    return
  }

  invoke('notif_count', {
    amount: Number(notifs[1]),
  })
}

function _notifGetter() {
  const notifObserver = new MutationObserver(_applyNotificationCount)

  notifObserver.observe(document.querySelector('title'), {
    subtree: true,
    childList: true,
    attributes: true,
    characterData: true,
  })
}

async function _applyExtraCSS() {
  const { invoke } = window.__TAURI__
  const css = await invoke('get_extra_css')
  const style = document.createElement('style')

  style.innerHTML = css

  // Append some background-transparenting css if blur_css is true
  if (window.__DORION_CONFIG__.blur !== 'none' && window.__DORION_CONFIG__.blur_css) {
    style.innerHTML += `
      * {
        background: transparent !important;
      }
    `
  }

  document.head.appendChild(style)
}

async function _ensurePlugins() {
  const requiredPlugins = {
    'Dorion Settings':
      'https://spikehd.github.io/shelter-plugins/dorion-settings/',
    'Always Trust': 'https://spikehd.github.io/shelter-plugins/always-trust/',
    'Dorion Notifications':
      'https://spikehd.github.io/shelter-plugins/dorion-notifications/',
    'Dorion Streamer Mode':
      'https://spikehd.github.io/shelter-plugins/dorion-streamer-mode/',
    'Dorion Updater':
      'https://spikehd.github.io/shelter-plugins/dorion-updater/',
    'Dorion PTT': 'https://spikehd.github.io/shelter-plugins/dorion-ptt/',
  }

  const promises = [
    ...Object.entries(requiredPlugins).map(async ([name, url]) => {
      const res = await fetch(`${url}/plugin.js`)
      const text = await res.text()

      // Eval
      try {
        console.log('[Ensure Plugins] Loading plugin: ', name)

        // Create a new plugin object. Simpler version of https://github.com/uwu/shelter/blob/ac74061864479ecb688ae5efc321e981cd1b54fa/packages/shelter/src/plugins.tsx#L54
        const pluginStr = `shelter=>{return ${text}}${atob('Ci8v')}`
        const fn = eval(pluginStr)
        const plugin = fn(window.shelter)

        // Run plugin.onLoad if it exists
        plugin.onLoad?.()
      } catch (e) {
        console.error(`[Ensure Plugins] Failed to load plugin ${name}: `, e)
      }
    }),
  ]

  await Promise.all(promises)
}

async function _setMaximizeIcon() {
  if (await window.__TAURI__.window.appWindow.isMaximized()) {
    document.querySelector('#topmax').classList.add('maximized')
  } else {
    document.querySelector('#topmax').classList.remove('maximized')
  }
}
