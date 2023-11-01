// Ensure we don't fire more than we have to
window.__TAURI__.invoke('is_injected')

// Create URL opener which will open links in the default system browser
// TODO: Don't resort to using this yet
// window.openURL = (url) => {
//   window.ipc.postMessage(JSON.stringify({
//     cmd: 'open_url',
//     callback: 0,
//     error: 0,
//     inner: {
//       url
//     }
//   }));
// }
window.dorion = true

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
    if (!window.DorionConfig.use_native_titlebar) createTopBar()

    onClientLoad()

    // The comments ahead are read by tauri and used to insert plugin/theme injection code
    
    /* __THEMES__ */
  } else {
    console.log('Discord not loaded...')
  }
});

observer.observe(document, {
  childList: true,
  subtree: true
});

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
  const content = await window.__TAURI__.invoke('get_top_bar');

  // If the top bar failed to load, stick to the default
  if (!content) return;

  topbar.innerHTML = content

  const appMount = document.querySelector('#app-mount')

  if (!appMount) return

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
  // It's kinda stupid to have to wait but we have to make sure Shelter loaded
  setTimeout(ensurePlugins, 3000)
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

function applyExtraCSS() {
  const { invoke } = window.__TAURI__
  invoke('get_extra_css').then(css => {
    const style = document.createElement('style')
    style.innerHTML = css
    document.head.appendChild(style)
  })
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

  // eslint-disable-next-line no-undef
  const installed = shelter.plugins.installedPlugins()

  for (const name of Object.keys(installed)) {
    console.log('[Ensure Plugins] Checking if ' + name + ' is installed...')
    if (requiredPlugins[name]) {
      console.log('[Ensure Plugins] ' + name + ' is installed!')
      requiredPlugins[name].installed = true
    }
  }

  // Now iterate the plugins that are not installed, and install them
  for (const [name, plugin] of Object.entries(requiredPlugins)) {
    if (!plugin.installed) {
      // eslint-disable-next-line no-undef
      await shelter.plugins.addRemotePlugin(name, plugin.url, true)?.catch(e => console.error(e))

      // Then set it to installed
      requiredPlugins[name].installed = true
    }
  }

  const isPluginOn = (p) => installed[p]?.on

  // Finally, enable the ones that are required
  for (const [name, plugin] of Object.entries(requiredPlugins)) {
    if (plugin.installed && plugin.required && !isPluginOn(name)) {
      // eslint-disable-next-line no-undef
      await shelter.plugins.startPlugin(name)?.catch(e => console.error(e))
    }
  }
}