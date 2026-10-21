import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { stripTypeScriptTypes } from 'node:module'
import test from 'node:test'
import vm from 'node:vm'

const source = stripTypeScriptTypes(readFileSync(new URL('../src-tauri/injection/shared/display_media.ts', import.meta.url), 'utf8'))
  .replace('export function configureDisplayMedia', 'function configureDisplayMedia')

// Run the shipped policy in an isolated browser-like realm for each scenario.
function createBrowser(platform = 'Win32', supported = true) {
  const calls = []
  const stream = Promise.resolve({ stream: 'native stream' })
  const devices = {
    getSupportedConstraints: () => ({ restrictOwnAudio: supported }),
    getDisplayMedia(...args) {
      assert.equal(this, devices)
      calls.push(args)
      return stream
    }
  }
  const original = devices.getDisplayMedia
  const context = vm.createContext({ navigator: { platform, mediaDevices: devices } })
  vm.runInContext(`${source}\nconfigureDisplayMedia()`, context)
  return { devices, original, calls, stream, context }
}

test('Windows capture excludes Dorion and requests application audio without changing video', () => {
  const browser = createBrowser()
  const video = Object.freeze({ frameRate: 60, height: 1440 })
  const audio = Object.freeze({ echoCancellation: false, channelCount: 2, restrictOwnAudio: false })
  const controller = {}
  const options = Object.freeze({ video, audio, controller, surfaceSwitching: 'include' })
  const result = browser.devices.getDisplayMedia(options)
  assert.equal(result, browser.stream)
  assert.equal(browser.calls.length, 1)
  const forwarded = browser.calls[0][0]
  assert.equal(forwarded.video, video)
  assert.equal(forwarded.controller, controller)
  assert.equal(forwarded.surfaceSwitching, 'include')
  assert.equal(forwarded.systemAudio, 'include')
  assert.equal(forwarded.windowAudio, 'window')
  assert.equal(forwarded.audio.restrictOwnAudio, true)
  assert.equal(forwarded.audio.echoCancellation, false)
  assert.equal(forwarded.audio.channelCount, 2)
  assert.equal(audio.restrictOwnAudio, false)
  assert.equal(Object.hasOwn(options, 'windowAudio'), false)
})

test('audio:true receives the same policy without requesting a second capture', () => {
  const browser = createBrowser()
  browser.devices.getDisplayMedia({ video: true, audio: true })
  assert.equal(browser.calls.length, 1)
  assert.equal(browser.calls[0][0].audio.restrictOwnAudio, true)
  assert.equal(browser.calls[0][0].windowAudio, 'window')
})

test('video-only and invalid requests retain native argument handling', () => {
  const browser = createBrowser()
  for (const options of [undefined, null, false, 123, { video: true }, { video: true, audio: false }, { audio: null }, { audio: 'invalid' }]) {
    const previous = browser.calls.length
    browser.devices.getDisplayMedia(options, 'extra argument')
    assert.equal(browser.calls.length, previous + 1)
    assert.equal(browser.calls[previous][0], options)
    assert.equal(browser.calls[previous][1], 'extra argument')
  }
})

test('explicit window scopes and audio exclusions are preserved', () => {
  const browser = createBrowser()
  for (const windowAudio of ['system', 'window', 'exclude', null, 'invalid']) {
    browser.devices.getDisplayMedia({ video: true, audio: {}, windowAudio, systemAudio: 'exclude' })
    const forwarded = browser.calls.at(-1)[0]
    assert.equal(forwarded.windowAudio, windowAudio)
    assert.equal(forwarded.systemAudio, 'exclude')
  }
  browser.devices.getDisplayMedia({ video: true, audio: {}, systemAudio: 'exclude' })
  assert.equal(browser.calls.at(-1)[0].windowAudio, 'exclude')
})

test('Vencord can bind the early hook and rebuild options without losing the audio policy', () => {
  const browser = createBrowser()
  const retainedCapture = browser.devices.getDisplayMedia.bind(browser.devices)
  // Match WebScreenShare's native-call boundary, including its mute control.
  browser.devices.getDisplayMedia = function (options, muted = false) {
    return retainedCapture({
      video: { ...options.video, height: 1080, frameRate: 60 },
      audio: { ...options.audio, restrictOwnAudio: true },
      surfaceSwitching: 'include',
      systemAudio: muted ? 'exclude' : 'include'
    })
  }
  browser.devices.getDisplayMedia({ video: { displaySurface: 'window' }, audio: true })
  const forwarded = browser.calls[0][0]
  assert.equal(forwarded.windowAudio, 'window')
  assert.equal(forwarded.audio.restrictOwnAudio, true)
  assert.equal(forwarded.video.displaySurface, 'window')
  assert.equal(forwarded.video.height, 1080)
  assert.equal(forwarded.video.frameRate, 60)
  browser.devices.getDisplayMedia({ video: true, audio: true }, true)
  assert.equal(browser.calls[1][0].systemAudio, 'exclude')
  assert.equal(browser.calls[1][0].windowAudio, 'exclude')
})

test('macOS, Linux, and older Windows runtimes keep their original API', () => {
  for (const [platform, supported] of [['MacIntel', true], ['Linux x86_64', true], ['Win32', false], ['Win32', undefined]]) {
    const browser = createBrowser(platform, supported === undefined ? false : supported)
    assert.equal(browser.devices.getDisplayMedia, browser.original)
    const options = { video: true, audio: true }
    browser.devices.getDisplayMedia(options)
    assert.equal(browser.calls[0][0], options)
  }
})

test('missing media APIs do not interrupt initialization', () => {
  for (const mediaDevices of [undefined, {}, { getDisplayMedia() {} }]) {
    const context = vm.createContext({ navigator: { platform: 'Win32', mediaDevices } })
    assert.doesNotThrow(() => vm.runInContext(`${source}\nconfigureDisplayMedia()`, context))
  }
})

test('native cancellation and receiver errors propagate without retry or fallback', async () => {
  const browser = createBrowser()
  assert.throws(() => browser.devices.getDisplayMedia.call({}, { video: true, audio: true }))
  const rejection = new Error('User canceled the picker')
  const native = () => Promise.reject(rejection)
  const devices = { getDisplayMedia: native, getSupportedConstraints: () => ({ restrictOwnAudio: true }) }
  const context = vm.createContext({ navigator: { platform: 'Win32', mediaDevices: devices } })
  vm.runInContext(`${source}\nconfigureDisplayMedia()`, context)
  await assert.rejects(devices.getDisplayMedia({ video: true, audio: true }), error => error === rejection)
})

test('the hook runs before the first await and client-mod injection', () => {
  const preinject = readFileSync(new URL('../src-tauri/injection/preinject.ts', import.meta.url), 'utf8')
  const install = preinject.indexOf('  configureDisplayMedia()')
  assert.ok(install > preinject.indexOf('if (window.self !== window.top)'))
  assert.ok(install < preinject.indexOf('await '))
})
