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

Download a [release](https://github.com/SpikeHD/Dorion/releases) (`.msi` for Windows 10/11, `.zip` for Windows 7, `.deb` for Debian, etc.) and install!

You can also [build it](#building) yourself!

# Features

* Themes
* Plugins
* Multi-profile
* Smaller overall size
* (Hopefully) better low-end system performance

## Plugins

Dorion comes with a [custom fork of Vencord](https://github.com/SpikeHD/Vencordorion), so that should cover a lot of your plugin needs. Otherwise, it also supports most *browser-based* plugins! There is no BetterDiscord plugin support, however.

## Themes

Dorion supports all themes, BetterDiscord and others, with a [couple caveats](#known-issues).

[Jump to "Using Plugins and Themes"](#using-plugins-and-themes)

# Platform Support

<div width="100%" align="center">

| Feature                                          | Windows | Linux | MacOS           |
|--------------------------------------------------|---------|-------|-----------------|
| *Basics (logging in, navigation, text/DMs etc.)* | ✓       | ✓     | ✓               |
| Voice                                            | ✓       | ✓     | ✗ <sup>[1]</sup>|
| Themes                                           | ✓       | ✓     | ✓               |
| Vencord (and included plugins)                   | ✓       | ✓     | ✓               |
| Dorion Plugins                                   | ✓       | ✓     | ✓               |
</div>

<sup>[1]</sup> Currently can connect to VC, but won't pass "RTC Connecting". Needs a bit more coaxing to get working.<br/>

# Building

## Prerequisites

* [NodeJS](https://nodejs.org)
* [PNPM](https://pnpm.io/)
* [Rust and Cargo](https://www.rust-lang.org/tools/install)

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

5. Pull the Vencord fork

    ```sh
    pnpm vupdate
    ```

6. Build!

    ```sh
    pnpm tauri build
    # or to debug/open in dev mode
    pnpm tauri dev
    ```

All built files will be in `src-tauri/target/(release|debug)/`. When using portably, the `html`, `icons`, and `injection` folders are required. Installation files (eg. `.msi`) are located in `bundle/msi`

# Known Issues

* Large images in themes will not load
* Fonts/font-faces from sources other than Google will not load
* (MacOS) Injection JS does not reinject after reloading the page

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
C:/Users/USERNAME/dorion/
    ├── plugins/
    |   └── plugin.js
    └── themes/
        └── theme.css
```

and like so on Linux:

```
~/dorion/
    ├── plugins/
    |   └── plugin.js
    └── themes/
        └── theme.css
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
