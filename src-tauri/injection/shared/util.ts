export function cssSanitize(css: string) {
  const style = document.createElement('style')
  style.innerHTML = css

  document.body.appendChild(style)

  if (!style.sheet) return

  const result = Array.from(style.sheet.cssRules).map(rule => rule.cssText || '').join('\n')

  document.body.removeChild(style)
  return result
}

export async function timeout(ms: number) {
  return new Promise(r => setTimeout(r, ms))
}

export function isJson(s: string) {
  try {
    JSON.parse(s)
  } catch (_e) {
    return false
  }
  return true
}

/**
 * Sorta yoinked from https://github.com/uwu/shelter/blob/main/packages/shelter/src/index.ts
 */
export async function waitForApp() {
  // Ensure appMount exists
  const appMount = document.querySelector('#app-mount')

  if (!appMount) {
    setTimeout(waitForApp, 100)
    return
  }

  while (appMount.childElementCount === 0)
    await new Promise((r) => setTimeout(r, 100))

  return appMount
}

export async function waitForElm(selector: string) {
  const elm = document.querySelector(selector)

  if (!elm) {
    await timeout(100)
    return
  }

  return elm
}

export async function fetchImage(url: string) {
  const { invoke } = window.__TAURI__.core
  return await invoke('fetch_image', { url })
}

export async function saferEval(code: string) {
  return eval?.(`"use strict";${code}`)
}