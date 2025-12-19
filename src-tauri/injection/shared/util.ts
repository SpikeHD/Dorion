import { waitForElmEx } from "./wait_elm"

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

export function waitForDom() {
  return new Promise<void>((resolve) => {
    if (document.body) return resolve()
    document.addEventListener("DOMContentLoaded", () => {
      resolve()
    });
  })
}

/**
 * Ensure appMount exists
 * Sorta yoinked from https://github.com/uwu/shelter/blob/main/packages/shelter/src/index.ts
 */
export async function waitForApp() {
  return await waitForElmEx(['>div#app-mount', '>*'])
}

export async function waitForElm(selector: string, max: number | undefined = undefined) {
  return await waitForElmEx(selector, {timeout: max})
}

export async function fetchImage(url: string) {
  const { invoke } = window.__TAURI__.core
  return await invoke('fetch_image', { url })
}

export async function saferEval(code: string) {
  return eval?.(`"use strict";${code}`)
}
