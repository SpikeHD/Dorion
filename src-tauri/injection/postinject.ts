import { applyExtraCSS } from './shared/ui'
import { initWindowsKeybinds } from './shared/windows_keybinds'

const DORION_AUDIO_REFRESH_ATTR = '__DORION_AUDIO_REFRESH_PATCHED__'
let audioRefreshTimer: number | undefined

function scheduleVoiceMediaRefresh(delay = 150) {
  window.clearTimeout(audioRefreshTimer)
  audioRefreshTimer = window.setTimeout(() => {
    refreshVoiceMediaElements()
  }, delay)
}

function refreshVoiceMediaElements() {
  const mediaElements = Array.from(document.querySelectorAll('audio, video')) as HTMLMediaElement[]

  for (const mediaElement of mediaElements) {
    if (!mediaElement.isConnected) continue
    if (!mediaElement.srcObject && !mediaElement.currentSrc && !mediaElement.src) continue

    if (mediaElement instanceof HTMLVideoElement && mediaElement.muted) continue

    const srcObject = mediaElement.srcObject
    const currentSrc = mediaElement.currentSrc || mediaElement.src
    const currentTime = Number.isFinite(mediaElement.currentTime) ? mediaElement.currentTime : 0
    const shouldResumePlayback = !mediaElement.paused || mediaElement.autoplay

    try {
      if (srcObject) {
        mediaElement.srcObject = null
        mediaElement.srcObject = srcObject
      } else if (currentSrc) {
        mediaElement.src = ''
        mediaElement.src = currentSrc
      }

      if (currentTime > 0) {
        mediaElement.currentTime = currentTime
      }

      if (shouldResumePlayback) {
        mediaElement.play().catch(() => { })
      }
    } catch (_error) {
      // ignored on purpose, this is a best-effort playback recovery
    }
  }
}

function initWindowsVoiceMediaRecovery() {
  if (document.documentElement.getAttribute('data-dorion-platform') !== 'windows') return

  const patchedWindow = window as Window & Record<string, boolean>
  if (patchedWindow[DORION_AUDIO_REFRESH_ATTR]) return
  patchedWindow[DORION_AUDIO_REFRESH_ATTR] = true

  const peerConnectionPrototype = window.RTCPeerConnection?.prototype
  const originalSetRemoteDescription = peerConnectionPrototype?.setRemoteDescription

  if (peerConnectionPrototype && originalSetRemoteDescription) {
    peerConnectionPrototype.setRemoteDescription = async function(...args: Parameters<RTCPeerConnection['setRemoteDescription']>) {
      const result = await originalSetRemoteDescription.apply(this, args)
      scheduleVoiceMediaRefresh()
      return result
    }
  }

  const observer = new MutationObserver(mutations => {
    const shouldRefresh = mutations.some(({ addedNodes, removedNodes }) => {
      const nodes = [...addedNodes, ...removedNodes]
      return nodes.some(node => {
        if (!(node instanceof Element)) return false
        return node.matches('audio, video') || !!node.querySelector('audio, video')
      })
    })

    if (shouldRefresh) {
      scheduleVoiceMediaRefresh(250)
    }
  })

  observer.observe(document.body, {
    childList: true,
    subtree: true,
  })
}

(async () => {
  console.log('Discord is loaded!')

  // Ensure top bar exists if we want it
  if (window.__DORION_CONFIG__.use_native_titlebar)
    window.__TAURI__.core.invoke('set_decorations', { enable: true }).catch(_e => { }) // This is allowed to fail

  initWindowsKeybinds()
  initWindowsVoiceMediaRecovery()
  // Load up our extra css
  applyExtraCSS()

  // The comment ahead is read by tauri and used to insert theme injection code

  /*! __THEMES__ */
})()
