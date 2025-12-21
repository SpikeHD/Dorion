import { badPostMessagePatch, createLocalStorage, proxyFetch, proxyXHR, proxyAddEventListener, proxyOpen, proxyNotification } from './shared/recreate'
import { extraCssChangeWatch, safemodeTimer, typingAnim } from './shared/ui'
import { cssSanitize, fetchImage, isJson, waitForApp, waitForElm, saferEval, timeout } from './shared/util'
import { waitForElmEx } from './shared/wait_elm'

// Let's expose some stuff for use in plugins and such
window.Dorion = {
  util: {
    cssSanitize,
    isJson,
    fetchImage,
    waitForApp,
    waitForElm,
    waitForElmEx,
  },
  recreate: {
    createLocalStorage,
  },
  shouldShowUnreadBadge: false
}

const INJECTED_PLUGIN_OPTIONS = {
  isVisible: true,
  allowedActions: {},
  loaderName: 'Dorion'
}

window.SHELTER_INJECTOR_PLUGINS = {
  'Dorion Titlebar': ['https://spikehd.dev/shelter-plugins/dorion-titlebar/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Settings': ['https://spikehd.dev/shelter-plugins/dorion-settings/', INJECTED_PLUGIN_OPTIONS],
  'Always Trust': ['https://spikehd.dev/shelter-plugins/always-trust/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Notifications': ['https://spikehd.dev/shelter-plugins/dorion-notifications/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Streamer Mode': ['https://spikehd.dev/shelter-plugins/dorion-streamer-mode/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Updater': ['https://spikehd.dev/shelter-plugins/dorion-updater/', INJECTED_PLUGIN_OPTIONS],
  'Dorion PTT': ['https://spikehd.dev/shelter-plugins/dorion-ptt/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Tray': ['https://spikehd.dev/shelter-plugins/dorion-tray/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Fullscreen': ['https://spikehd.dev/shelter-plugins/dorion-fullscreen/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Custom Keybinds': ['https://spikehd.dev/shelter-plugins/dorion-custom-keybinds/', INJECTED_PLUGIN_OPTIONS],
  'Dorion Helpers': ['https://spikehd.dev/shelter-plugins/dorion-helpers/', INJECTED_PLUGIN_OPTIONS],
  'Web Keybinds': ['https://spikehd.dev/shelter-plugins/web-keybinds/', {
    ...INJECTED_PLUGIN_OPTIONS,
    allowedActions: {
      toggle: true,
    }
  }],
}

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
  proxyXHR()
  proxyAddEventListener()
  proxyNotification()

  while (!window.__TAURI__) {
    console.log('Waiting for definition...')
    await timeout(50)
  }

  window.__TAURI__.event.emit('js_context_loaded', null)

  proxyFetch()

  console.log('__TAURI__ defined!')

  extraCssChangeWatch()
  proxyOpen()

  const platform = await window.__TAURI__.core.invoke('get_platform')
  document.documentElement.setAttribute('data-dorion-platform', platform)

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

  // Discord Web depends on the `beforeunload` event being dispatched by the browser when
  // a tab is closed. However, this event is not triggered by the Webview so we need to
  // dispatch the `beforeunload` event ourselves.
  const dispatchBeforeUnload = () => {
    const event = new Event('beforeunload') as Event & { isTrustedOverwrite: boolean }
    event.isTrustedOverwrite = true
    window.dispatchEvent(event)
  }

  event.listen('beforeunload', dispatchBeforeUnload)
  event.listen(event.TauriEvent.WINDOW_CLOSE_REQUESTED, dispatchBeforeUnload)

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
    loadingContainer?.remove()
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

  if (!window.__DORION_CONFIG__?.themes || window.__DORION_CONFIG__?.themes.length === 0) return ''

  updateOverlay({
    midtitle: 'Loading theme CSS...'
  })

  // Get the initial theme
  const themeContents = await invoke('get_themes').catch(e => console.error(e)) || ''

  updateOverlay({
    midtitle: 'Localizing CSS imports...'
  })

  // Create a "name" for the "theme" (or combo) based on the retrieved enabled theme list
  const themeNames = await invoke('get_enabled_themes').catch(e => console.error(e)) || []
  // Gotta adhere to filename length restrictions
  const themeName = themeNames.join('').substring(0, 254)

  // Localize the imports. On windows this no longer does anything
  const localized = await invoke('localize_imports', {
    css: themeContents,
    name: themeName
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
