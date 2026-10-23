type DisplayAudioConstraints = MediaTrackConstraints & {
  restrictOwnAudio?: ConstrainBoolean
}

type DisplayAudioOptions = Omit<DisplayMediaStreamOptions, 'audio'> & {
  audio?: boolean | DisplayAudioConstraints
  systemAudio?: 'include' | 'exclude'
  windowAudio?: 'window' | 'system' | 'exclude'
}

/** Set Windows display-audio defaults before client mods retain the capture function. */
export function configureDisplayMedia() {
  if (!navigator.platform.startsWith('Win')) return

  const devices = navigator.mediaDevices
  if (!devices?.getDisplayMedia || !devices.getSupportedConstraints) return

  const supported = devices.getSupportedConstraints() as MediaTrackSupportedConstraints & {
    restrictOwnAudio?: boolean
  }
  if (supported.restrictOwnAudio !== true) return

  devices.getDisplayMedia = new Proxy(devices.getDisplayMedia, {
    apply(target, receiver, args: unknown[]) {
      const options = args[0] as DisplayAudioOptions | undefined
      if (!options || typeof options !== 'object') return Reflect.apply(target, receiver, args)

      const audio = options.audio
      if (audio !== true && (typeof audio !== 'object' || audio === null)) {
        return Reflect.apply(target, receiver, args)
      }

      const nextArgs = [...args]
      nextArgs[0] = {
        ...options,
        systemAudio: options.systemAudio === undefined ? 'include' : options.systemAudio,
        // Vencord's mute control excludes system audio without setting audio:false.
        windowAudio: options.windowAudio === undefined
          ? (options.systemAudio === 'exclude' ? 'exclude' : 'window')
          : options.windowAudio,
        audio: {
          ...(typeof audio === 'object' ? audio : {}),
          restrictOwnAudio: true,
        } satisfies DisplayAudioConstraints,
      } satisfies DisplayAudioOptions

      return Reflect.apply(target, receiver, nextArgs)
    }
  })
}
