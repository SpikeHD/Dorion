const TITLE = 'Dorion'

// Tell tauri to re-inject as we unload, in the case of a refresh
window.onbeforeunload = () => {
  window.__TAURI__.invoke('do_injection')
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
  createLocalStorage()

  await displayLoadingTop()

  const { invoke, event } = window.__TAURI__
  let config = JSON.parse(await invoke('read_config_file')) || null

  // If there is an issue with the config, we need to recreate a default one
  if (!config) {
    const defaultConf = await invoke('default_config')
    // Write
    await invoke('write_config_file', {
      config: defaultConf
    })

    config = JSON.parse(defaultConf)
  }

  const plugins = await invoke('load_plugins');
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

  if (config.theme !== 'none') {
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
    // const cleanContents = cssSanitize(localized)?.replaceAll('\\"', '\'')
    const cleanContents = localized

    console.log(cleanContents)

    // Write theme injection code
    themeInjection = `;(() => {
      const ts = document.createElement('style')

      ts.textContent = \`
        ${cleanContents?.replace(/`/g, '\\`')
            .replace(/\\8/g, '')
            .replace(/\\9/g, '')
          }
      \`

      document.head.appendChild(ts)
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
 * Make all events "trusted", preventing Discord from discarding them
 * 
 * https://stackoverflow.com/a/64991159
 */
function _interceptEventListeners() {
  Element.prototype._addEventListener = Element.prototype.addEventListener;
  Element.prototype.addEventListener = function () {
    let args = [...arguments]
    let temp = args[1];
    args[1] = function () {
      let args2 = [...arguments];
      args2[0] = Object.assign({}, args2[0])
      args2[0].isTrusted = true;
      return temp(...args2);
    }
    return this._addEventListener(...args);
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