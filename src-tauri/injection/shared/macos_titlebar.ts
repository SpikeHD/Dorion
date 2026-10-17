const TITLEBAR_SELECTORS = [
  'div[class^="base"] > div[class^="bar_"]',
  'div[class^="base"] > div[class*="-bar"]',
]

const FALLBACK_TITLEBAR_HEIGHT = 30

const CLICKABLE_TAGS = new Set([
  'A',
  'BUTTON',
  'INPUT',
  'SELECT',
  'TEXTAREA',
  'LABEL',
  'SUMMARY',
])

const INTERACTIVE_ROLES = new Set([
  'button',
  'link',
  'menuitem',
  'tab',
  'checkbox',
  'radio',
  'switch',
  'option',
])

function enableWindowDrag(elm: Element) {
  if (elm.getAttribute('data-tauri-drag-region') !== 'deep') {
    elm.setAttribute('data-tauri-drag-region', 'deep')
  }
}

function isMacosOverlayTitlebar() {
  return (
    document.documentElement.dataset.dorionPlatform === 'macos'
    && window.__TAURI__?.webviewWindow?.getCurrentWebviewWindow?.()?.label === 'main'
    && !window.__DORION_CONFIG__?.use_native_titlebar
  )
}

function isClickableElement(elm: HTMLElement) {
  return (
    CLICKABLE_TAGS.has(elm.tagName)
    || (elm.hasAttribute('contenteditable') && elm.getAttribute('contenteditable') !== 'false')
    || (elm.hasAttribute('tabindex') && elm.getAttribute('tabindex') !== '-1')
    || INTERACTIVE_ROLES.has(elm.getAttribute('role') ?? '')
  )
}

function checkDragRegion(path: EventTarget[]) {
  for (const elm of path) {
    if (!(elm instanceof HTMLElement)) continue

    if (elm.hasAttribute('data-tauri-drag-region')) return 'tauri'
    if (isClickableElement(elm)) return 'interactive'
  }

  return 'unhandled'
}

function initWindowDragFallback(getDragAreaHeight: () => number) {
  let doubleClickPosition: [number, number] | null = null

  const invoke = (command: string) =>
    window.__TAURI__.core.invoke(`plugin:window|${command}`)

  document.addEventListener('mousedown', (event) => {
    if (
      event.button !== 0
      || (event.detail !== 1 && event.detail !== 2)
      || event.clientY >= getDragAreaHeight()
      || checkDragRegion(event.composedPath()) !== 'unhandled'
    ) return

    event.preventDefault()
    event.stopImmediatePropagation()

    if (event.detail === 2) {
      doubleClickPosition = [event.clientX, event.clientY]
      return
    }

    invoke('start_dragging')
  }, true)

  document.addEventListener('mouseup', (event) => {
    if (event.button !== 0 || event.detail !== 2) return

    const position = doubleClickPosition
    doubleClickPosition = null

    if (!position) return
    if (event.clientX !== position[0] || event.clientY !== position[1]) return
    if (checkDragRegion(event.composedPath()) !== 'unhandled') return

    invoke('internal_toggle_maximize')
  }, true)
}

export function initMacosTitlebar() {
  if (!isMacosOverlayTitlebar()) return

  let titlebarHeight = FALLBACK_TITLEBAR_HEIGHT

  const measureTitlebar = () => {
    titlebarHeight = FALLBACK_TITLEBAR_HEIGHT

    for (const selector of TITLEBAR_SELECTORS) {
      const titlebar = document.querySelector(selector)
      const bottom = titlebar?.getBoundingClientRect().bottom ?? 0

      if (bottom > 0) {
        titlebarHeight = bottom
        return
      }
    }
  }

  const keepTitlebarDraggable = () => {
    document.querySelectorAll(TITLEBAR_SELECTORS.join(', ')).forEach(enableWindowDrag)
    measureTitlebar()
  }

  keepTitlebarDraggable()

  let throttle: number | undefined

  new MutationObserver(() => {
    if (throttle) return

    throttle = window.setTimeout(() => {
      throttle = undefined
      keepTitlebarDraggable()
    }, 250)
  }).observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ['class'],
  })

  window.addEventListener('resize', measureTitlebar)

  initWindowDragFallback(() => titlebarHeight)
}

export function initLoadingWindowDrag(loadingContainer: HTMLElement) {
  if (document.documentElement.dataset.dorionPlatform !== 'macos') return
  if (window.__DORION_CONFIG__?.use_native_titlebar) return

  initWindowDragFallback(() =>
    loadingContainer.isConnected ? window.innerHeight : 0
  )
}
