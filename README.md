<h1 align="center">Dorion</h1>
<div align="center">
 <img src="https://img.shields.io/github/workflow/status/SpikeHD/Dorion/Build" />
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
</div>

# Table of Contents

* [Setup](#setup)
* [Benefits](#benefits)
* [Limitations](#limitations)
* [Known Issues](#known-issues)
* [TODO](#todo)
* [Using Plugins and Themes](#using-plugins-and-themes)
* [Contributing](#contributing)
* [Screenshots](#screenshots)

# Setup

Download a [release](https://github.com/SpikeHD/Dorion/releases) (`.msi` for Windows 10/11, ~~`.zip` for Windows 7~~, `.deb` for Linux, etc.) and install!

# Benefits

* Portable (mostly)
* Plugins
* Themes
  * Including support for BetterDiscord themes!
* Low - if any - cache footprint (compared to [the PTB client](https://user-images.githubusercontent.com/25207995/189549033-b372ca74-5f30-4864-b71a-10a88405537a.png))
* Extremely small installation size (~10mb!) on Windows
  * This is because unlike the Discord client, Dorion does *not* bundle an entire Chromium engine
* Often loads slightly faster (vanilla)
* Switch between Stable, Canary and PTB clients straight from the settings
* Made by me (automatically makes it cooler)

I know I made the program 'n all, but I actually use this on my laptop as opposed to the official client, so I can vouch 😎

# Limitations

Dorion simply runs the web-based version of Discord within it's own client. This means that things Discord web doesn't support will not work. I am unaware of any significant feature that is missing.

# Known Issues

* Drag 'n Drop ([#3](https://github.com/SpikeHD/Dorion/issues/3))
* A couple bugs with CSS & image importt related stuff

# TODO

* Desktop notifications
  * AND displaying the number of notifs in the desktop icon
* Rich presence(?)
* Helper API methods and events for plugins
* Backup localized themes
* Minimize to tray

# Using Plugins and Themes

*See the `examples` directory for examples of plugins, including how to include external code, and themes. You can also look at [my own plugins/themes repo](https://github.com/SpikeHD/DorionPluginsAndThemes) for some existing ones with actual use.*

Plugins and themes are relatively simple to use, the file structure looks like so on Windows:

```
.
└── C:/Users/USERNAME/dorion
    ├── plugins/
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
└── ~/dorion
    ├── plugins/
    |   └── plugin_name/
    |       └── index.js
    └── themes/
        └── theme.css
        └── Theme2
            └── theme2.css
```

so if you download a plugin or theme, just pop it into it's own folder in the `plugins`/`themes` folder!

# Contributing

Issues, PRs, etc. are all welcome!

# Screenshots

![image](https://user-images.githubusercontent.com/25207995/202989727-e467e711-b916-42d8-ad0c-4cbbb645a133.png)
Installation size.

![image](https://user-images.githubusercontent.com/25207995/202835496-d10156bf-803c-4d3e-804f-761618ba8bb8.png)
Loading screen. So cool!

![image](https://user-images.githubusercontent.com/25207995/202835284-910be0cb-a4fc-4272-b8ae-1a6187d68062.png)
Theme: [Dark Matter](https://betterdiscord.app/theme/Dark%20Matter)

![image](https://user-images.githubusercontent.com/25207995/202835451-31432fbd-69f1-4564-8830-59ebfcfde7fe.png)
Theme: [Dark Neon](https://betterdiscord.app/theme/Dark%20Neon)

