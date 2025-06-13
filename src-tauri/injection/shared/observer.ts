const observeForElement = (selector: string) => {
  return new Promise(r => {
    const el = document.querySelector(selector)
    if (el) {
      r(el)
      return
    }

    new MutationObserver((_m, observer) => {
      Array.from(document.querySelectorAll(selector)).forEach((element) => {
        r(element)
        observer.disconnect()
      })
    })
      .observe(document.documentElement, {
        childList: true,
        subtree: true
      })
  })
}

export {
  observeForElement,
}
