const { invoke } = window.__TAURI__;

window.addEventListener("DOMContentLoaded", () => {
  console.log('Ready!')

  invoke('eval', {
    contents: `
      let loaded = false

      let observer = new MutationObserver((mutations, obs) => {
        const innerApp = document.querySelector('div[class*="notDevTools"]').querySelector('div[class*="app-"]')
        const loading = Array.from(
          innerApp.children
        ).length === 2 || !innerApp.querySelector('div').className.includes('app')

        if (!loading && !loaded) {
          console.log('Discord is loaded!')

          onClientLoad()

          const nameAndDecorators = document.querySelector('div[class*="nameAndDecorators"]')
          nameAndDecorators.innerHTML = 'Welcome to Taurcord!'
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