export function applyNotificationCount() {
  // Check if we should update
  if (!window.Dorion.shouldShowUnreadBadge) return

  const { invoke } = window.__TAURI__.core
  let notifs = 0

  if (document.title.startsWith('•')) {
    notifs = -1
  } else {
    const matches = document.title.match(/\((.*?)\)/g)

    if (matches) {
      try {
        notifs = parseInt(matches[matches.length - 1].replace('(', '').replace(')', ''), 10)
      } catch (e) {
        console.error(e)
      }
    }
  }

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
