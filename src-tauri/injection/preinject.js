
if (!window.__DORION_INITIALIZED__) window.__DORION_INITIALIZED__ = false
const TITLE = 'Dorion'

// Check if window.__TAURI__ is available, if not, wait for it to be available
// This is to prevent the script from running before Tauri is ready
;(async () => {
  // Set useragent to be Chrome as it is closest to what we actually are
  Object.defineProperty(navigator, 'userAgent', {
    get: () => 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36'
  })

  _createLocalStorage()
  _proxyFetch()

  while (!window.__TAURI__) {
    console.log('Waiting for definition...')
    await new Promise(resolve => setTimeout(resolve, 100))
  }

  if (window.__DORION_INITIALIZED__) return

  console.log('__TAURI__ defined! Let\'s do this')

  // Make window.open become window.__TAURI__.shell.open
  window.open = (url) => window.__TAURI__.shell.open(url)

  // Set the app as initialized
  window.__DORION_INITIALIZED__ = true

  _init()
})()

async function _init() {
  const { invoke, event } = window.__TAURI__
  const config = await invoke('read_config_file')

  window.__DORION_CONFIG__ = _isJson(config) ? JSON.parse(config) : {}

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

  await _displayLoadingTop()

  // Start the safemode timer
  _safemodeTimer(
    document.querySelector('#safemode')
  )

  _updateOverlay({
    subtitle: `Made with ❤️ by SpikeHD - v${version}`,
    midtitle: 'Localizing JS imports...'
  })

  _typingAnim()

  // Start the loading_log event listener
  event.listen('loading_log', (event) => {
    const log = event.payload

    _updateOverlay({
      logs: log
    })
  })

  const themeJs = await _handleThemeInjection()

  _updateOverlay({
    midtitle: 'Getting injection JS...'
  })

  const injectionJs = await invoke('get_injection_js', {
    themeJs,
  })

  await invoke('load_injection_js', {
    contents: injectionJs,
    plugins
  })

  _updateOverlay({
    midtitle: 'Done!'
  })

  // Remove loading container
  document.querySelector('#loadingContainer').style.opacity = 0

  setTimeout(() => {
    document.querySelector('#loadingContainer')?.remove()
  }, 200)
}

/**
 * Nasty helper function _for updating the text on the overlay
 */
async function _updateOverlay(toUpdate) {
  const midtitle = document.querySelector('#midtitle')
  const subtitle = document.querySelector('#subtitle')
  const safemode = document.querySelector('#safemode')
  const logs = document.querySelector('#logContainer')

  for (const [key, value] of Object.entries(toUpdate)) {
    if (key === 'midtitle' && midtitle) midtitle.innerHTML = value
    if (key === 'subtitle' && subtitle) subtitle.innerHTML = value
    if (key === 'safemode' && safemode) safemode.innerHTML = value
    if (key === 'logs' && logs) logs.innerHTML = value
  }
}

async function _handleThemeInjection() {
  const { invoke } = window.__TAURI__

  if (!window.__DORION_CONFIG__?.theme || window.__DORION_CONFIG__?.theme === 'none') return ''

  _updateOverlay({
    midtitle: 'Loading theme CSS...'
  })

  // Get the initial theme
  const themeContents = await invoke('get_theme', {
    name: window.__DORION_CONFIG__.theme
  })

  _updateOverlay({
    midtitle: 'Localizing CSS imports...'
  })

  // Localize the imports
  const localized = await invoke('localize_imports', {
    css: themeContents,
    name: window.__DORION_CONFIG__.theme
  })

  // This will use the DOM in a funky way to validate the css, then we make sure to fix up quotes
  const cleanContents = _cssSanitize(localized)?.replaceAll('\\"', '\'')

  return `;(() => {
    const ts = document.createElement('style')

    ts.textContent = \`
      ${cleanContents?.replace(/`/g, '\\`')
  // To this day I do not know why I need to do this
    .replace(/\\8/g, '')
    .replace(/\\9/g, '')
}
    \`

    console.log('[Theme Loader] Appending Styles')
    document.body.appendChild(ts)
  })()`
}

/**
 * Display the splashscreen
 */
async function _displayLoadingTop() {
  const { invoke } = window.__TAURI__
  const html = await invoke('get_index')
  const loadingContainer = document.createElement('div')
  loadingContainer.id = 'loadingContainer'
  loadingContainer.innerHTML = html

  loadingContainer.style.zIndex = 99999
  loadingContainer.style.position = 'absolute'
  loadingContainer.style.top = '0'
  loadingContainer.style.left = '0'
  loadingContainer.style.width = '100vw'
  loadingContainer.style.height = '100vh'

  document.body.appendChild(loadingContainer)
}

