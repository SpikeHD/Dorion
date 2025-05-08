import { handleTopBar } from './ui'

let isLogin = false

const navObserver = new MutationObserver(function(_m) {
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

export default navObserver
