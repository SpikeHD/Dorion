import { isJson } from './util'

export function proxyFetch() {
  window.nativeFetch = window.fetch

  window.fetch = async (url, options) => {
    const { http } = window.__TAURI__
    const discordReg = /https?:\/\/(?:[a-z]+\.)?(?:discord\.com|discordapp\.com)(?:\/.*)?/g
    const scienceReg = /\/api\/v.*\/(science|track)/g

    // If it matches, just let it go through native OR its a relative URL
    if (url.toString().match(discordReg) || url.toString().startsWith('ipc://') || url.toString().startsWith('/')) {
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

export function proxyOpen() {
  // Make window.open become window.__TAURI__.shell.open
  window.nativeOpen = window.open
  window.open = (url: string | undefined | URL, target?: string, features?: string) => {
    // If this needs to open externally, do so
    if (target === '_blank' || !target) {
      window.__TAURI__.shell.open(url as string)
      return null
    }

    // Otherwise, use the native open
    return window.nativeOpen(url as string, target, features)
  }
}

export function createLocalStorage() {
  const iframe = document.createElement('iframe')

  // Wait for document.head to exist, then append the iframe
  const interval = setInterval(() => {
    if (!document.head || window.localStorage) return

    document.head.append(iframe)
    const pd = Object.getOwnPropertyDescriptor(iframe.contentWindow, 'localStorage')
    iframe.remove()

    if (!pd) return

    Object.defineProperty(window, 'localStorage', pd)

    console.log('[Create LocalStorage] Done!')

    clearInterval(interval)
  }, 50)
}

export function badPostMessagePatch() {
  // this should support all OS
  // @ts-expect-error shut up
  window.__TAURI_POST_MESSAGE__ = () => {
    return null
  }
}
