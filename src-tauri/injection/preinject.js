const { invoke } = window.__TAURI__

;(async () => {
  console.log('yo')
  await displayLoadingTop()

  const config = JSON.parse(await invoke('read_config_file'))
  const plugins = await invoke('load_plugins');

  const imports = await invoke('localize_all_js', {
    urls: await invoke('get_plugin_import_urls', {
      pluginJs: plugins
    })
  });

  console.log('yo2')
  // Get theme if it exists
  let themeInjection = ''

  if (config?.theme !== 'none') {
    const themeContents = await invoke('get_theme', {
      name: config.theme
    })
  
    const localized = await invoke('localize_imports', {
      css: themeContents
    })

    console.log('yo3')

    // This will use the DOM in a funky way to validate the css, then we make sure to fix up quotes
    const cleanContents = cssSanitize(localized)?.replaceAll('\\"', '\'')

    // Write theme injection code
    themeInjection = `;(() => {
      const ts = document.createElement('style')

      ts.textContent = \`
        ${cleanContents?.replace(/`/g, '\\`')
            .replace(/\\8/g, '')
            .replace(/\\9/g, '')
          }
      \`

      document.head.appendChild(ts)
    })()`
  }

  const injectionJs = await invoke('get_injection_js', {
    pluginJs: plugins,
    themeJs: themeInjection,
    origin: window.location.origin
  })

  invoke('load_injection_js', {
    imports,
    contents: injectionJs
  })

  // Remove loading container
  //document.querySelector('#loadingContainer')?.remove()
})()

async function displayLoadingTop() {
  const html = await invoke('get_index')
  const loadingContainer = document.createElement('div')
  loadingContainer.id = 'loadingContainer'
  loadingContainer.innerHTML = html

  loadingContainer.style.zIndex = 99999
  loadingContainer.style.position = 'absolute'
  loadingContainer.style.top = '0'
  loadingContainer.style.left = '0'
  loadingContainer.style.width = '100vw'
  loadingContainer.style.height = '100vh'

  document.body.appendChild(loadingContainer)
}

// Prevent any fuckery within themes
function cssSanitize(css) {
  const style = document.createElement('style');
  style.innerHTML = css;

  document.head.appendChild(style);

  if (!style.sheet) return

  const result = Array.from(style.sheet.cssRules).map(rule => rule.cssText || '').join('\n');

  document.head.removeChild(style);
  return result;
}