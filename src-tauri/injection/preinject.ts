import { createLocalStorage, proxyFetch } from './shared/recreate'
import { safemodeTimer, typingAnim } from './shared/ui'
import { cssSanitize, fetchImage, isJson, waitForApp, waitForElm } from './shared/util'
import { applyNotificationCount } from './shared/window'

// Let's expose some stuff for use in plugins and such
window.Dorion = {
  util: {
    cssSanitize,
    isJson,
    fetchImage,
    waitForApp,
    waitForElm
  },
  recreate: {
    createLocalStorage,
    proxyFetch
  },
  window: {
    applyNotificationCount
  }
}

if (!window.__DORION_INITIALIZED__) window.__DORION_INITIALIZED__ = false

;(async () => {
  // Set useragent to be Chrome as it is closest to what we actually are
  Object.defineProperty(navigator, 'userAgent', {
    get: () => 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36'
  })

  createLocalStorage()
  proxyFetch()

  while (!window.__TAURI__) {
    console.log('Waiting for definition...')
    await new Promise(resolve => setTimeout(resolve, 100))
  }

  if (window.__DORION_INITIALIZED__) return

  console.log('__TAURI__ defined! Let\'s do this')

  // Make window.open become window.__TAURI__.shell.open
  window.nativeOpen = window.open
  window.open = (url: string | undefined | URL, target?: string, features?: string) => {
    // If this needs to open externally, do so
    if (target === '_blank' || !target) {
      window.__TAURI__.shell.open(url as string)
      return null
    } 

    // Otherwise, use the native open
    return window.nativeOpen(url as string, target, features)
  }

  // Set the app as initialized
  window.__DORION_INITIALIZED__ = true

  init()
})()

async function init() {
  const { invoke, event } = window.__TAURI__
  const config = await invoke('read_config_file')

  window.__DORION_CONFIG__ = isJson(config) ? JSON.parse(config) : {}

  // Recreate config if there is an issue
  if (!Object.keys(config).length || !config) {
    const defaultConf = await invoke('default_config')
    // Write
    await invoke('write_config_file', {
      config: defaultConf
    })

    window.__DORION_CONFIG__ = JSON.parse(defaultConf)
  }

  // Run a couple other background tasks before we begin the main stuff
  invoke('start_streamer_mode_watcher')

  const plugins = await invoke('load_plugins')
    .catch(e => console.error('Error reading plugins: ', e))
  const version = await window.__TAURI__.app.getVersion()

  await displayLoadingTop()

  // Start the safemode timer
  safemodeTimer(
    document.querySelector('#safemode') as HTMLDivElement
  )

  updateOverlay({
    subtitle: `Made with ❤️ by SpikeHD - v${version}`,
    midtitle: 'Localizing JS imports...'
  })

  typingAnim()

  // Start the loading_log event listener
  event.listen('loading_log', (event: TauriEvent) => {
    const log = event.payload as string

    updateOverlay({
      logs: log
    })
  })

  let themeJs = await handleThemeInjection()
  themeJs += await handleClientModThemeInjection()

  updateOverlay({
    midtitle: 'Getting injection JS...'
  })

  const injectionJs = await invoke('get_injection_js', {
    themeJs,
  })

  await invoke('load_injection_js', {
    contents: injectionJs,
    plugins
  })

  updateOverlay({
    midtitle: 'Done!'
  })

  // Remove loading container
  const loadingContainer = document.querySelector('#loadingContainer') as HTMLDivElement
  loadingContainer.style.opacity = '0'

  setTimeout(() => {
    document.querySelector('#loadingContainer')?.remove()
  }, 200)
}

/**
 * Nasty helper function _for updating the text on the overlay
 */
async function updateOverlay(toUpdate: Record<string, string>) {
  const midtitle = document.getElementById('#midtitle')
  const subtitle = document.getElementById('#subtitle')
  const safemode = document.getElementById('#safemode')
  const logs = document.getElementById('#logContainer')

  for (const [key, value] of Object.entries(toUpdate)) {
    if (key === 'midtitle' && midtitle) midtitle.innerHTML = value
    if (key === 'subtitle' && subtitle) subtitle.innerHTML = value
    if (key === 'safemode' && safemode) safemode.innerHTML = value
    if (key === 'logs' && logs) logs.innerHTML = value
  }
}

async function handleThemeInjection() {
  const { invoke } = window.__TAURI__

  // This needs to exist for hot-switching to work
  const ts = document.createElement('style')
  ts.id = 'dorion-theme'
  document.body.appendChild(ts)

  if (!window.__DORION_CONFIG__?.theme || window.__DORION_CONFIG__?.theme === 'none') return ''

  updateOverlay({
    midtitle: 'Loading theme CSS...'
  })

  // Get the initial theme
  const themeContents = await invoke('get_theme', {
    name: window.__DORION_CONFIG__.theme
  })

  updateOverlay({
    midtitle: 'Localizing CSS imports...'
  })

  // Localize the imports
  const localized = await invoke('localize_imports', {
    css: themeContents,
    name: window.__DORION_CONFIG__.theme
  })

  // This will use the DOM in a funky way to validate the css, then we make sure to fix up quotes
  const cleanContents = cssSanitize(localized)?.replaceAll('\\"', '\'')

  return `;(() => {
    const ts = document.querySelector('#dorion-theme')
    ts.textContent = \`
      ${cleanContents?.replace(/`/g, '\\`')
  // To this day I do not know why I need to do this
    .replace(/\\8/g, '')
    .replace(/\\9/g, '')
}
    \`

    console.log('[Theme Loader] Appending Styles')
  })()`
}

async function handleClientModThemeInjection() {
  const { invoke } = window.__TAURI__

  const ts = document.createElement('style')
  ts.id = 'dorion-client-mods-themes'
  document.body.appendChild(ts)

  updateOverlay({
    midtitle: 'Loading client mod theme CSS...'
  })

  // Get the initial theme
  const themeContents = await invoke('load_mods_css')

  // This will use the DOM in a funky way to validate the css, then we make sure to fix up quotes
  const cleanContents = cssSanitize(themeContents)?.replaceAll('\\"', '\'')

  return `;(() => {
    const ts = document.querySelector('#dorion-client-mods-themes')
    ts.textContent = \`
      ${cleanContents?.replace(/`/g, '\\`')
  // To this day I do not know why I need to do this
    .replace(/\\8/g, '')
    .replace(/\\9/g, '')
}
    \`

    console.log('[Theme Loader] Appending Client Mod Styles')
  })()`
}

/**
 * Display the splashscreen
 */
async function displayLoadingTop() {
  const { invoke } = window.__TAURI__
  const html = await invoke('get_index')
  const loadingContainer = document.createElement('div') satisfies HTMLDivElement
  loadingContainer.id = 'loadingContainer'
  loadingContainer.innerHTML = html

  loadingContainer.style.zIndex = '99999'
  loadingContainer.style.position = 'absolute'
  loadingContainer.style.top = '0'
  loadingContainer.style.left = '0'
  loadingContainer.style.width = '100vw'
  loadingContainer.style.height = '100vh'

  document.body.appendChild(loadingContainer)
}