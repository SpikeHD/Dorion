const { invoke } = window.__TAURI__.core

const currentlyPressed = new Set<string>()

export async function initWindowsKeybinds() {
  try {
    const platform = await invoke('get_platform')
    if (platform !== 'windows') return
  } catch {
    return
  }

  document.addEventListener('keydown', handleKeyDown)
  document.addEventListener('keyup', handleKeyUp)
}

function handleKeyDown(event: KeyboardEvent) {
  currentlyPressed.add(event.code)

  const keys = Array.from(currentlyPressed).map(code => ({
    name: getDisplayName(code),
    code: code
  }))

  invoke('trigger_keys_pressed', { keys, pressed: true }).catch(() => {})
}

function handleKeyUp(event: KeyboardEvent) {
  const keys = Array.from(currentlyPressed).map(code => ({
    name: getDisplayName(code),
    code: code
  }))

  invoke('trigger_keys_pressed', { keys, pressed: false }).catch(() => {})

  currentlyPressed.delete(event.code)
}

function getDisplayName(code: string): string {
  const names: Record<string, string> = {
    'ControlLeft': 'Ctrl', 'ControlRight': 'Ctrl',
    'ShiftLeft': 'Shift', 'ShiftRight': 'Shift',
    'AltLeft': 'Alt', 'AltRight': 'Alt',
    'MetaLeft': 'Meta', 'MetaRight': 'Meta',
    'Space': 'Space'
  }

  if (code.startsWith('Key') && code.length === 4) {
    return code.slice(3)
  }

  return names[code] || code
}
