// Ensure we don't fire more than we have to
window.__TAURI__.invoke('is_injected')

let loaded = false

/**
 * Observer that lets us know when Discord is loaded
 */
let observer = new MutationObserver(() => {
  const innerApp = document?.querySelector('div[class*="app"]')?.querySelector('div[class*="app"]')
  const loading = Array.from(
    innerApp?.children || []
  ).length === 2

  if (loading && !loaded) {
    console.log('Discord is loaded!')

    // This needs to render after discord is loaded
    if (!window.DorionConfig.use_native_titlebar && !document.querySelector('#dorion_topbar')) createTopBar()

    onClientLoad()

    // The comments ahead are read by tauri and used to insert plugin/theme injection code
    
    /*! __THEMES__ */
  } else {
    console.log('Discord not loaded...')
  }
});

observer.observe(document, {
  childList: true,
  subtree: true
});

/**
 * Sorta yoinked from https://github.com/uwu/shelter/blob/main/packages/shelter/src/index.ts
 */
async function waitForApp() {
  // Ensure appMount exists
  const appMount = document.querySelector('#app-mount')

  if (!appMount) {
    setTimeout(waitForApp, 100)
    return
  }

  while (appMount.childElementCount === 0) await new Promise(r => setTimeout(r, 100))

  return appMount
}

/**
 * Functions for window controls
 */
function close() {
  window.__TAURI__.invoke('close')
}

function minimize() {
  window.__TAURI__.invoke('minimize')
}

function maximize() {
  window.__TAURI__.invoke('maximize')
}

async function createTopBar() {
  const topbar = document.createElement("div");
  const content = await window.__TAURI__.invoke('get_top_bar')
    .catch(e => console.error("Error reading top bar: ", e));

  // If the top bar failed to load, stick to the default
  if (!content) return;

  topbar.innerHTML = content
  topbar.id = "dorion_topbar"

  const appMount = await waitForApp()

  if (!appMount || document.querySelector('#dorion_topbar')) return

  appMount.prepend(topbar);

  // Set version displayed in top bar
  window.dorionVersion = await window.__TAURI__.app.getVersion()
  const versionElm = document.querySelector('#dorion_version')
  if (versionElm) versionElm.innerHTML = `Dorion - v${window.dorionVersion}`

  // Once done, remove original top bar
  window.__TAURI__.invoke('remove_top_bar')

  initTopBarEvents()
}

/**
 * Run when the client is "loaded"
 */
function onClientLoad() {
  observer.disconnect()

  // Notifcation watcher
  notifGetter()

  // Assign notification count
  applyNotificationCount()

  // Load up our extra css
  applyExtraCSS()

  // Ensure Dorion-related plugins are installed
  // It's kinda stupid to have to wait but we have to make sure Shelter loaded fully
  waitForApp().then(() => ensurePlugins())
}

/**
 * Give events to the top bar buttons
 */
function initTopBarEvents() {
  document.querySelector('#topclose').onclick = close
  document.querySelector('#topmin').onclick = minimize
  document.querySelector('#topmax').onclick = maximize
}

function applyNotificationCount() {
  const { invoke } = window.__TAURI__
  const title = document.querySelector('title')
  const notifs = title.innerHTML.match(/\((.*)\)/)

  if (!notifs) {
    invoke('notif_count', {
      amount: 0
    })

    return
  }

  invoke('notif_count', {
    amount: Number(notifs[1])
  })
}

function notifGetter() {
  const notifObserver = new MutationObserver(applyNotificationCount)

  notifObserver.observe(document.querySelector('title'), {
    subtree: true,
    childList: true,
    attributes: true,
    characterData: true
  })
}

async function applyExtraCSS() {
  const { invoke } = window.__TAURI__
  const css = await invoke('get_extra_css')
  const style = document.createElement('style')

  style.innerHTML = css

  document.head.appendChild(style)
}

