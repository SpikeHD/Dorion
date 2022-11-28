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

/**
 * Run when the client is "loaded"
 */
function onClientLoad() {
  loaded = true
  observer.disconnect()

  settingInserter()
  createLocalStorage()
  notifGetter()
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

function notifGetter() {
  const { invoke } = window.__TAURI__

  const notifObserver = new MutationObserver(() => {
    const title = document.querySelector('title')
    const notifs = title.innerHTML.match(/\((.*)\)/)

    if (!notifs) {
      invoke('notfi_count', {
        amount: 0
      })

      return
    }

    invoke('notif_count', {
      amount: Number(notifs[1])
    })
  })

  notifObserver.observe(document.querySelector('title'), {
    subtree: true,
    characterData: true
  })
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

/**
 * Fill up the config page with the existing config
 * 
 * @param {Object} config 
 */
function prefillConfig(config) {
  const themeSelect = document.querySelector('#themeSelect')
  const zoomSelect = document.querySelector('#zoomLevel')
  const zoomPct = document.querySelector('#zoomLevelValue')
  const clientType = document.querySelector('#clientType')
  const systray = document.querySelector('#systray')
  const telemetry = document.querySelector('#telemetry')

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

  if (systray) {
    systray.checked = config.sys_tray
    setSlider('systray', config.sys_tray)
  }

  if (telemetry) {
    telemetry.checked = config.block_telemetry
    setSlider('telemetry', config.block_telemetry)
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
  const systray = document.querySelector('#systray')
  const telemetry = document.querySelector('#telemetry')

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

  systray?.addEventListener('change', (evt) => {
    const tgt = evt.target

    setSlider(evt.target.id, tgt.checked)
    setConfigValue('sys_tray', tgt.checked)
  })

  telemetry?.addEventListener('change', (evt) => {
    const tgt = evt.target

    setSlider(evt.target.id, tgt.checked)
    setConfigValue('block_telemetry', tgt.checked)
  })
}

/**
 * Set a sliders position based on whether its checked or not
 * 
 * @param {String} id 
 * @param {Boolean} enabled 
 * @returns 
 */
function setSlider(id, enabled) {
  const elm = document.querySelector('#' + id).parentElement
  const svg = elm.querySelector('svg')
  const svgEnabled = `
    <path fill="rgba(59, 165, 92, 1)" d="M7.89561 14.8538L6.30462 13.2629L14.3099 5.25755L15.9009 6.84854L7.89561 14.8538Z"></path>
    <path fill="rgba(59, 165, 92, 1)" d="M4.08643 11.0903L5.67742 9.49929L9.4485 13.2704L7.85751 14.8614L4.08643 11.0903Z"></path>
  `
  const svgDisabled = `
    <path fill="rgba(114, 118, 125, 1)" d="M5.13231 6.72963L6.7233 5.13864L14.855 13.2704L13.264 14.8614L5.13231 6.72963Z"></path>
    <path fill="rgba(114, 118, 125, 1)" d="M13.2704 5.13864L14.8614 6.72963L6.72963 14.8614L5.13864 13.2704L13.2704 5.13864Z"></path>
  `
  
  if (enabled) {
    svg.style.left = '12px'
    svg.querySelector('svg').innerHTML = svgEnabled
    elm.classList.add('enabled')
    elm.classList.remove('disabled')

    return
  }

  svg.style.left = '-4px'
  svg.querySelector('svg').innerHTML = svgDisabled
  elm.classList.remove('enabled')
  elm.classList.add('disabled')
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
 * Dynamically create a slider checkbox
 * 
 * @param {String} label 
 * @param {String} id 
 * @param {Function} onchange 
 * @returns 
 */
function createSlider(label, id, onchange) {
  const div = document.createElement('div')
  const htmlStr = `
  <label for="${id}">${label}</label>
  <div class="control-1fl03-">
    <div class="container-2nx-BQ" style="opacity: 1;">
      <!-- Checkbox/slider SVG -->
      <svg class="slider-32CRPX" viewBox="0 0 28 20" preserveAspectRatio="xMinYMid meet" style="left: 12px;">
        <rect fill="white" x="4" y="0" height="20" width="20" rx="10"></rect>
        <svg viewBox="0 0 20 20" fill="none">
          <path fill="rgba(59, 165, 92, 1)" d="M7.89561 14.8538L6.30462 13.2629L14.3099 5.25755L15.9009 6.84854L7.89561 14.8538Z"></path>
          <path fill="rgba(59, 165, 92, 1)" d="M4.08643 11.0903L5.67742 9.49929L9.4485 13.2704L7.85751 14.8614L4.08643 11.0903Z"></path>
        </svg>
      </svg>
      <!-- End SVG -->
      <input type="checkbox" class="input-2XRLou" id="${id}" name="${id}" />
    </div>
  </div>`

  div.innerHTML = htmlStr

  div.querySelector('input').onchange = onchange

  return div;
}

/**
 * Hot-create the list of plugins
 */
async function createPluginList() {
  const { invoke } = window.__TAURI__
  const plugins = await invoke('get_plugin_list')
  const table = document.querySelector('#plugin_table tbody')

  plugins.forEach(plugin => {
    const trow = document.createElement('tr')
    const plId = `plugin_${plugin.name}`
    const plToggle = createSlider('', `${plId}_enable`, (e) => {
      setSlider(e.target.id, e.target.checked)

      invoke('toggle_plugin', {
        name: plugin.name
      })
    })
    const plPreload = createSlider('', `${plId}_preload`, (e) => {
      setSlider(e.target.id, e.target.checked)

      invoke('toggle_plugin', {
        name: plugin.name
      })
    })

    const tdName = document.createElement('td')
    tdName.innerHTML = plugin.name

    const tdEnabled = document.createElement('td')
    tdEnabled.appendChild(plToggle)

    const tdPreload = document.createElement('td')
    tdPreload.appendChild(plPreload)

    trow.appendChild(tdName)
    trow.appendChild(tdEnabled)
    trow.appendChild(tdPreload)
    table.appendChild(trow)

    console.log(plugin)
    console.log(`${plId}_enable`)

    // Toggle the sliders
    setSlider(`${plId}_enable`, !plugin.disabled)
    setSlider(`${plId}_preload`, plugin.preload)
  })
}