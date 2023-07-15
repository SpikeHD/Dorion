// Ensure we don't fire more than we have to
window.__TAURI__.invoke('is_injected')

// Create URL opener which will open links in the default system browser
// TODO: Don't resort to using this yet
// window.openURL = (url) => {
//   window.ipc.postMessage(JSON.stringify({
//     cmd: 'open_url',
//     callback: 0,
//     error: 0,
//     inner: {
//       url
//     }
//   }));
// }
window.dorion = true

let loaded = false

/**
 * Observer that lets us know when Discord is loaded
 */
let observer = new MutationObserver(() => {
  const innerApp = document?.querySelector('div[class*="app"]')?.querySelector('div[class*="app"]')
  const loading = Array.from(
    innerApp?.children || []
  ).length === 2

  if (loading && !loaded) {
    console.log('Discord is loaded!')

    onClientLoad()

    // The comments ahead are read by tauri and used to insert plugin/theme injection code
    
    /* __THEMES__ */
  } else {
    console.log('Discord not loaded...')
  }
});

observer.observe(document, {
  childList: true,
  subtree: true
});

function close() {
  window.__TAURI__.invoke('close')
}

function minimize() {
  window.__TAURI__.invoke('minimize')
}

function maximize() {
  window.__TAURI__.invoke('maximize')
}

/**
 * Run when the client is "loaded"
 */
function onClientLoad() {
  loaded = true
  observer.disconnect()

  // Recreate window.localStorage
  createLocalStorage()

  // Notifcation watcher
  notifGetter()

  // Assign notification count
  applyNotificationCount()

  // Initialize top bar events
  initTopBarEvents()

  // Check for updates
  console.log('Checking for updates...')
  checkForUpdates()
}

/**
 * Show notification
 */
async function showNotification(title, body) {
  const { invoke } = window.__TAURI__
  const notifHtml = await invoke('get_notif')
  const notif = document.createElement('div')
  notif.innerHTML = notifHtml

  const inner = notif.querySelector('#dorion_notif')

  inner.style.top = '-100%'
  inner.style.transition = 'all 0.5s ease-in-out'

  inner.querySelector('#notif_title').innerHTML = title
  inner.querySelector('#notif_body').innerHTML = body

  const inst = document.body.appendChild(notif)

  // Move into view
  setTimeout(() => {
    inner.style.top = '5%'
  }, 100)

  // After 4 seconds, move out of view and remove
  setTimeout(() => {
    inner.style.top = '-100%'
    setTimeout(() => {
      inst.remove()
    }, 500)
  }, 4000)
}

/**
 * Check for updates
 */
async function checkForUpdates() {
  const { invoke, app } = window.__TAURI__
  const version = await app.getVersion()
  const latest = await invoke('get_latest_release')

  // remove letters from latest release
  const latestNum = latest.tag_name.replace(/[a-z]/gi, '').trim()

  if (version !== latestNum) {
    showNotification('Update Available', `<a href="${latest.link}">Dorion v${latestNum}</a> is now available!`)
  }
}
/**
 * Give events to the top bar buttons
 */
function initTopBarEvents() {
  document.querySelector('#topclose').onclick = close
  document.querySelector('#topmin').onclick = minimize
  document.querySelector('#topmax').onclick = maximize
}

/**
 * Discord wipes `window.localStorage`, so we have to recreate it in case plugins require it
 * 
 * https://github.com/SpikeHD/Dorion/issues/7#issuecomment-1320861432
 */
function createLocalStorage() {
  const iframe = document.createElement('iframe');
  document.head.append(iframe);
  const pd = Object.getOwnPropertyDescriptor(iframe.contentWindow, 'localStorage');
  iframe.remove();
  
  Object.defineProperty(window, 'localStorage', pd)
}

function applyNotificationCount() {
  const { invoke } = window.__TAURI__
  const title = document.querySelector('title')
  const notifs = title.innerHTML.match(/\((.*)\)/)

  if (!notifs) {
    invoke('notif_count', {
      amount: 0
    })

    return
  }

  invoke('notif_count', {
    amount: Number(notifs[1])
  })
}

function notifGetter() {
  const notifObserver = new MutationObserver(applyNotificationCount)

  notifObserver.observe(document.querySelector('title'), {
    subtree: true,
    childList: true,
    attributes: true,
    characterData: true
  })
}
