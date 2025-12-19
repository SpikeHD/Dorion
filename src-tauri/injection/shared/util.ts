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
 * Observes the DOM for newly added nodes and executes a callback for each.
 * @template T - The type of the value that the Promise will eventually resolve to.
 * @param {Node} rootElm - The DOM node to start observing from (e.g., document.body).
 * @param {(Node, resolve(T)) => boolean} callbackFn - A function executed for every added node.
 * - Call `resolve(value)` to fulfill the promise.
 * - Return `false` to disconnect the observer and stop listening.
 * - Return `true` to continue observing for more nodes.
 * * @returns {Promise<T>} A promise that resolves when `resolve()` is called within the callback.
 * * @example
 * const target = await observeDom<HTMLDivElement>(document.body, (node, resolve) => {
 *   if (node instanceof HTMLDivElement && node.id === 'my-element') {
 *     resolve(node);
 *     return false; // Found it, stop observing
 *   }
 *   return true; // Keep looking
 * });
 */
function observeDom<T>(rootElm: Node, callbackFn: (node: Node, resolve: (value: T) => void) => boolean, subtree: boolean): Promise<T> {
  return new Promise((resolve) => {
    const observer = new MutationObserver(mutations => {
      for (const mutation of mutations) {
        if (mutation.type === 'childList') {
          const addedNodes = Array.from(mutation.addedNodes)
          for (const node of addedNodes) {
            if (!callbackFn(node, resolve)) {
              observer.disconnect()
              return
            }
          }
        }
      }
    })
    observer.observe(rootElm, {
      childList: true,
      subtree // reduce callback count for perf
    })
  })
}

// Ensure at least one element on the chain would callback
type query = Array<string> | string
type waitCfg = { callbackFn: null | ((elm: Element) => void), root: Element, timeout: number }
const isString = (v: any) => typeof v === 'string' || v instanceof String
const subtreeFind = (p: Element, q: Array<string>) => Array.from(p.children).find(c => q.some(q => c.matches(q)))
const queryFind = (p: Element, query: Array<string>) => {
  for (let q of query) {
    const subtree = q[0] === '>'
    if (subtree) q = q.slice(1)
    const elm = subtree ? subtreeFind(p, [q]) : p.querySelector(q)
    if (elm) return elm
  }
  return
}
export async function waitForElmEx(queries: Array<query> | query, cfg: Partial<waitCfg> = {}): Promise<Element> {
  const callbackFn = cfg.callbackFn;
  let root = typeof cfg.root !== 'undefined' ? cfg.root : document.body;
  const timeout = cfg.timeout;

  let query: string[]
  let stop = false;
  if (timeout) setTimeout(() => { stop = true }, timeout)

  if (isString(queries)) queries = [queries]
  loop: while (queries.length) {
    // prepare query
    const q = queries.shift()
    if (!q) break
    query = isString(q) ? [q] : q
    const subtree = query.every(q => q[0] === '>')
    if (subtree) query = query.map(q => q.slice(1))
    // no observe if this elm already exist
    const elm = subtree ? subtreeFind(root, query) : queryFind(root, query)
    if (elm) { root = elm; if (callbackFn) callbackFn(root); continue loop }
    // start observer
    root = await observeDom(root, (node, res) => {
      if (stop) { res(root); return false }
      if (node.nodeType !== Node.ELEMENT_NODE) return true
      const e = node as Element
      for (let q of query) {
        if (!subtree) {
          const s = q[0] === '>'
          if (s) q = q.slice(1)
        }
        let ret = e.matches(q) ? e : null
        if (!ret) {
          ret = e.querySelector(q)
        }
        if (ret) {
          res(e)
          return false
        }
      }
      return true
    }, subtree) as Element
    // callback after found
    if (callbackFn) callbackFn(root)
  }
  return root
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
