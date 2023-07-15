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

While Dorion does *not* support BetterDiscord plugins (or other mods that use modified `.asar`s), it *does* support browser-based ones. This includes popular plugins like [Vencord](https://github.com/Vendicated/Vencord)!

## Themes

Dorion supports all themes, BetterDiscord and others

[Jump to "Using Plugins and Themes"](#using-plugins-and-themes)

# Building

### Prerequisites

* [NodeJS](https://nodejs.org)
* [Rust and Cargo](https://www.rust-lang.org/tools/install)

### Steps

1. Clone/download the repository
2. Open a terminal window in the root project folder
3. Install JS dependencies:
  ```sh
  yarn install
  # or
  npm install
  ```
4. Build the minified versions of the JS/HTML files:
  ```sh
  yarn build
  # or
  npm run build
  ```
5. Pull the Vencord fork
  ```sh
  yarn update
  ```
6. Build!
  ```sh
  yarn tauri build
  # or to debug/open in dev mode
  yarn tauri dev
  ```

All built files will be in `src-tauri/target/(release|debug)/`. When using portably, the `html`, `icons`, and `injection` folders are required. Installation files (eg. `.msi`) are located in `bundle/msi`

# Known Issues

* Drag 'n Drop ([#3](https://github.com/SpikeHD/Dorion/issues/3))
* Push-to-talk
* A couple bugs with CSS & image import related stuff

# Troubleshooting

If you are having problems opening Dorion, or it instantly crashes, or something similar, try the following:
* Install via MSI instead of the `.zip` file
* Use the `.zip` file instead of the MSI
* (If using the `.zip` file) make sure all files were extracted properly (`html`, `injection`, etc.)
* [Reinstall WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)


# TODO

* [ ] Use resource files from application itself instead of the filesystem
* [ ] Desktop notifications
  * [x] AND displaying the number of notifs in the desktop icon
* [x] Webpack stuff
* [ ] Global push-to-talk
* [ ] Rich presence(?)
* [ ] Helper API methods and events for plugins
* [ ] Backup localized themes
* [ ] Localization timeout
* [x] Safemode key (disable themes and plugins)
* [x] New release notifications

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

![image](https://user-images.githubusercontent.com/25207995/233516070-146c3835-edf1-4fba-96dd-7df57022a06b.png)

![image](https://user-images.githubusercontent.com/25207995/202835451-31432fbd-69f1-4564-8830-59ebfcfde7fe.png)
Theme: [Dark Neon](https://betterdiscord.app/theme/Dark%20Neon)

