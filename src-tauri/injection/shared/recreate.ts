import { isJson } from './util'

export async function proxyFetch() {
  window.nativeFetch = window.fetch

  const extensionInjected = await window.__TAURI__.core.invoke('extension_injected')

  console.log('[Proxy Fetch] Extension injected: ', extensionInjected)

  window.fetch = async (url, options) => {
    const { http } = window.__TAURI__
    const discordReg = /https?:\/\/(?:[a-z]+\.)?(?:discord\.com|discordapp\.com)(?:\/.*)?/g
    const scienceReg = /\/api\/v.*\/(science|track)/g

    // If it matches, just let it go through native OR its a relative URL
    if (extensionInjected || url.toString().match(discordReg) || url.toString().startsWith('ipc://') || url.toString().startsWith('/')) {
      // Block science though!
      if (url.toString().match(scienceReg)) {
        console.log(`[Fetch Proxy] Blocked URL: ${url}`)
        return
      }

      return window.nativeFetch(url, options)
    }

    // If there is an options.body, check if it's valid JSON. if so, set that up
    if (options && options?.body) {
      const bodyContent = isJson(String(options.body)) ? http.Body.json(options.body) : typeof options.body === 'string' ? http.Body.text(options.body) : http.Body.bytes(options.body)
      options.body = bodyContent
    }

    if (options && options?.headers) {
      // Check if header object, if so convert back to Record<String, any>
      if (options.headers instanceof Headers) {
        const headers = {}

        // @ts-expect-error Headers is iterable
        for (const [key, value] of options.headers.entries()) {
          // @ts-expect-error Headers is iterable
          headers[key] = value
        }

        options.headers = headers
      }
    }

    const response = await http.fetch(url, {
      responseType: 3,
      ...options
    }).catch((e: Error) => console.error('[Proxy Fetch] Failed to fetch: ', e))

    return response
  }
}

export function proxyXHR() {
  const open = XMLHttpRequest.prototype.open

  XMLHttpRequest.prototype.open = function(...args: unknown[]) {
    const [_method, url] = args
    const rgx = /\/api\/v.*\/(science|track)/g

    if (String(url).match(rgx)) {
      console.log(`[XHR Blocker] Blocked URL: ${url}`)
      return
    }

    // @ts-expect-error this is fine
    open.apply(this, args)
  }
}

export function proxyAddEventListener() {
  const original = window.addEventListener

  window.addEventListener = function(...args: Parameters<typeof window.addEventListener>) {
    const [type, listener] = args
    if (type === 'beforeunload') {
      args[1] = (...listenerArgs: Parameters<EventListener>) => {
        // @ts-expect-error this is fine
        const isTrustedOverwrite = listenerArgs[0]?.isTrustedOverwrite

        if (isTrustedOverwrite !== undefined) {
          const event = listenerArgs[0]
          listenerArgs[0] = new Proxy(event, {
            get(target, prop, receiver) {
              if (prop === 'isTrusted') return isTrustedOverwrite
              return Reflect.get(target, prop, receiver)
            }
          })
        }

        return ('handleEvent' in listener) ? listener.handleEvent(...listenerArgs) : listener(...listenerArgs)
      }
    }

    return original(...args)
  }
}

const INTERNAL_DOMAINS = [
  'discord.com',
  'discordapp.com',
  'cdn.discordapp.com'
]

const LOCAL_PROTOCOLS = ['about:', 'blob:', 'data:', 'javascript:']

function isExternal(url: string) {
  try {
    if (url.startsWith('/')) return false

    const parsed = new URL(url)

    if (LOCAL_PROTOCOLS.includes(parsed.protocol)) return false

    if (parsed.protocol === 'http:' || parsed.protocol === 'https:') {
      return !INTERNAL_DOMAINS.some(domain => parsed.hostname.endsWith(domain))
    }

    return true
  } catch {
    return false
  }
}

function closedPopup(url: string): Window {
  const obj = {
    url,
    closed: false,
    name: '',
    title: ''
  }

  const stub = new Proxy(obj, {
    get(target, prop) {
      if (prop === 'closed') return target.closed
      if (prop === 'close' || prop === 'blur') {
        return () => {
          target.closed = true
        }
      }
      if (prop in target) return Reflect.get(target, prop)

      return () => stub
    },
    set<T>(target: T, prop: keyof T, value: T[keyof T]) {
      target[prop] = value
      return true
    }
  })

  return stub as unknown as Window
}

export function proxyOpen() {
  // Make window.open become window.__TAURI__.shell.open
  window.nativeOpen = window.open
  window.open = (url: string | undefined | URL, target?: string, features?: string) => {
    if (!url) {
      return window.nativeOpen(url, target, features)
    }

    const urlStr = url.toString()

    // If this needs to open externally, do so
    if ((target === '_blank' || !target) && isExternal(urlStr)) {
      const protocol = new URL(urlStr).protocol
      console.log(protocol === 'http:' || protocol === 'https:' ? '[Proxy Open] External URL:' : '[Proxy Open] App scheme:', urlStr)

      window.__TAURI__.shell.open(urlStr).catch((e) => console.error('[Proxy Open] Failed to open externally: ', e))
      return closedPopup(urlStr)
    }

    console.log('[Proxy Open] In-app URL:', urlStr)

    // Otherwise, use the native open
    return window.nativeOpen(urlStr, target, features)
  }
}

export function badPostMessagePatch() {
  // this should support all OS
  // @ts-expect-error shut up
  window.__TAURI_POST_MESSAGE__ = () => {
    return null
  }
}

export function proxyNotification() {
  let permVal = 'granted'

  // @ts-expect-error shut up
  window.nativeNotification = window.Notification

  // @ts-expect-error shut up
  window.Notification = function(...args) {
    // Stub this
  }

  window.Notification.requestPermission = async () => 'granted'
  // For checking if we have stubbed
  Object.defineProperty(window.Notification, '__IS_STUBBED__', {
    enumerable: true,
    value: true
  })

  Object.defineProperty(window.Notification, 'permission', {
    enumerable: true,
    get: () => permVal,
    set: (v) => {
      permVal = v
    }
  })
}
