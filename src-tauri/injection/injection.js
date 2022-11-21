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
  const innerApp = document?.querySelector('div[class*="notDevTools"]')?.querySelector('div[class*="app-"]')
  const loading = Array.from(
    innerApp?.children || []
  ).length === 2 || !innerApp?.querySelector('div').className.includes('app')

  if (!loading && !loaded) {
    console.log('Discord is loaded!')

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

function onClientLoad() {
  loaded = true
  observer.disconnect()

  settingInserter()
  createLocalStorage()
}

/**
 * Discord wipes `window.localStorage`, so we have to recreate it in case plugins require it
 * 
 * https://github.com/SpikeHD/Dorion/issues/7#issuecomment-1320861432
 */
function createLocalStorage() {
  const iframe = document.createElement('iframe');
  document.head.append(iframe);
  const pd = Object.getOwnPropertyDescriptor(iframe.contentWindow, 'localStorage');
  iframe.remove();
  
  Object.defineProperty(window, 'localStorage', pd)
}

/**
 * Insert the settings element
 */
function settingInserter() {
  let insertedSetting = false

  observer = new MutationObserver(() => {
    // Shove a new option in settings when it's open to go back to Dorion settings
    const appSettings = document.querySelectorAll('div[aria-label="User Settings"] div[class*="header-"]')[2]
    
    if (appSettings && !insertedSetting) {
      // Yoink the next tabs styling
      const classes = appSettings.nextSibling.classList
      const dorionTab = document.createElement('div')

      dorionTab.innerHTML = 'Dorion Settings'
      dorionTab.onclick = showSettings
      dorionTab.classList = classes

      // There needs to be a small delay for some reason, or else the client just freezes up
      setTimeout(() => {
        appSettings.parentNode.insertBefore(dorionTab, appSettings.nextSibling)
      }, 100)

      insertedSetting = true
    } else if (!appSettings) {
      insertedSetting = false;
    }
  })

  observer.observe(document.querySelector('div[class*="app"]'), {
    childList: true,
    subtree: true
  });
}

/*
 * Ahead is a bunch of settings and configuration and such
 */
async function showSettings() {
  const { invoke } = window.__TAURI__
  const settingsRegion = document.querySelector('div[class*=contentRegion] div[class*=contentColumn] div')
  const settingsHTML = await invoke('get_settings')

  settingsRegion.innerHTML = settingsHTML

  const themes = await invoke('get_theme_names')
  const themeSelect = document.querySelector('#themeSelect')

  themes.forEach(theme => {
    theme = theme.replace(/"/g, '')
    const opt = document.createElement('option')

    opt.value = theme
    opt.innerHTML = theme

    themeSelect?.appendChild(opt)
  })

  prefillConfig(
    JSON.parse(await invoke('read_config_file'))
  )

  initOnchangeHandlers()
  initOnclickHandlers()
  createPluginList()
}

function prefillConfig(config) {
  const themeSelect = document.querySelector('#themeSelect')
  const zoomSelect = document.querySelector('#zoomLevel')
  const zoomPct = document.querySelector('#zoomLevelValue')
  const clientType = document.querySelector('#clientType')

  if (themeSelect) {
    themeSelect.value = config.theme
  }

  if (zoomSelect) {
    zoomSelect.value = `${Number(config.zoom) * 100}`
    if (zoomPct) zoomPct.innerHTML = `${Number(config.zoom) * 100}%`
  }

  if (clientType) {
    clientType.value = config.client_type
  }
}

/**
 * Create a bunch of onChange handlers for the different settings
 */
function initOnchangeHandlers() {
  const { invoke } = window.__TAURI__
  const themeSelect = document.querySelector('#themeSelect')
  const zoomSelect = document.querySelector('#zoomLevel')
  const clientType = document.querySelector('#clientType')

  themeSelect?.addEventListener('change', (evt) => {
    const tgt = evt.target
    setConfigValue('theme', tgt.value)
  })

  zoomSelect?.addEventListener('change', (evt) => {
    const tgt = evt.target
    const val = document.querySelector('#zoomLevelValue')

    setConfigValue('zoom', String((Number(tgt.value) / 100).toFixed(2)))

    invoke('change_zoom', {
      zoom: Number(tgt.value) / 100
    })
    
    if (val) val.innerHTML = evt.target.value + '%'
  })

  clientType?.addEventListener('change', (evt) => {
    const tgt = evt.target
    setConfigValue('client_type', tgt.value)
  })
}

/**
 * Onclick handlers, just like the onChange handlers
 */
function initOnclickHandlers() {
  const { invoke } = window.__TAURI__
  const openPlugins = document.querySelector('#openPlugins')
  const openThemes = document.querySelector('#openThemes')
  const finishBtn = document.querySelector('#finishBtn')

  if (openPlugins) {
    openPlugins.addEventListener('click', () => {
      invoke('open_plugins')
    })
  }

  if (openThemes) {
    openThemes.addEventListener('click', () => {
      invoke('open_themes')
    })
  }

  if (finishBtn) {
    finishBtn.addEventListener('click', () => {
      window.__TAURI__.process.relaunch()
    })
  }
}

/**
 * Set a value in the configuration
 * 
 * @param {String} key 
 * @param {any} val 
 */
async function setConfigValue(key, val) {
  const { invoke } = window.__TAURI__
  const cfg = JSON.parse(await invoke('read_config_file'))
  cfg[key] = val

  await invoke('write_config_file', {
    contents: JSON.stringify(cfg)
  })
}

/**
 * Hot-create the list of plugins
 */
async function createPluginList() {
  const { invoke } = window.__TAURI__
  const plugins = await invoke('get_plugin_list')
  const list = document.querySelector('.settingFullBlock')

  plugins.forEach(plugin => {
    const li = document.createElement('li')
    const nameDisplay = document.createElement('div')
    const input = document.createElement('input')

    nameDisplay.innerHTML = plugin.name
    nameDisplay.className = 'pluginName'

    input.type = 'checkbox'
    input.checked = !plugin.disabled
    input.onchange = () => {
      invoke('toggle_plugin', {
        name: plugin.name
      })
    }

    li.appendChild(nameDisplay)
    li.appendChild(input)

    list?.appendChild(li);
  })
}