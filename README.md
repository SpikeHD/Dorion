<h1 align="center">
 <img height="100px" src="https://user-images.githubusercontent.com/25207995/233253555-7f398710-bf80-4241-9397-b01930e56714.png" />
 <br />
 Dorion
</h1>
<div align="center">
 <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/Dorion/build.yml" />
 <img src="https://img.shields.io/github/package-json/v/SpikeHD/Dorion" />
 <img src="https://img.shields.io/github/repo-size/SpikeHD/Dorion" />
</div>
<div align="center">
 <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/Dorion" />
 <img src="https://img.shields.io/github/release-date/SpikeHD/Dorion" />
 <img src="https://img.shields.io/github/stars/SpikeHD/Dorion" />
</div>

<div align="center">
Dorion is an alternative Discord client aimed and lower-spec or storage-sensitive PCs that supports themes, plugins, and more!
 <br />
 https://discord.gg/agQ9mRdHMZ
</div>

# Table of Contents

* [Setup](#setup)
* [Why use Dorion?](#why-use-dorion)
  * [Plugins](#plugins)
  * [Themes](#themes)
* [Platform Support](#platform-support)
* [Building](#building)
  * [Prerequisites](#prerequisites)
  * [Steps](#steps)
* [Known Issues](#known-issues)
* [Troubleshooting](#troubleshooting)
* [TODO](#todo)
* [Using Plugins and Themes](#using-plugins-and-themes)
* [Contributing](#contributing)
* [Screenshots](#screenshots)

# Setup

Download a [release](https://github.com/SpikeHD/Dorion/releases) (`.msi` for Windows 10/11, `.zip` for Windows 7, `.deb` for Debian, etc.) and install!

If you'd like to be on the cutting edge, you can also grab an artifact from the [actions tab!](https://github.com/SpikeHD/Dorion/actions/workflows/build.yml)

# Why use Dorion?

* Portable (mostly)
* Low - if any - cache footprint (compared to [the PTB client](https://user-images.githubusercontent.com/25207995/189549033-b372ca74-5f30-4864-b71a-10a88405537a.png))
* Extremely small installation size (~10mb!) on Windows
  * This is because unlike the Discord client, Dorion does *not* bundle an entire Chromium engine
* Often loads slightly faster (vanilla)
* Switch between Stable, Canary and PTB clients straight from the settings
* Built-in telemetry blocking option
* Not only is Vencord included by default, but Dorion uses a [custom fork of Vencord](https://github.com/SpikeHD/Vencordorion) with extras specifically for Dorion
* Made by me (automatically makes it cooler)

## Plugins

While Dorion does *not* support BetterDiscord plugins (or other mods that use modified `.asar`s), it *does* support browser-based ones!

## Themes

Dorion supports all themes, BetterDiscord and others

[Jump to "Using Plugins and Themes"](#using-plugins-and-themes)

# Platform Support

<div width="100%" align="center">
 <table width="100%">
  <tr>
   <th><i>Feature</i></th>
   <th>Windows</th>
   <th>Linux</th>
   <th>MacOS</th>
  </tr>

  <tr>
   <td>Basics (logging in, navigation, text/DMs etc.)</td>
   <td>✓</td>
   <td>✓</td>
   <td>✓</td>
  </tr>

  <tr>
   <td>Voice</td>
   <td>✓</td>
   <td>❌<sup>[1]</sup></td>
   <td>❌<sup>[2]</sup></td>
  </tr>

  <tr>
   <td>Themes</td>
   <td>✓</td>
   <td>✓</td>
   <td>✓</td>
  </tr>
  
  <tr>
   <td>Vencord (and included plugins)</td>
   <td>✓</td>
   <td>❌<sup>[3]</sup></td>
   <td>✓</td>
  </tr>

  <tr>
   <td>Dorion Plugins</td>
   <td>✓</td>
   <td>✓</td>
   <td>✓</td>
  </tr>
 </table>
</div>

<sup>[1]</sup> Webkit2GTK does not support WebRTC. See <a href="https://github.com/SpikeHD/Dorion/issues/30">#30</a>.<br/>
<sup>[2]</sup> Currently can connect to VC, but won't pass "RTC Connecting". Needs a bit more coaxing to get working.<br/>
<sup>[3]</sup> This is due to unsupported RegEx. In order for this to work, Webkit2GTK needs to update to support it, or I need to rewrite all RegEx in Vencord to not use lookbehind/ahead (there is a lot).

# Building

### Prerequisites

* [NodeJS](https://nodejs.org)
* [PNPM](https://pnpm.io/)
* [Rust and Cargo](https://www.rust-lang.org/tools/install)

### Steps

1. Clone/download the repository
2. Open a terminal window in the root project folder
3. Install JS dependencies:
  ```sh
  pnpm install
  ```
4. Build the minified versions of the JS/HTML files:
  ```sh
  pnpm build
  ```
5. Pull the Vencord fork
  ```sh
  pnpm update
  ```
6. Build!
  ```sh
  pnpm tauri build
  # or to debug/open in dev mode
  pnpm tauri dev
  ```

All built files will be in `src-tauri/target/(release|debug)/`. When using portably, the `html`, `icons`, and `injection` folders are required. Installation files (eg. `.msi`) are located in `bundle/msi`

# Known Issues

* A couple bugs with CSS & image import related stuff
* Fonts/font-faces will randomly not work
* (MacOS) Injection JS does not reinject after reloading the page
* (Linux) Vencord JS does not inject... at all.
  * This is due to lookbehind/ahead RegEx not being supported in Webkit2GTK, which is what Dorion uses on Linux. Either I have to replace all lookbehind/ahead RegEx with something else, or wait for Webkit2GTK to support it.

# Troubleshooting

If you are having problems opening Dorion, or it instantly crashes, or something similar, try the following:
* Install via MSI instead of the `.zip` file
* Use the `.zip` file instead of the MSI
* (If using the `.zip` file) make sure all files were extracted properly (`html`, `injection`, etc.)
* [Reinstall WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
  * Fully uninstall and reinstall.
  * If you are having trouble uninstalling it, try deleting this registry folder and uninstalling again `Computer\HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}`
  * You can also try [uninstalling from the Command Prompt](https://superuser.com/a/1743626)


# TODO

* [x] Pre-process fonts like images/CSS imports are already done
* [x] Multi-thread CSS processing
* [ ] Use resource files from application itself instead of the filesystem
* [ ] Desktop notifications
  * [x] AND displaying the number of notifs in the desktop icon
* [x] Webpack stuff
* [x] Global push-to-talk
* [ ] Rich presence(?)
* [ ] Helper API methods and events for plugins
* [x] Backup localized themes
* [x] Localization timeout
* [x] Safemode key (disable themes and plugins)
* [x] New release notifications
* [ ] Logging system (like [reMITM](https://github.com/SpikeHD/reMITM))

# Using Plugins and Themes

*See the `examples` directory for examples of plugins, including how to include external code and themes. You can also look at [my own plugins/themes repo](https://github.com/SpikeHD/DorionPluginsAndThemes) for some existing ones with actual use.*

Plugins and themes are relatively simple to use, the file structure looks like so on Windows:

```
.
└── C:/Users/USERNAME/dorion/
    ├── plugins/
    |   └── plugin.js
    |   └── plugin_name/
    |       └── index.js
    └── themes/
        └── theme.css
        └── Theme2
            └── theme2.css
```

and like so on Linux:

```
.
└── ~/dorion/
    ├── plugins/
    |   └── plugin.js
    |   └── plugin_name/
    |       └── index.js
    └── themes/
        └── theme.css
        └── Theme2
            └── theme2.css
```

so if you download a plugin or theme, just pop it into the `plugins`/`themes` folder!

# Contributing

Issues, PRs, etc. are all welcome!

# Screenshots

![image](https://user-images.githubusercontent.com/25207995/202989727-e467e711-b916-42d8-ad0c-4cbbb645a133.png)
Installation size.

![image](https://user-images.githubusercontent.com/25207995/202835496-d10156bf-803c-4d3e-804f-761618ba8bb8.png)
Loading screen. So cool!

![image](https://github.com/SpikeHD/Dorion/assets/25207995/3958ad8f-6bb3-4e1d-b8a8-aae1d4d07157)

![image](https://user-images.githubusercontent.com/25207995/202835451-31432fbd-69f1-4564-8830-59ebfcfde7fe.png)
Theme: [Dark Neon](https://betterdiscord.app/theme/Dark%20Neon)

