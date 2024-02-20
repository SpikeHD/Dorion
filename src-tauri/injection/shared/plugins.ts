export async function ensurePlugins() {
  const requiredPlugins = {
    'Dorion Settings':
      'https://spikehd.github.io/shelter-plugins/dorion-settings/',
    'Always Trust': 'https://spikehd.github.io/shelter-plugins/always-trust/',
    'Dorion Notifications':
      'https://spikehd.github.io/shelter-plugins/dorion-notifications/',
    'Dorion Streamer Mode':
      'https://spikehd.github.io/shelter-plugins/dorion-streamer-mode/',
    'Dorion Updater':
      'https://spikehd.github.io/shelter-plugins/dorion-updater/',
    'Dorion PTT': 'https://spikehd.github.io/shelter-plugins/dorion-ptt/',
    'Dorion Tray': 'https://spikehd.github.io/shelter-plugins/dorion-tray/',
    'Dorion Fullscreen':
      'https://spikehd.github.io/shelter-plugins/dorion-fullscreen/',
  }

  if (window.__DORION_CONFIG__?.client_plugins !== null && !window.__DORION_CONFIG__?.client_plugins) {
    console.log('[Ensure Plugins] Client plugins are disabled. I hope you know what you are doing!')
    return
  }

  const promises = [
    ...Object.entries(requiredPlugins).map(async ([name, url]) => {
      const res = await fetch(`${url}/plugin.js`)
      const text = await res.text()

      // Eval
      try {
        console.log('[Ensure Plugins] Loading plugin: ', name)

        // Create a new plugin object. Simpler version of https://github.com/uwu/shelter/blob/ac74061864479ecb688ae5efc321e981cd1b54fa/packages/shelter/src/plugins.tsx#L54
        const pluginStr = `shelter=>{return ${text}}${atob('Ci8v')}`
        const fn = window.eval(pluginStr)
        const plugin = fn(window.shelter)

        // Run plugin.onLoad if it exists
        plugin.onLoad?.()
      } catch (e) {
        console.error(`[Ensure Plugins] Failed to load plugin ${name}: `, e)
      }
    }),
  ]

  await Promise.all(promises)
}