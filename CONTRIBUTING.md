# Contributing

Thank you for considering working on Dorion! There are only a couple things to keep in mind, as well as some tips to make development quick.

## Guidelines for Pull Requests

* Ensure pull requests only change one feature or "thing". Do not put 6 different bug fixes into one PR, for example.
* Describe what your pull request does in some amount of detail. No need to write an essay, but knowing what it does at a glance is helpful to me and others.
* For pull requests that also require a change in [shelter-plugins](https://github.com/SpikeHD/shelter-plugins), link that pull request in your PR to Dorion

## Working with Dorion

Dorion as a whole is only two components, the main stuff (this repo), and [shelter-plugins](https://github.com/SpikeHD/shelter-plugins). My shelter-plugins control things like the settings menu, and complex patches to things like Discords internals.

Jump to:
* [Set up Dorion to think the debug version is portable](#set-up-Dorion-to-think-the-debug-version-is-portable)
* [Testing changes in Dorion](#testing-changes-in-dorion)
* [Testing changes in the updater](#testing-changes-in-the-updater)
* [Testing changes in Shelter Plugins](#testing-changes-in-shelter-plugins)

### Set up Dorion to think the debug version is portable

It might be easiest to set up `pnpm tauri dev` to actually think that it is portable. This way, everything up to the config is seperated from your actual installation (if you have one),
and all contained in the `./src-tauri/target/debug` folder, instead of all over your system.

To do this, run `./setup_portable_debug.sh` or `./setup_portable_debug.cmd` on Windows. You may need to `chmod +x` it first. 

### Testing changes in Dorion

1. Ensure Dorion is not running already (since it will just focus that window otherwise)
2. Start in dev mode
   ```sh
   pnpm tauri dev
   ```

That's it! You'll see all sorts of logs spit out, and you can test your changes.

### Testing changes in the updater

1. Ensure Dorion is not running already (since it will just focus that window otherwise)
2. Build the updater
   ```sh
   pnpm build:updater
   ```

From here, you can test your changes in two ways:
* Let Dorion run the updater. Good for testing how the frontend and backend communicate
* Run the updater from CLI:
  ```sh
  # Just and example
  ./updater -arg1
  ```

### Testing changes in shelter-plugins

Since my [shelter-plugins](https://github.com/SpikeHD/shelter-plugins) are an entirely seperate Dorion component, you will also need to clone and build them. To do so is simple:

1. Clone, install dependencies, and change whatever you need to in shelter-plugins
2. Build them
   ```sh
   pnpm lune ci
   ```
3. Setup Dorion debug to [think it's portable](#set-up-Dorion-to-think-the-debug-version-is-portable) (if desired)
4. Start Dorion like [above](#testing-changes-in-dorion)
5. Copy the contents of whichever plugin you're testing via `./dist/plugins/plugin.js` into the Shelter "Add Plugin" menu. Remember to disable the default version
of whichever you are testing, if needed.
