const TITLE = 'Dorion'

window.onbeforeunload = () => {
  window.__TAURI__.invoke('inject_routine')
}

// Needs to be done ASAP
// interceptEventListeners()

function safemodeTimer(elm) {
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

/**
 * This is a bunch of scaffolding stuff that is run before the actual injection script is run.
 * This will localize imports for JS and CSS, as well as some other things
 */
(async () => {
  // Set useragent to be Chrome as it is closest to what we actually are
  Object.defineProperty(navigator, 'userAgent', {
    get: () => 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36'
  })

  createLocalStorage()
  proxyFetch()

  await displayLoadingTop()

  const { invoke, event } = window.__TAURI__
  let config = {}
  
  try {
    config = JSON.parse(await invoke('read_config_file'))
  } catch(e) {
    console.log('Error reading config.')
  }

  window.DorionConfig = config

  // If there is an issue with the config, we need to recreate a default one
  if (!config) {
    const defaultConf = await invoke('default_config')
    // Write
    await invoke('write_config_file', {
      config: defaultConf
    })

    config = JSON.parse(defaultConf)
  }

  // Make window.open become window.__TAURI__.shell.open
  window.open = (url) => window.__TAURI__.shell.open(url)

  const plugins = await invoke('load_plugins')
    .catch(e => console.error("Error reading plugins: ", e));
  const version = await window.__TAURI__.app.getVersion()
  const midtitle = document.querySelector('#midtitle')
  const subtitle = document.querySelector('#subtitle')
  const safemode = document.querySelector('#safemode')
  const logs = document.querySelector('#logContainer')

  // Start safemode timer and event listener right away, just in case
  safemodeTimer(safemode)
  
  if (subtitle) subtitle.innerHTML = `Made with ❤️ by SpikeHD - v${version}`

  typingAnim()
  
  if (midtitle) midtitle.innerHTML = "Localizing JS imports..."

  let importUrls = []

  // Iterate through the values of "plugins" (which is all of the plugin JS)
  for (const js in Object.values(plugins)) {
    importUrls = [ ...importUrls, ...(await invoke('get_plugin_import_urls', {
      pluginJs: js
    }))]
  }

  event.listen('loading_log', (event) => {
    const log = event.payload

    if (!logs) return

    logs.innerHTML = `${log}`
  })

  const imports = await invoke('localize_all_js', {
    urls: importUrls
  })

  // Get theme if it exists
  let themeInjection = ''

  if (config.theme && config.theme !== 'none') {
    if (midtitle) midtitle.innerHTML = "Loading theme CSS..."

    const themeContents = await invoke('get_theme', {
      name: config.theme
    })

    if (midtitle) midtitle.innerHTML = "Localizing CSS imports..."
    const localized = await invoke('localize_imports', {
      css: themeContents,
      name: config.theme
    })

    // This will use the DOM in a funky way to validate the css, then we make sure to fix up quotes
    const cleanContents = cssSanitize(localized)?.replaceAll('\\"', '\'')

    // Write theme injection code
    themeInjection = `;(() => {
      const ts = document.createElement('style')

      ts.textContent = \`
        ${cleanContents?.replace(/`/g, '\\`')
            .replace(/\\8/g, '')
            .replace(/\\9/g, '')
          }
      \`

      console.log('[Theme Loader] Appending Styles')
      document.body.appendChild(ts)
    })()`
  }

  if (midtitle) midtitle.innerHTML = "Getting injection JS..."

  const injectionJs = await invoke('get_injection_js', {
    themeJs: themeInjection,
  })

  await invoke('load_injection_js', {
    imports,
    contents: injectionJs,
    plugins
  })

  // Disable telemetry
  if (!config.block_telemetry) blockTelemetry()

  if (midtitle) midtitle.innerHTML = "Done!"

  // Remove loading container
  document.querySelector('#loadingContainer').style.opacity = 0

  setTimeout(() => {
    document.querySelector('#loadingContainer')?.remove()
  }, 200)
})()

/**
 * Display the splashscreen
 */
async function displayLoadingTop() {
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
async function typingAnim() {
  const title = document.querySelector('#title')

  if (!title) return

  for (const letter of TITLE.split('')) {
    title.innerHTML = title.innerHTML.replace('|', '') + letter + '|'

    await timeout(100)
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
async function timeout(ms) {
  return new Promise(r => setTimeout(r, ms))
}

/**
 * Prevent any fuckery within themes
 * 
 * @param {String} css 
 * @returns The sanitized CSS
 */
function cssSanitize(css) {
  const style = document.createElement('style');
  style.innerHTML = css;

  document.head.appendChild(style);

  if (!style.sheet) return

  const result = Array.from(style.sheet.cssRules).map(rule => rule.cssText || '').join('\n');

  document.head.removeChild(style);
  return result;
}

/**
 * Block Discord telemetry
 */
function blockTelemetry() {
  const open = XMLHttpRequest.prototype.open;
  
  XMLHttpRequest.prototype.open = function(method, url) {
    open.apply(this, arguments)

    const send = this.send

    this.send = function() {
      const rgx = /\/api\/v.*\/(science|track)/g

      if (!String(url).match(rgx)) {
        return send.apply(this, arguments)
      }

      console.log(`[Telemetry Blocker] Blocked URL: ${url}`)
    }
  }
}

/**
 * Discord wipes `window.localStorage`, so we have to recreate it in case plugins require it
 * 
 * https://github.com/SpikeHD/Dorion/issues/7#issuecomment-1320861432
 */
function createLocalStorage() {
  const iframe = document.createElement('iframe');

  // Wait for document.head to exist, then append the iframe
  const interval = setInterval(() => {
    if (!document.head) return

    document.head.append(iframe);
    const pd = Object.getOwnPropertyDescriptor(iframe.contentWindow, 'localStorage');
    iframe.remove();
    
    Object.defineProperty(window, 'localStorage', pd)

    clearInterval(interval)
  }, 50)
}

function isJson(s) {
  try {
    JSON.parse(s);
  } catch (e) {
    return false;
  }
  return true;
}

/**
 * Overwrite the global fetch function with a new one that will redirect to the tauri API 
 */
function proxyFetch() {
  window.nativeFetch = window.fetch

  // eslint-disable-next-line no-global-assign
  fetch = async (url, options) => {
    const { http } = window.__TAURI__
    const discordReg = /https?:\/\/(?:[a-z]+\.)?(?:discord\.com|discordapp\.net)(?:\/.*)?/g

    // If it matches, just let it go through native
    if (url.toString().match(discordReg)) {
      return window.nativeFetch(url, options)
    }

    // If there is an options.body, check if it's valid JSON. if so, set that up
    if (options && options?.body) {
      const bodyContent = isJson(options.body) ? http.Body.json(options.body) : typeof options.body === 'string' ? http.Body.text(options.body) : http.Body.bytes(options.body)
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