window.dorionOrigin = '/* __ORIGIN__ */'
window.dorion = true

let loaded = false

let observer = new MutationObserver(() => {
  const innerApp = document?.querySelector('div[class*="notDevTools"]')?.querySelector('div[class*="app-"]')
  const loading = Array.from(
    innerApp?.children || []
  ).length === 2 || !innerApp?.querySelector('div').className.includes('app')

  if (!loading && !loaded) {
    console.log('Discord is loaded!')

    onClientLoad()

    // The comments ahead are read by tauri and used to insert plugin/theme injection code
    
    /* __PLUGINS__ */

    /* __THEMES__ */
  } else {
    console.log('Discord not loaded...')
  }
});

observer.observe(document, {
  childList: true,
  subtree: true
});

function onClientLoad() {
  loaded = true
  observer.disconnect()

  settingInserter()
}

function settingInserter() {
  let insertedSetting = false

  observer = new MutationObserver(() => {
    // Shove a new option in settings when it's open to go back to Dorion settings
    const appSettings = document.querySelectorAll('div[aria-label="User Settings"] div[class*="header-"]')[2]
    
    if (appSettings && !insertedSetting) {
      // Yoink the next tabs styling
      const classes = appSettings.nextSibling.classList
      const dorionTab = document.createElement('div')

      dorionTab.innerHTML = "Dorion Settings"
      dorionTab.onclick = () => window.location.assign(window.dorionOrigin + '/settings.html')
      dorionTab.classList = classes

      // There needs to be a small delay for some reason, or else the client just freezes up
      setTimeout(() => {
        appSettings.parentNode.insertBefore(dorionTab, appSettings.nextSibling)
      }, 100)

      insertedSetting = true
    } else if (!appSettings) {
      insertedSetting = false;
    }
  })

  observer.observe(document.querySelector('div[class*="app"]'), {
    childList: true,
    subtree: true
  });
}