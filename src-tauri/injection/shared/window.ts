/**
 * Functions for window controls
 */
export function close() {
  window.__TAURI__.core.invoke('close')
}

export function minimize() {
  window.__TAURI__.core.invoke('minimize')
}

export function toggleMaximize() {
  window.__TAURI__.core.invoke('toggle_maximize')
}

export async function setMaximizeIcon() {
  if (await window.__TAURI__.webviewWindow.getCurrentWebviewWindow().isMaximized()) {
    const topmax = document.querySelector('#topmax') as HTMLDivElement
    topmax.classList.add('maximized')
  } else {
    const topmax = document.querySelector('#topmax') as HTMLDivElement
    topmax.classList.remove('maximized')
  }
}

export function applyNotificationCount() {
  // Check if we should update
  if (!window.Dorion.shouldShowUnreadBadge) return

  const { invoke } = window.__TAURI__.core
  const title = document.querySelector('title') as HTMLTitleElement
  const notifs = title.innerHTML.startsWith('•') ? -1 : title.innerHTML?.match(/\((.*)\)/)?.[1]

  if (!notifs) {
    invoke('notification_count', {
      amount: 0,
    })

    return
  }

  invoke('notification_count', {
    amount: Number(notifs),
  })
}