/**
 * Play the little typing animation in the splash screen
 */
async function _typingAnim() {
  const title = document.querySelector('#title')

  if (!title) return

  for (const letter of TITLE.split('')) {
    title.innerHTML = title.innerHTML.replace('|', '') + letter + '|'

    await _timeout(100)
  }

  // Once the "typing" is done, blink the cursor
  let cur = true

  setInterval(() => {
    if (cur) {
      cur = false

      title.innerHTML = title.innerHTML.replace('|', '&nbsp;')
      return
    }

    cur = true

    title.innerHTML = title.innerHTML.replace(/&nbsp;$/, '|')
  }, 500)
}

/**
 * Small helper to wait a couple seconds before doing something
 * 
 * @param {Number} ms 
 * @returns 
 */
async function _timeout(ms) {
  return new Promise(r => setTimeout(r, ms))
}

/**
 * Prevent any fuckery within themes
 * 
 * @param {String} css 
 * @returns The sanitized CSS
 */
function _cssSanitize(css) {
  const style = document.createElement('style')
  style.innerHTML = css

  document.head.appendChild(style)

  if (!style.sheet) return

  const result = Array.from(style.sheet.cssRules).map(rule => rule.cssText || '').join('\n')

  document.head.removeChild(style)
  return result
}

function _safemodeTimer(elm) {
  setTimeout(() => {
    elm.classList.add('show')
  }, 10000)

  const tmpKeydown = (evt) => {
    // If loading container doesn't exist, we made it through and should stop watching key events
    if (!document.querySelector('#loadingContainer')) {
      document.removeEventListener('keydown', tmpKeydown)
      return
    }

    // If spacebar, remove #loadingContainer
    if (evt.code === 'Space') {
      document.querySelector('#loadingContainer')?.remove()
    }

    // If F, open plugins folder
    if (evt.code === 'KeyF') {
      window.__TAURI__.invoke('open_themes')
    }
  }

  document.addEventListener('keydown', tmpKeydown)
}

async function _createLocalStorage() {
  const iframe = document.createElement('iframe')

  // Wait for document.head to exist, then append the iframe
  const interval = setInterval(() => {
    if (!document.head || window.localStorage) return

    document.head.append(iframe)
    const pd = Object.getOwnPropertyDescriptor(iframe.contentWindow, 'localStorage')
    iframe.remove()

    Object.defineProperty(window, 'localStorage', pd)

    console.log('[Create LocalStorage] Done!')

    clearInterval(interval)
  }, 50)
}

function _isJson(s) {
  try {
    JSON.parse(s)
  } catch (e) {
    return false
  }
  return true
}

/**
 * Overwrite the global fetch function _with a new one that will redirect to the tauri API 
 */
function _proxyFetch() {
  window.nativeFetch = window.fetch

  window.fetch = async (url, options) => {
    const { http } = window.__TAURI__
    const discordReg = /https?:\/\/(?:[a-z]+\.)?(?:discord\.com|discordapp\.net)(?:\/.*)?/g
    const scienceReg = /\/api\/v.*\/(science|track)/g

    // If it matches, just let it go through native
    if (url.toString().match(discordReg)) {
      // Block science though!
      if (url.toString().match(scienceReg)) {
        console.log(`[Fetch Proxy] Blocked URL: ${url}`)
        return
      }

      return window.nativeFetch(url, options)
    }

    // If there is an options.body, check if it's valid JSON. if so, set that up
    if (options && options?.body) {
      const bodyContent = _isJson(options.body) ? http.Body.json(options.body) : typeof options.body === 'string' ? http.Body.text(options.body) : http.Body.bytes(options.body)
      options.body = bodyContent
    }

    if (options && options?.headers) {
      // Check if header object, if so convert back to Record<String, any>
      if (options.headers instanceof Headers) {
        const headers = {}

        for (const [key, value] of options.headers.entries()) {
          headers[key] = value
        }

        options.headers = headers
      }
    }

    const response = await http.fetch(url, {
      responseType: 2,
      ...options
    })

    // Adherence to what most scripts will expect to have available when they are using fetch(). These have to pretend to be promises
    response.json = async () => JSON.parse(response.data)
    response.text = async () => response.data

    response.headers = new Headers(response.headers)

    return response
  }
}
