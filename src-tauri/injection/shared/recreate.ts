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

    response.headers = new Headers(response.headers)

    return response
  }
}

export function proxyXHR() {
  const open = XMLHttpRequest.prototype.open
  
  XMLHttpRequest.prototype.open = function(...args: unknown[]) {
    // @ts-expect-error this is fine
    open.apply(this, args)

    const [_method, url] = args
    const send = this.send

    this.send = function() {
      const rgx = /\/api\/v.*\/(science|track)/g

      if (!String(url).match(rgx)) {
        // @ts-expect-error this is fine
        return send.apply(this, args)
      }

      console.log(`[XHR Blocker] Blocked URL: ${url}`)
    }
  }
}

export function createLocalStorage() {
  // Wait for document.head to exist, then append the iframe
  const interval = setInterval(() => {
    if (!document.head || window.localStorage) return
    window.localStorage = window.__localStorage

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