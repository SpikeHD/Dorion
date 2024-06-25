import { isJson } from './util'

export function proxyFetch() {
  window.nativeFetch = window.fetch

  window.fetch = async (url, options) => {
    const { http } = window.__TAURI__
    const discordReg = /https?:\/\/(?:[a-z]+\.)?(?:discord\.com|discordapp\.com)(?:\/.*)?/g
    const scienceReg = /\/api\/v.*\/(science|track)/g

    // If it matches, just let it go through native OR its a relative URL
    if (url.toString().match(discordReg) || url.toString().startsWith('/')) {
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
    })

    // Adherence to what most scripts will expect to have available when they are using fetch(). These have to pretend to be promises
    response.json = async () => JSON.parse(await response.text())
    response.text = async () => {
      // Decode binary array to string
      return response.data.reduce((data: string, byte: number) => data + String.fromCharCode(byte), '')
    }
    response.arrayBuffer = async () => {
      // Create a new arraybuffer
      const buffer = new ArrayBuffer(response.data.length)
      const view = new Uint8Array(buffer)

      // Copy the data over
      response.data.forEach((byte: number, i: number) => view[i] = byte)

      return buffer
    }

    response.headers = new Headers(response.headers)

    return response
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
  // @ts-expect-error shut up
  const nativePostMessage = window?.chrome?.webview?.postMessage

  if (!nativePostMessage) {
    // overwrite window.chrome
    Object.defineProperty(window, 'chrome', {
      value: {
        webview: {
          postMessage: () => {
            // we just gonna have to deal with this not existing
            return null
          }
        }
      }
    })
  }
}