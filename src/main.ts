const TITLE = 'Dorion'

interface Config {
  theme: string
  zoom: string
  client_type: string
}

window.addEventListener("DOMContentLoaded", async () => {
  const { invoke } = window.__TAURI__;

  const config = JSON.parse(await invoke('read_config_file')) as Config
  const plugins = await invoke('load_plugins')
  const version = await window.__TAURI__.app.getVersion()
  const subtitle = document.querySelector('#subtitle')

  if (subtitle) subtitle.innerHTML = `Made with ❤️ by SpikeHD - v${version}</br></br>Press 'F' to enter settings`

  typingAnim()

  document.addEventListener('keydown', (e) => {
    if (e.key.toLowerCase() === 'f') {
      // Interrupt the loading and put us in settings
      window.location.assign('/settings.html')
    }
  })

  // Wait just a couple seconds in case the user wants to enter the settings menu
  await new Promise(r => setTimeout(r, 2000))

  // Get theme if it exists
  let themeInjection = ''

  if (config.theme !== 'none') {
    const themeContents = await invoke('get_theme', {
      name: config.theme
    }) as string

    const localized = await invoke('localize_imports', {
      css: themeContents
    }) as string

    const cleanContents = cssSanitize(localized)

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

  const injectionJs = await invoke('get_injection_js', {
    pluginJs: plugins,
    themeJs: themeInjection,
    origin: window.location.origin
  })

  invoke('load_injection_js', {
    contents: injectionJs
  })

  if (config.client_type !== 'default') {
    window.location.assign(`https://${config.client_type}.discord.com/app`)
  } else window.location.assign('https://discord.com/app')
});

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

async function timeout(ms: number) {
  return new Promise(r => setTimeout(r, ms))
}

// Prevent any fuckery within themes
function cssSanitize(css: string) {
  const style = document.createElement('style');
  style.innerHTML = css;

  document.head.appendChild(style);

  if (!style.sheet) return

  const result = Array.from(style.sheet.cssRules).map(rule => rule.cssText || '').join('\n');

  document.head.removeChild(style);
  return result;
}