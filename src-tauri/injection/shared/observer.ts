import { handleTopBar } from './ui'

let isLogin = false

const navObserver = new MutationObserver((_m) => {
  // If we were on login, and now we are not
  if (isLogin && !window.location.href.includes('login')) {
    isLogin = false
    handleTopBar()
  }

  // If we were not on login, and now we are
  if (!isLogin && window.location.href.includes('login')) {
    isLogin = true
    handleTopBar()
  }
})

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

const titleKeepArounder = () => {
  // This sucks but mutation-observing the entire app is definitely worse
  setInterval(() => {
    const top = document.querySelector('#dorion_topbar')
    if (!top) handleTopBar()
  }, 1000)
}

export {
  navObserver,
  observeForElement,
  titleKeepArounder
}
