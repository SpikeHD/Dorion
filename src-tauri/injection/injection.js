// Ensure we don't fire more than we have to
window.__TAURI__.invoke('is_injected')

// Keys for PTT key thing
let keyCapturing = false
let pttKeysCaptured = []
const keyClass = 'key-RP8gj3'

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

/**
 * Run when the client is "loaded"
 */
function onClientLoad() {
  loaded = true
  observer.disconnect()
  
  // Insert settings tab
  settingInserter()

  // Recreate window.localStorage
  createLocalStorage()

  // Notifcation watcher
  notifGetter()

  // Assign notification count
  applyNotificationCount()

  // Initialize top bar events
  initTopBarEvents()

  // Check for updates
  console.log('Checking for updates...')
  checkForUpdates()
}

/**
 * Show notification
 */
async function showNotification(title, body) {
  const { invoke } = window.__TAURI__
  const notifHtml = await invoke('get_notif')
  const notif = document.createElement('div')
  notif.innerHTML = notifHtml

  const inner = notif.querySelector('#dorion_notif')

  inner.style.top = '-100%'
  inner.style.transition = 'all 0.5s ease-in-out'

  inner.querySelector('#notif_title').innerHTML = title
  inner.querySelector('#notif_body').innerHTML = body

  const inst = document.body.appendChild(notif)

  // Move into view
  setTimeout(() => {
    inner.style.top = '5%'
  }, 100)

  // After 4 seconds, move out of view and remove
  setTimeout(() => {
    inner.style.top = '-100%'
    setTimeout(() => {
      inst.remove()
    }, 500)
  }, 4000)
}

/**
 * Check for updates
 */
async function checkForUpdates() {
  const { invoke, app } = window.__TAURI__
  const version = await app.getVersion()
  const latest = await invoke('get_latest_release')

  // remove letters from latest release
  const latestNum = latest.tag_name.replace(/[a-z]/gi, '').trim()

  if (version !== latestNum) {
    showNotification('Update Available', `<a href="${latest.link}">Dorion v${latestNum}</a> is now available!`)
  }
}
/**
 * Give events to the top bar buttons
 */
function initTopBarEvents() {
  document.querySelector('#topclose').onclick = close
  document.querySelector('#topmin').onclick = minimize
  document.querySelector('#topmax').onclick = maximize
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
    const appSettings = document.querySelectorAll('nav[class*="sidebar-"] div[class*="header"]')[4]
    
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
  const ptt = document.querySelector('#ptt')
  const pttKeys = document.querySelector('#ptt-key-section')

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

  if (ptt) {
    ptt.checked = config.ptt
    setSlider('ptt', config.ptt)

    // If true, enable ptt-keys
    if (config.ptt) {
      document.querySelector('#ptt-keys').style.display = 'block'
    }
  }

  if (pttKeys) {
    // Fill with keys
    const keys = config.push_to_talk_keys

    keys.forEach(key => {
      pttKeys.innerHTML = ''
      pttKeys.innerHTML += `
      <span class="${keyClass}">${key}</span>
      `
    })
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
  const ptt = document.querySelector('#ptt')

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

  ptt?.addEventListener('change', (evt) => {
    const tgt = evt.target

    setSlider(evt.target.id, tgt.checked)
    setConfigValue('ptt', tgt.checked)

    // Also make ptt-keys appear if checked
    const pttKeys = document.querySelector('#ptt-keys')
    if (pttKeys) {
      pttKeys.style.display = tgt.checked ? 'block' : 'none'
    }
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

  elm.checked = enabled
  
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
  const pttKeys = document.querySelector('#ptt-key-section')
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

  if (pttKeys) {
    pttKeys.addEventListener('click', pttKeyFunc)
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
  <div class="control-10qYax">
    <div class="container-1QtPKm default-colors" style="opacity: 1; background-color: rgb(128, 132, 142);">
      <!-- Checkbox/slider SVG -->
      <svg class="slider-HJFN2i" viewBox="0 0 28 20" preserveAspectRatio="xMinYMid meet" aria-hidden="true"
        style="left: -3px;">
        <rect fill="white" x="4" y="0" height="20" width="20" rx="10"></rect><svg viewBox="0 0 20 20"
          fill="none">
          <path fill="rgba(128, 132, 142, 1)"
            d="M5.13231 6.72963L6.7233 5.13864L14.855 13.2704L13.264 14.8614L5.13231 6.72963Z"></path>
          <path fill="rgba(128, 132, 142, 1)"
            d="M13.2704 5.13864L14.8614 6.72963L6.72963 14.8614L5.13864 13.2704L13.2704 5.13864Z"></path>
        </svg>
      </svg>
      <!-- End SVG -->
      <input type="checkbox" class="input-125oad" id="${id}" name="${id}" />
    </div>
  </div>
  `

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
    const plToggle = createSlider('', `${plId}_enable`, async (e) => {
      const enabled = await invoke('toggle_plugin', {
        name: plugin.name
      })

      setSlider(e.target.id, enabled)
    })
    const plPreload = createSlider('', `${plId}_preload`, async (e) => {
      const preload = await invoke('toggle_preload', {
        name: plugin.name
      })

      setSlider(e.target.id, preload)
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

    // Toggle the sliders
    setSlider(`${plId}_enable`, !plugin.disabled)
    setSlider(`${plId}_preload`, plugin.preload)
  })
}

/**
 * Used to capture the key combo for PTT
 */
function pttKeyFunc(evt) {
  if (keyCapturing) return

  keyCapturing = true

  // Clear existing content
  evt.target.innerHTML = ''

  // Insert text with instructions
  evt.target.innerHTML = `
  <span class="${keyClass}">Press any combination of keys</span>
  `

  // Clear the array
  pttKeysCaptured = []

  // Begin capturing key presses
  document.addEventListener('keydown', pttKeyCapture)
  document.addEventListener('keyup', pttEndCapture)
}

function pttKeyCapture(evt) {
  const pttKeys = document.querySelector('#ptt-key-section')
  let key = ''

  if (pttKeysCaptured.includes(evt.keyCode) || !evt.keyCode) {
    return
  }

  if (pttKeysCaptured.length <= 0) {
    pttKeys.innerHTML = ``
  }

  // Differentiate from left and right control
  key = evt.key === 'Control' ? evt.location === 1 ? 'LControl' : 'RControl' : evt.key

  // ...and left/right Alt
  key = evt.key === 'Alt' ? evt.location === 1 ? 'LAlt' : 'RAlt' : evt.key

  // ...and left/right Shift
  key = evt.key === 'Shift' ? evt.location === 1 ? 'LShift' : 'RShift' : evt.key

  pttKeysCaptured.push(key)

  // Add the key to the ptt keys section element
  pttKeys.innerHTML += `
  <span class="${keyClass}">${key}</span>
  `
}

/**
 * When the user releases keys from a key combo, clelar both event listeners
 */
function pttEndCapture(evt) {
  document.removeEventListener('keydown', pttKeyCapture)
  document.removeEventListener('keyup', pttEndCapture)

  keyCapturing = false

  console.log('Final key combo: ', pttKeysCaptured)

  // Save the key combo to the config
  setConfigValue('push_to_talk_keys', pttKeysCaptured)
}