async function ensurePlugins() {
  const requiredPlugins = {
    'shelteRPC': {
      url: 'https://spikehd.github.io/shelter-plugins/shelteRPC/',
      installed: false,
      required: false,
    },
    'Dorion Settings': {
      url: 'https://spikehd.github.io/shelter-plugins/dorion-settings/',
      installed: false,
      required: true,
    },
    'Always Trust': {
      url: 'https://spikehd.github.io/shelter-plugins/always-trust/',
      installed: false,
      required: true,
    },
    'Dorion Notifications': {
      url: 'https://spikehd.github.io/shelter-plugins/dorion-notifications/',
      installed: false,
      required: true,
    },
    'Dorion Streamer Mode': {
      url: 'https://spikehd.github.io/shelter-plugins/dorion-streamer-mode/',
      installed: false,
      required: true,
    },
    'Dorion Updater': {
      url: 'https://spikehd.github.io/shelter-plugins/dorion-updater/',
      installed: false,
      required: true,
    },
    'Inline CSS': {
      url: 'https://spikehd.github.io/shelter-plugins/inline-css/',
      installed: false,
      required: false,
    }
  }

  // Welcome to another SpikeHD "hack a thing til it works no matter how terrible the solution is". This time featuring: my lack of desire to maintain a modified Shelter fork!
  // Read from the "plugins-data" indexedDb
  const shelterDB = await new Promise(r => {
    // continuously attempt to find the "shelter" db since it may not exist yet (fisrt time opening, for example)
    const attempt = () => {
      console.log('[Ensure Plugins] Attempting to get database')
      const db = indexedDB.databases().then(dbs => dbs.find(db => db.name === 'shelter'))

      console.log(db)

      if (db) return r(db)

      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      setTimeout(() => attempt(), 1000)
    }

    attempt()
  })

  const open = await new Promise(r => {
    const req = indexedDB.open(shelterDB.name, shelterDB.version)
    req.onsuccess = () => r(req.result)

    console.log('[Ensure Plugins] Attempting to open plugins-internal at version: ', shelterDB.version)
  })

  // Continuously attempt to open plugins-internal
  const pluginStore = await new Promise(r => {
    const attempt = () => {
      console.log('[Ensure Plugins] Attempting to get plugin store')
      const store = open.transaction('plugins-internal', 'readwrite').objectStore('plugins-internal')

      if (store) return r(store)

      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      setTimeout(() => attempt(), 1000)
    }

    attempt()
  })

  console.log('[Ensure Plugins] Got plugin store! Getting installed plugins...')

  // Finally, we can get all of the keys and values of the plugins
  const installed = await new Promise(r => {
    const req = pluginStore.getAll()
    req.onsuccess = () => r(req.result)
  })

  console.log('[Ensure Plugins] Got installed plugins!')

  // Mark plugins as installed or not installed
  for (const [name, _plugin] of Object.entries(requiredPlugins)) {
    const maybeInstalled = installed.find(p => p.manifest.name === name)

    if (maybeInstalled) {
      requiredPlugins[name].installed = true
    }
  }

  console.log('[Ensure Plugins] Plugins not installed: ', Object.entries(requiredPlugins).filter(([_name, plugin]) => !plugin.installed).map(([name, _plugin]) => name))

  // Now iterate the plugins that are not installed, and install them
  for (const [name, plugin] of Object.entries(requiredPlugins)) {
    if (!plugin.installed) {
      // eslint-disable-next-line no-undef
      await shelter.plugins.addRemotePlugin(name, plugin.url, true)?.catch(e => console.error(e))

      // Then set it to installed
      requiredPlugins[name].installed = true
    }
  }

  // Wait a second before checking for loaded plugins
  await new Promise(r => setTimeout(r, 1000))

  const isPluginOn = (p) => installed.find(plugin => plugin.manifest.name === p)?.on

  // Enable the ones that are required
  for (const [name, plugin] of Object.entries(requiredPlugins)) {
    if (plugin.installed && plugin.required && !isPluginOn(name)) {
      // eslint-disable-next-line no-undef
      await shelter.plugins.startPlugin(name)?.catch(e => console.error(e))
    }
  }

  // eslint-disable-next-line no-undef
  console.log('[Ensure Plugins] Loaded plugins: ', shelter.plugins.loadedPlugins())

  // In case all of our weird stuff made shelter freak out, check loadedPlugins(). If it's undefined, load them
  // eslint-disable-next-line no-undef
  if (!shelter.plugins.loadedPlugins()) {
    console.log('[Ensure Plugins] Plugins not loaded, loading...')

    // eslint-disable-next-line no-undef
    for (const plugin in shelter.plugins.installedPlugins()) {
      // If its required, start it
      if (requiredPlugins[plugin]?.required) {
        // eslint-disable-next-line no-undef
        await shelter.plugins.startPlugin(plugin)?.catch(e => console.error(e))
      }
    }
  }
}