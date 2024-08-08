import { badPostMessagePatch, createLocalStorage } from './shared/recreate'
import { safemodeTimer, typingAnim } from './shared/ui'
import { cssSanitize, fetchImage, isJson, waitForApp, waitForElm, saferEval } from './shared/util'
import { applyNotificationCount } from './shared/window'

// Let's expose some stuff for use in plugins and such
window.Dorion = {
  util: {
    cssSanitize,
    isJson,
    fetchImage,
    waitForApp,
    waitForElm,
    applyNotificationCount
  },
  recreate: {
    createLocalStorage,
  },
  window: {
    applyNotificationCount
  },
  shouldShowUnreadBadge: false
}

if (!window.__DORION_INITIALIZED__) window.__DORION_INITIALIZED__ = false

;(async () => {
  // if we are in an iframe we don't really need to load anything, else we bork whatever is inside
  if (window.self !== window.top) {
    // fixes activities
    console.log('Patching postMessage...')
    badPostMessagePatch()

    console.log('Stopping here, we are in an iframe!')
    return
  }

  createLocalStorage()

  while (!window.__TAURI__) {
    console.log('Waiting for definition...')
    await new Promise(resolve => setTimeout(resolve, 200))
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
  const { event, app } = window.__TAURI__
  const { invoke } = window.__TAURI__.core
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

  window.Dorion.shouldShowUnreadBadge = window.__DORION_CONFIG__.unread_badge

  await invoke('load_plugins')

  const version = await app.getVersion()

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
  const logUnlisten = await event.listen('loading_log', (event: TauriEvent) => {
    const log = event.payload as string

    updateOverlay({
      logs: log
    })
  })

  let themeJs = await handleClientModThemeInjection()
  themeJs += await handleThemeInjection()

  updateOverlay({
    midtitle: 'Getting injection JS...'
  })

  const injectionJs = await invoke('get_injection_js', {
    themeJs,
  })

  saferEval(injectionJs)

  updateOverlay({
    midtitle: 'Done!'
  })

  // Remove loading container
  const loadingContainer = document.querySelector('#loadingContainer') as HTMLDivElement
  loadingContainer.style.opacity = '0'

  setTimeout(() => {
    document.querySelector('#loadingContainer')?.remove()
  }, 200)

  // Unlisten from the log event
  logUnlisten()
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
  const { invoke } = window.__TAURI__.core

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
  const { invoke } = window.__TAURI__.core

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
  const { invoke } = window.__TAURI__.core
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
