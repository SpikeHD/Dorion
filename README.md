# Dorion

Alternative Discord client that just opens the web app. Way smaller than the official client, but it works basically identically.

# Setup

Download a [release](/releases) (`.msi` for Windows 10/11, `.zip` for Windows 7, `.AppImage` for Linux, etc.) and install!

# Using Plugins

Plugins are relatively simple to use, the file structure looks like so:

```
.
└── /path/to/Dorion/
    ├── Dorion.exe or whatever
    └── plugins/
        └── plugin_name/
            └── index.js
```

so if you download a plugin, just pop it into it's own folder in the `plugins` folder!
