<h1 align="center">
 <img height="100px" src="https://raw.githubusercontent.com/SpikeHD/Dorion/main/src-tauri/icons/icon.png" />
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
 <img src="https://img.shields.io/github/downloads/SpikeHD/Dorion/total" />
</div>

<div align="center">
 Dorion is an alternative Discord client aimed towards lower-spec or storage-sensitive PCs that supports themes, plugins, and more!
 <br />
 https://discord.gg/agQ9mRdHMZ
</div>

# Table of Contents

* [Setup](#setup)
* [Features](#features)
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

Download a [release](https://github.com/SpikeHD/Dorion/releases) (`.msi` for Windows 10/11, `.zip` for Windows 7, `.dmg` for MacOS, `.deb` for Debian, etc.) and install!

You can also [build it](#building) yourself, or download a [build artifact](https://github.com/SpikeHD/Dorion/actions/workflows/build.yml?query=branch%3Amain) from GitHub Actions!

# Features

* [Significantly smaller](https://github.com/SpikeHD/Dorion/assets/25207995/90d35eb0-5a34-45b9-b707-d64b3bc99cdf) than most other web-based client alternatives
* Theme support
* [Shelter](https://github.com/uwu/Shelter) included out of the box
* Support for other client mods and plugins, like [Vencord](https://github.com/vendicated/vencord)
  * There is ***no*** BetterDiscord support... [yet](https://github.com/SpikeHD/Dorion/issues/91#issuecomment-1712269268)
* Partial [game presence](https://github.com/SpikeHD/rsRPC) support included out of the box. Enable it in "Performance & Extras"!
* (Hopefully) better low-end system performance.

## Plugins

Dorion comes with [shelter](https://github.com/uwu/shelter), so that should cover at least some plugin-related needs. You can also install mods like
[Vencord](https://github.com/vendicated/vencord) if you'd like! Remember to download the `browser.js` version.

## Themes

Dorion supports all themes, BetterDiscord and others, with a [couple caveats](#known-issues).

[Jump to "Using Plugins and Themes"](#using-plugins-and-themes)

# Platform Support

<div width="100%" align="center">

| Feature                                          | Windows | Linux            | MacOS           |
|--------------------------------------------------|---------|------------------|-----------------|
| *Basics (logging in, navigation, text/DMs etc.)* | ✓       | ✓               | ✓               |
| Voice                                            | ✓       | ✗ <sup>[1]</sup>| ✓               |
| Themes                                           | ✓       | ✓               | ✓               |
| Shelter                                          | ✓       | ✓               | ✓               |
| Dorion Plugins                                   | ✓       | ✓               | ✓               |
</div>

<sup>[1]</sup> Currently can connect to VC, but won't pass "RTC Connecting". Needs a bit more coaxing to get working.<br/>

# Building

## Prerequisites

* [NodeJS](https://nodejs.org)
* [PNPM](https://pnpm.io/)
* [Rust and Cargo](https://www.rust-lang.org/tools/install)
* [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/#1-system-dependencies)

## Steps

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

5. Pull the latest shelter build

    ```sh
    pnpm shupdate
    ```

6. Build!

    ```sh
    # Build the updater
    pnpm build:updater

    # Build Dorion
    pnpm tauri build
    # or to debug/open in dev mode
    pnpm tauri dev
    ```

All built files will be in `src-tauri/target/(release|debug)/`. When using portably, the `html`, `icons`, and `injection` folders are required. Installation files (eg. `.msi`) are located in `bundle/`

# Known Issues

* (Windows) Large images in themes will not load
* (MacOS) Injection JS does not reinject after reloading the page
* (Linux) Uses a wackload of memory for some reason
* Fonts/font-faces from sources other than Google will not load

# Troubleshooting

If you are having problems opening Dorion, or it instantly crashes, or something similar, try the following:

* Install via MSI instead of the `.zip` file
* Use the `.zip` file instead of the MSI
* (If using the `.zip` file) make sure all files were extracted properly (`html`, `injection`, etc.)
* [Reinstall WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
  * Fully uninstall and reinstall.
  * If you are having trouble uninstalling it, or the installer says its already installed even though you uninstalled, try deleting this registry folder and uninstalling again `Computer\HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}`
  * You can also try [uninstalling from the Command Prompt](https://superuser.com/a/1743626)

# TODO

* [x] Pre-process fonts like images/CSS imports are already done
* [x] Multi-thread CSS processing
* [ ] Use resource files from within the binary itself instead of the filesystem
* [x] Desktop notifications
  * [x] AND displaying the number of notifs in the desktop icon
* [x] Webpack stuff
* [x] Global push-to-talk
* [x] Rich presence(?)
  * [ ] FULL rich presence 
* [ ] Helper API methods and events for plugins
* [x] Backup localized themes
* [x] Localization timeout
* [x] Safemode key (disable themes and plugins)
* [x] New release notifications
* [ ] Logging system (like [reMITM](https://github.com/SpikeHD/reMITM))

# Using Plugins and Themes

*See the `examples` directory for examples of plugins, including how to include external code and themes. You can also look at [my own plugins/themes repo](https://github.com/SpikeHD/DorionPluginsAndThemes) for some basic ones.*

Plugins and themes are relatively simple to use, the file structure looks like so on Windows:

```
C:/Users/%USERNAME%/dorion/
    ├── plugins/
    |   └── plugin.js
    └── themes/
        └── theme.css
```

and like so on Linux:

```
~/.config/dorion/
    ├── plugins/
    |   └── plugin.js
    └── themes/
        └── theme.css
```

so if you download a plugin or theme, just pop it into the `plugins`/`themes` folder. If you need help finding them, there are buttons in Dorion settings that'll take you where you need!

# Contributing

Issues, PRs, etc. are all welcome! For guidelines and tips, see [CONTRIBUTING.md](https://github.com/SpikeHD/Dorion/blob/main/CONTRIBUTING.md)

# Screenshots

## Installation size
![image](https://github.com/SpikeHD/Dorion/assets/25207995/90d35eb0-5a34-45b9-b707-d64b3bc99cdf)

## Loading screen
![image](https://user-images.githubusercontent.com/25207995/202835496-d10156bf-803c-4d3e-804f-761618ba8bb8.png)

## Settings Menu
![image](https://github.com/SpikeHD/Dorion/assets/25207995/e8d610b9-fd02-43eb-943c-0003e0c07d11)
Theme: [Catpuccin - Frappe](https://github.com/catppuccin/discord)

![image](https://github.com/SpikeHD/Dorion/assets/25207995/c73a2333-31fb-404a-9489-5e1b1f8cfa54)
Theme: [Fluent](https://betterdiscord.app/theme/Fluent)
