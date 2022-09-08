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

    const cleanContents = cssSanitize(themeContents)

    // Write theme injection code
    themeInjection = `;(() => {
      const ts = document.createElement('style')

      ts.textContent = \`
        ${cleanContents?.replace(/`/g, '\\`')}
      \`

      document.head.append(ts)
    })()`
  }

  invoke('load_injection_js', {
    contents: `
      window.dorion = true

      let loaded = false

      let observer = new MutationObserver((mutations, obs) => {
        const innerApp = document?.querySelector('div[class*="notDevTools"]')?.querySelector('div[class*="app-"]')
        const loading = Array.from(
          innerApp?.children || []
        ).length === 2 || !innerApp?.querySelector('div').className.includes('app')

        if (!loading && !loaded) {
          console.log('Discord is loaded!')

          onClientLoad()

          // Exec plugins
          ${plugins}

          // Load theme
          ${themeInjection}
        } else {
          console.log('Discord not loaded...')
        }
      });

      observer.observe(document, {
        childList: true,
        subtree: true
      });

      function onClientLoad() {
        observer.disconnect()
        observer = null
        loaded = true
      }
    `
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