// Avoid Discord voice/video getting stuck at "DTLS Connecting" under
// VPN/TUN/fake-IP setups where WebRTC may pick a non-default interface.
const POLICY = "default_public_and_private_interfaces";

function applyWebRtcPolicy() {
  const setting = chrome?.privacy?.network?.webRTCIPHandlingPolicy;

  if (!setting) {
    console.warn("[Dorion WebRTC] chrome.privacy.network.webRTCIPHandlingPolicy is unavailable");
    return;
  }

  setting.set({ value: POLICY }, () => {
    if (chrome.runtime.lastError) {
      console.warn("[Dorion WebRTC] Failed to set WebRTC policy:", chrome.runtime.lastError.message);
      return;
    }

    console.log("[Dorion WebRTC] Applied WebRTC IP handling policy:", POLICY);
  });
}

chrome.runtime.onInstalled.addListener(applyWebRtcPolicy);
chrome.runtime.onStartup.addListener(applyWebRtcPolicy);

applyWebRtcPolicy();
