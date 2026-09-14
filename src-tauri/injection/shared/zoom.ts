async function stepZoom(dir: number) {
  const { invoke } = window.__TAURI__.core
  const cur = parseFloat(window.__DORION_CONFIG__?.zoom ?? '1.0') || 1.0
  let next: number

  if (dir === 0) {
    next = 1.0
  } else if (dir > 0) {
    next = Math.min(2.0, Math.round((cur + 0.1) * 100) / 100)
  } else {
    next = Math.max(0.5, Math.round((cur - 0.1) * 100) / 100)
  }

  await invoke('window_zoom_level', { value: next })
    .then(() => {
      if (window.__DORION_CONFIG__) {
        window.__DORION_CONFIG__.zoom = String(next)
      }
    })
    .catch(() => {})
}

export function initZoomHotkeys() {
  document.addEventListener('keydown', async (e: KeyboardEvent) => {
    if (!(e.ctrlKey || e.metaKey)) return

    const isPlus = e.code === 'Equal' || e.code === 'NumpadAdd' || e.key === '+' || e.key === '='
    const isMinus = e.code === 'Minus' || e.code === 'NumpadSubtract' || e.key === '-'
    const isReset = e.code === 'Digit0' || e.code === 'Numpad0' || e.key === '0'

    if (!isPlus && !isMinus && !isReset) return

    e.preventDefault()

    if (isReset) {
      await stepZoom(0)
    } else if (isPlus) {
      await stepZoom(1)
    } else {
      await stepZoom(-1)
    }
  })

  document.addEventListener('wheel', async (e: WheelEvent) => {
    if (!(e.ctrlKey || e.metaKey) || e.deltaY === 0) return

    e.preventDefault()
    await stepZoom(e.deltaY < 0 ? 1 : -1)
  }, { passive: false })
}
