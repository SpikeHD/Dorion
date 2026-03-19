import { applyExtraCSS } from './shared/ui'
import { initWindowsKeybinds } from './shared/windows_keybinds'

declare global {
  interface Window {
    __DORION_AUDIO_FIX_INSTALLED__?: boolean
  }
}

type MaybePC = RTCPeerConnection & {
  __dorionAudioFixTrackListener__?: boolean
}

const REARM_DELAY_MS = 120

function isWindowsDorion() {
  return document.documentElement.getAttribute('data-dorion-platform') === 'windows'
}

function audioRecoveryEnabled() {
  return window.__DORION_CONFIG__.audio_recovery_fix !== false
}

function audioRecoveryDebugEnabled() {
  return window.__DORION_CONFIG__.audio_recovery_debug === true
}

function audioRecoveryLog(message: string, ...args: unknown[]) {
  if (!audioRecoveryDebugEnabled()) return
  console.debug(`[Dorion Audio Recovery] ${message}`, ...args)
}

function getRemoteAudioElements(): HTMLMediaElement[] {
  const els = Array.from(document.querySelectorAll('audio')) as HTMLAudioElement[]

  return els.filter(el => {
    const stream = el.srcObject
    if (!(stream instanceof MediaStream)) return false

    const audioTracks = stream.getAudioTracks()
    if (!audioTracks.length) return false

    return audioTracks.some(track => track.readyState === 'live')
  })
}

async function rearmAudioElement(el: HTMLMediaElement) {
  if (!el.isConnected) return

  const stream = el.srcObject
  if (!(stream instanceof MediaStream)) return

  const wasPaused = el.paused
  const currentTime = Number.isFinite(el.currentTime) ? el.currentTime : 0

  try {
    el.srcObject = null
    el.srcObject = stream

    if (currentTime > 0) {
      try {
        el.currentTime = currentTime
      } catch {
        // no-op
      }
    }

    if (!wasPaused || el.autoplay) {
      await el.play().catch(() => { })
    }
  } catch (error) {
    audioRecoveryLog('Failed to rearm audio element', error)
  }
}

function scheduleAudioRecovery(reason: string) {
  audioRecoveryLog('Scheduling recovery', reason)

  window.setTimeout(() => {
    const elements = getRemoteAudioElements()
    audioRecoveryLog('Recovering elements', elements.length, reason)

    for (const el of elements) {
      void rearmAudioElement(el)
    }
  }, REARM_DELAY_MS)
}

function attachTrackListeners(pc: MaybePC) {
  if (pc.__dorionAudioFixTrackListener__) return
  pc.__dorionAudioFixTrackListener__ = true

  pc.addEventListener('track', ev => {
    const track = ev.track
    if (track.kind !== 'audio') return

    audioRecoveryLog('Audio track event attached', track.id)

    track.addEventListener('mute', () => {
      scheduleAudioRecovery('track-mute')
    })
    track.addEventListener('unmute', () => {
      scheduleAudioRecovery('track-unmute')
    })
    track.addEventListener('ended', () => {
      scheduleAudioRecovery('track-ended')
    })

    scheduleAudioRecovery('track-added')
  })

  pc.addEventListener('connectionstatechange', () => {
    audioRecoveryLog('Connection state changed', pc.connectionState)

    if (pc.connectionState === 'connected') {
      scheduleAudioRecovery('connection-connected')
    }
  })
}

function installPermanentAudioMitigation() {
  if (!isWindowsDorion()) return
  if (!audioRecoveryEnabled()) return
  if (window.__DORION_AUDIO_FIX_INSTALLED__) return
  if (!window.RTCPeerConnection) return

  window.__DORION_AUDIO_FIX_INSTALLED__ = true

  const OriginalRTCPeerConnection = window.RTCPeerConnection

  window.RTCPeerConnection = class extends OriginalRTCPeerConnection {
    constructor(configuration?: RTCConfiguration) {
      super(configuration)
      attachTrackListeners(this as MaybePC)
    }
  } as typeof RTCPeerConnection

  const originalSetRemoteDescription = OriginalRTCPeerConnection.prototype.setRemoteDescription

  OriginalRTCPeerConnection.prototype.setRemoteDescription = async function(
    ...args: Parameters<RTCPeerConnection['setRemoteDescription']>
  ) {
    const result = await originalSetRemoteDescription.apply(this, args)
    attachTrackListeners(this as MaybePC)
    scheduleAudioRecovery('set-remote-description')
    return result
  }

  audioRecoveryLog('Installed audio recovery mitigation')
}

(async () => {
  console.log('Discord is loaded!')

  // Ensure top bar exists if we want it
  if (window.__DORION_CONFIG__.use_native_titlebar)
    window.__TAURI__.core.invoke('set_decorations', { enable: true }).catch(_e => { }) // This is allowed to fail

  initWindowsKeybinds()
  installPermanentAudioMitigation()
  // Load up our extra css
  applyExtraCSS()

  // The comment ahead is read by tauri and used to insert theme injection code

  /*! __THEMES__ */
})()
