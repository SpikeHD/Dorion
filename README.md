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

# Download

Installers are available on [the releases page](https://github.com/SpikeHD/Dorion/releases/latest). You can also get it from a [package repository](#package-repositories), or [GitHub Actions](https://github.com/SpikeHD/Dorion/actions/workflows/build.yml?query=branch%3Amain), or you can [build it yourself](#building)!

> [!NOTE]
> ***MacOS Users***: If opening Dorion gives you "Dorion.app is damaged and cannot be opened", MacOS is lying to you and you may just need to run `sudo xattr -rd com.apple.quarantine /Applications/Dorion.app`.
>
> ***Windows Users***: Defender may think Dorion is a virus. This just happens sometimes, and if SmartScreen blocks it from running, click "More Info" and "Run Anyways". Feel free to scan Dorion with [Virustotal](https://www.virustotal.com/gui/home/upload)!

# Table of Contents

* [Package Repositories](#package-repositories)
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

# Package Repositories

I do not maintain any instances of Dorion in any package repositories myself, however some very kind people maintain some in their own spare time:

* Windows:
  * Shovel/Scoop (Maintained by [Small-Ku](https://github.com/Small-Ku/)): 
    ```sh
    scoop bucket add turbo 'https://github.com/Small-Ku/turbo-bucket.git'
    scoop install turbo/dorion
    ```
  * WinGet (Maintained by [headquarter8302](https://github.com/headquarter8302))
    ```sh
    winget install --id SpikeHD.Dorion
    ```
* Linux:
  * Arch AUR (Maintained by [Refined7075](https://github.com/DarkCoder28))
    ```sh
    yay -S dorion-bin
    ```
  * NixOS
    ```sh
    nix-shell -p dorion
    ```

> [!NOTE]
> Maintaining Dorion in a different package repository that I don't know about? Feel free to open a PR to add it here!

# Features

* [Significantly smaller](https://github.com/SpikeHD/Dorion/assets/25207995/eb603f1f-f633-4913-a25e-1316b495a08a) than the original Discord client, as well as other web-based alternatives
* Theme support
* [Shelter](https://github.com/uwu/Shelter) included out of the box
* Support for other client mods and plugins, like [Vencord](https://github.com/vendicated/vencord)
  * There is ***no*** BetterDiscord support... [yet](https://github.com/SpikeHD/Dorion/issues/91#issuecomment-1712269268)
* Almost full [game presence](https://github.com/SpikeHD/rsRPC) support included out of the box. Enable it in "Performance & Extras"!
* (Hopefully) better low-end system performance.
* ARM support for ALL platforms

## Plugins

Dorion comes with [shelter](https://github.com/uwu/shelter), so that should cover at least some plugin-related needs. You can also enable client mods like [Vencord](https://github.com/vendicated/vencord) inside the Dorion settings page. If you want to install plugins not available within the Dorion settings page, ensure you are downloading a browser-compatible version.

## Themes

Dorion supports all themes, BetterDiscord and others, with a [couple caveats](#known-issues).

[Jump to "Using Plugins and Themes"](#using-plugins-and-themes)

# Platform Support

<div width="100%" align="center">

| Feature                                        | Windows 10/11 | Windows 7<sup>[1]</sup> | Linux            | MacOS           |
|------------------------------------------------|---------------|---------------|------------------|-----------------|
| Basics (logging in, navigation, text/DMs etc.) | ✓             | ✓             | ~<sup>[2]</sup>  | ✓               |
| Voice                                          | ✓             | ✓             | ✗<sup>[3]</sup> | ✓               |
| Themes                                         | ✓             | ✓             | ✓               | ✓               |
| Shelter                                        | ✓             | ✓             | ✓               | ✓               |
| Dorion Plugins                                 | ✓             | ✓             | ✓               | ✓               |

</div>

<sup>1</sup> While I am told it works fine, Windows 7 support is speculative and not guaranteed. It could break at any point, and if this happens, I probably won't put much effort into fixing it (PRs always welcome of course!).

<sup>2</sup> Some people report Dorion freezing on Linux, particularly when GIFs are playing. This is a bug in WebkitGTK.

<sup>3</sup> Support for WebRTC is hidden behind a build-time flag that is not used in almost all distros. Even with support, Tauri v1 does not support a high enough version of WebkitGTK bindings to enable the functionality. This will be available when WebkitGTK ships with WebRTC support.

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

All built files will be in `src-tauri/target/(release|debug)/`. When using portably, the `icons` and `injection` folders are required. Installation files (eg. `.msi`) are located in `bundle/`

# Known Issues

* (Windows) Large images in themes will not load
* (MacOS) Injection JS does not reinject after reloading the page
* External images (UserBG, Decor, UserPFP, etc.) will not load
* Fonts/font-faces will not load

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
* [x] Use resource files from within the binary itself instead of the filesystem
* [x] Desktop notifications
  * [x] AND displaying the number of notifs in the desktop icon
* [x] Webpack stuff
* [x] Global push-to-talk
* [x] Rich presence(?)
  * [ ] FULL rich presence
* [x] Custom keybinds
* [ ] Helper API methods and events for plugins
* [x] Backup localized themes
* [x] Localization timeout
* [x] Safemode key (disable themes and plugins)
* [x] New release notifications
* [x] Logging system (like [reMITM](https://github.com/SpikeHD/reMITM))
* [ ] Move from `device_query` to `rdev` (since `device_query` doesn't work when spawned in a thread on MacOS)
* [x] API abstractions

# Using Plugins and Themes

> [!TIP]
> See the `examples` directory for examples of plugins, including how to include external code and themes.

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

> [!NOTE]
> Themes can also be installed by clicking `Install Theme from Link` in Theme settings, if you prefer

# Contributing

Issues, PRs, etc. are all welcome! For guidelines and tips, see [CONTRIBUTING.md](https://github.com/SpikeHD/Dorion/blob/main/CONTRIBUTING.md)

# Screenshots

## Installer Size Comparison (Windows)
<img width="100%" src="https://github.com/SpikeHD/Dorion/assets/25207995/55ce8a69-1732-4e17-90f6-5582bcc21d0c" />

## Full Installed Size Comparison (Windows)
<img width="100%" src="https://github.com/SpikeHD/Dorion/assets/25207995/eb603f1f-f633-4913-a25e-1316b495a08a" />

## Loading screen
<img width="100%" src="https://github.com/SpikeHD/Dorion/assets/25207995/5c9041da-038c-465c-b048-a7c4034a45e0" />

## Settings Menu
<img width="100%" src="https://github.com/SpikeHD/Dorion/assets/25207995/b34577eb-a583-4c9d-abf9-fde791e0f0aa" />

Theme: [Catpuccin - Frappe](https://github.com/catppuccin/discord)

<img width="100%" src="https://github.com/SpikeHD/Dorion/assets/25207995/c73a2333-31fb-404a-9489-5e1b1f8cfa54" />

Theme: [Fluent](https://betterdiscord.app/theme/Fluent)
