// Fixes a weird focus bug in the CEF
export function focusGuard() {
  window.addEventListener('focusout', (e) => {
    if (e.relatedTarget !== null) return
    const t = e.target as HTMLElement
    if (
      (t.tagName === 'INPUT' || t.tagName === 'TEXTAREA' || t.isContentEditable) &&
      t.closest('[role="dialog"]')
    ) {
      e.stopImmediatePropagation()
      requestAnimationFrame(() => t.focus())
    }
  }, true)
}
