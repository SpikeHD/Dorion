const TITLE = 'Dorion'

window.addEventListener("DOMContentLoaded", async () => {
  const { invoke } = window.__TAURI__;
  
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

  invoke('load_injection_js', {
    contents: `
      window.dorion = true

      let loaded = false

      let observer = new MutationObserver((mutations, obs) => {
        const innerApp = document.querySelector('div[class*="notDevTools"]').querySelector('div[class*="app-"]')
        const loading = Array.from(
          innerApp.children
        ).length === 2 || !innerApp.querySelector('div').className.includes('app')

        if (!loading && !loaded) {
          console.log('Discord is loaded!')

          onClientLoad()

          // Exec plugins
          ${plugins}
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

  window.location.assign('https://discord.com/app')
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