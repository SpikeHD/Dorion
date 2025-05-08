import { TOP_HTML } from './static'
import { timeout, waitForApp, waitForElm } from './util'
import { close, minimize, setMaximizeIcon, toggleMaximize } from './window'

export function safemodeTimer(elm: HTMLDivElement) {
  setTimeout(() => {
    elm.classList.add('show')
  }, 10000)

  const tmpKeydown = (evt: KeyboardEvent) => {
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
      window.__TAURI__.core.invoke('open_themes')
    }
  }

  document.addEventListener('keydown', tmpKeydown)
}

export async function typingAnim() {
  const title = document.querySelector('#title')

  if (!title) return

  for (const letter of 'Dorion'.split('')) {
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

export async function applyExtraCSS() {
  const { invoke } = window.__TAURI__.core
  const css = await invoke('get_extra_css')
  const style = document.createElement('style')

  style.innerHTML = css
  style.id = 'dorion-extra-css'

  // Append some background-transparenting css if blur_css is true
  if (window.__DORION_CONFIG__.blur !== 'none' && window.__DORION_CONFIG__.blur_css) {
    style.innerHTML += `
      * {
        background: transparent !important;
      }
    `
  }

  document.body.appendChild(style)
}

async function initTopBarEvents() {
  const topclose = document.querySelector('#topclose') as HTMLDivElement
  const topmin = document.querySelector('#topmin') as HTMLDivElement
  const topmax = document.querySelector('#topmax') as HTMLDivElement

  topclose.onclick = close
  topmin.onclick = minimize
  topmax.onclick = toggleMaximize
}

export function handleTopBar() {
  // This needs to render after discord is loaded
  if (
    !window.__DORION_CONFIG__.use_native_titlebar
  ) {
    window.__TAURI__.core.invoke('set_decorations', { enable: false }).catch(_e => {})
    // Remove the old one, if it is loaded
    const topBar = document.querySelector('#dorion_topbar')

    if (topBar) topBar.remove()
    createTopBar()
  }
}

export async function createTopBar() {
  const isLogin = window.location.href.includes('login')

  // We have to use the old top bar if we are on the login screen, because there is nothing to integrate into
  if (!isLogin) {
    const topMount = await waitForElm('div[class*="recentsIcon_"]')
    const parent = topMount?.parentElement
  
    // This shouldn't ever happen
    if (!parent) return
  
    parent.insertAdjacentHTML('beforeend', TOP_HTML)
  
    // Set the tauri drag region to the top bar
    const wholeParent = parent?.parentElement
    const wholeTop = wholeParent?.querySelector('div[class*="title_"]')
  
    if (wholeTop) {
      wholeTop.setAttribute('data-tauri-drag-region', 'true')
      // @ts-expect-error this exists
      wholeParent.style.paddingRight = '0px'
    }
  } else {
    const topbar = document.createElement('div')
    const content = await window.__TAURI__.core
      .invoke('get_top_bar')
      .catch((e) => console.error('Error reading top bar: ', e))
  
    // If the top bar failed to load, stick to the default
    if (!content) return
  
    topbar.innerHTML = content
    topbar.id = 'dorion_topbar'

    const appMount = await waitForApp()
    // Actual mount is further up the tree on the new UI
    const innerMountBase = document.querySelector('div[class*=appAsidePanelWrapper_]')

    if (!appMount || document.querySelector('#dorion_topbar')) return
  
    if (innerMountBase) {
      // This should be defined if the base was
      const innerMount = await waitForElm('div[class*=notAppAsidePanel_]') as Element
      innerMount.prepend(topbar)
    } else {
      appMount.prepend(topbar)
    }
  }

  window.__TAURI__.event.listen(
    window.__TAURI__.event.TauriEvent.WINDOW_RESIZED,
    setMaximizeIcon
  )

  setMaximizeIcon()

  // Once done, remove original top bar
  window.__TAURI__.core.invoke('remove_top_bar')

  initTopBarEvents()
}

export async function extraCssChangeWatch() {
  const { event, core } = window.__TAURI__
  const style = document.createElement('style')
  style.id = 'dorion-os-accent'

  const elm = document.body.appendChild(style)

  // Get the initial color
  const initial = await core.invoke('get_os_accent')
  const setAccentColor = (color: string) => {
    elm.innerText = `html { --os-accent-color: ${color} !important; }`
  }

  setAccentColor(initial)

  event.listen('os_accent_update', (event) => {
    setAccentColor(event.payload as string)
  })
}
