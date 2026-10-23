// CEF on Linux cannot encode H264 apparently, but Discord's screen share
// only offers H264, so we map that to VP8.
export function sdpCodecRewrite() {
  const original = RTCPeerConnection.prototype.setLocalDescription
  RTCPeerConnection.prototype.setLocalDescription = function (desc: RTCSessionDescriptionInit) {
    try {
      if (typeof desc?.sdp === 'string' && desc.sdp.includes('H264')) {
        const lines = desc.sdp.split('\n')
        const h264pts = lines
          .filter(l => /^a=rtpmap:\d+ H264\//.test(l))
          .map(l => l.match(/^a=rtpmap:(\d+)/)![1])
        desc.sdp = lines
          .filter(l => !(l.startsWith('a=fmtp:') && h264pts.some(p => l.startsWith(`a=fmtp:${p} `))))
          .map(l => (/^a=rtpmap:\d+ H264\//.test(l) ? l.replace('H264/90000', 'VP8/90000') : l))
          .join('\n')
      }
    } catch {
      // keep the original SDP on any parsing failure
    }
    return original.call(this, desc)
  }
}
