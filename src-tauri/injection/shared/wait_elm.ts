// Observes the DOM for newly added nodes and executes a callback for each.
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
      subtree
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