type ResizeDirection =
  | 'East'
  | 'North'
  | 'NorthEast'
  | 'NorthWest'
  | 'South'
  | 'SouthEast'
  | 'SouthWest'
  | 'West'

interface ResizableWindow {
  isDecorated: () => Promise<boolean>
  startResizeDragging: (direction: ResizeDirection) => Promise<void>
}

interface TauriGlobal {
  window?: { getCurrentWindow?: () => unknown }
  webviewWindow?: { getCurrentWebviewWindow?: () => unknown }
}

const EDGE_SIZE = 8
const RESIZE_STYLE_ID = 'dorion-resize-cursor'
const RESIZE_CURSORS: Record<ResizeDirection, string> = {
  East: 'ew-resize',
  West: 'ew-resize',
  North: 'ns-resize',
  South: 'ns-resize',
  NorthEast: 'nesw-resize',
  NorthWest: 'nwse-resize',
  SouthEast: 'nwse-resize',
  SouthWest: 'nwse-resize',
}

function getResizableWindow(): ResizableWindow | null {
  const tauri = (window as unknown as { __TAURI__?: TauriGlobal }).__TAURI__

  if (!tauri) return null

  const win =
    tauri.window?.getCurrentWindow?.() ?? tauri.webviewWindow?.getCurrentWebviewWindow?.()

  return win == null ? null : (win as ResizableWindow)
}

function directionAt(x: number, y: number): ResizeDirection | null {
  const { innerWidth, innerHeight } = window

  const north = y <= EDGE_SIZE
  const south = y >= innerHeight - 1 - EDGE_SIZE
  const west = x <= EDGE_SIZE
  const east = x >= innerWidth - 1 - EDGE_SIZE

  if (north && west) return 'NorthWest'
  if (north && east) return 'NorthEast'
  if (south && west) return 'SouthWest'
  if (south && east) return 'SouthEast'
  if (north) return 'North'
  if (south) return 'South'
  if (west) return 'West'
  if (east) return 'East'

  return null
}

function setResizeCursor(direction: ResizeDirection | null) {
  const html = document.documentElement

  if (!direction) {
    html.classList.remove('dorion-resizing')
    document.getElementById(RESIZE_STYLE_ID)?.remove()
    return
  }

  if (!document.getElementById(RESIZE_STYLE_ID)) {
    const style = document.createElement('style')
    style.id = RESIZE_STYLE_ID
    document.head.appendChild(style)
  }

  const style = document.getElementById(RESIZE_STYLE_ID) as HTMLStyleElement
  style.textContent = `
    html.dorion-resizing * {
      cursor: ${RESIZE_CURSORS[direction]} !important;
      user-select: none !important;
    }
  `
  html.classList.add('dorion-resizing')
}

function attachResizeHandlers(win: ResizableWindow) {
  let activeDirection: ResizeDirection | null = null

  document.addEventListener(
    'mousemove',
    (event) => {
      const direction = directionAt(event.clientX, event.clientY)

      if (direction !== activeDirection) {
        activeDirection = direction
        setResizeCursor(direction)
      }
    },
    true,
  )

  document.addEventListener('mouseleave', () => {
    activeDirection = null
    setResizeCursor(null)
  })

  document.addEventListener(
    'mousedown',
    (event) => {
      if (event.button !== 0 || event.altKey || event.ctrlKey || event.shiftKey || event.metaKey) {
        return
      }

      const direction = directionAt(event.clientX, event.clientY)
      if (!direction) return

      event.preventDefault()
      event.stopPropagation()
      win.startResizeDragging(direction).catch(() => {})

      activeDirection = null
      setResizeCursor(null)
    },
    true,
  )
}

export function initResizeHelpers() {
  if (document.documentElement.getAttribute('data-dorion-platform') !== 'linux') return

  const win = getResizableWindow()
  if (!win) return

  win
    .isDecorated()
    .then((decorated) => {
      if (!decorated) attachResizeHandlers(win)
    })
    .catch(() => {})
}
