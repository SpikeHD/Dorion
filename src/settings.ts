document.addEventListener("DOMContentLoaded", async () => {
  const { invoke } = window.__TAURI__;
  const themes = await invoke("get_theme_names") as string[]
  const themeSelect = document.querySelector("#themeSelect")

  themes.forEach(theme => {
    theme = theme.replace(/"/g, '')
    const opt = document.createElement('option')

    opt.value = theme
    opt.innerHTML = theme

    themeSelect?.appendChild(opt)
  })
})

// Prevent any fuckery within themes
function css_sanitize(css: string) {
  const style = document.createElement('style');
  style.innerHTML = css;

  document.head.appendChild(style);

  if (!style.sheet) return

  const result = Array.from(style.sheet.cssRules).map(rule => rule.cssText || '').join('\n');

  document.head.removeChild(style);
  return result;
}