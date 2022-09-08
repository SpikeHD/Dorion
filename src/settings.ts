import { invoke } from "@tauri-apps/api";

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

  prefillConfig(
    JSON.parse(await invoke('read_config_file'))
  )

  initOnchangeHandlers()
  initOnclickHandlers()
})

function prefillConfig(config: Config) {
  const themeSelect = <HTMLSelectElement>document.querySelector("#themeSelect")
  const zoomSelect = <HTMLInputElement>document.querySelector("#zoomLevel")
  const zoomPct = document.querySelector('#zoomLevelValue')
  const clientType = <HTMLSelectElement>document.querySelector("#clientType")

  if (themeSelect) {
    themeSelect.value = config.theme
  }

  if (zoomSelect) {
    zoomSelect.value = `${Number(config.zoom) * 100}`
    if (zoomPct) zoomPct.innerHTML = `${Number(config.zoom) * 100}%`
  }

  if (clientType) {
    clientType.value = config.client_type
  }
}

function initOnchangeHandlers() {
  const themeSelect = document.querySelector("#themeSelect")
  const zoomSelect = document.querySelector("#zoomLevel")
  const clientType = document.querySelector("#clientType")

  themeSelect?.addEventListener('change', (evt) => {
    const tgt = <HTMLSelectElement>evt.target
    setConfigValue('theme', tgt.value)
  })

  zoomSelect?.addEventListener('change', (evt) => {
    const tgt = <HTMLSelectElement>evt.target
    setConfigValue('zoom', String(Number(tgt.value) / 100))

    invoke('change_zoom', {
      zoom: Number(tgt.value) / 100
    })
  })

  clientType?.addEventListener('change', (evt) => {
    const tgt = <HTMLSelectElement>evt.target
    setConfigValue('client_type', tgt.value)
  })
}

function initOnclickHandlers() {
  const openPlugins = document.querySelector("#openPlugins")
  const openThemes = document.querySelector("#openThemes")

  if (openPlugins) {
    openPlugins.addEventListener('click', () => {
      invoke('open_plugins')
    })
  }

  if (openThemes) {
    openThemes.addEventListener('click', () => {
      invoke('open_themes')
    })
  }
}

async function setConfigValue(key: keyof Config, val: string) {
  const cfg = JSON.parse(await invoke('read_config_file')) as Config
  cfg[key] = val

  await invoke('write_config_file', {
    contents: JSON.stringify(cfg)
  })
}
