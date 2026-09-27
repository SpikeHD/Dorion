#!/usr/bin/env bash
set -euo pipefail

# TODO: the moment shared CEF libs are available we are ditching this

REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
STAGE="$REPO_ROOT/src-tauri/.cef"

IFS=' ' read -r -a KEPT_LOCALES <<< "${KEPT_LOCALES:-en-US}"
IFS=' ' read -r -a DROPPED_FILES <<< "${DROPPED_CEF_FILES:-libvk_swiftshader.so vk_swiftshader_icd.json libvulkan.so.1 archive.json}"

find_cef_dir() {
  local d
  shopt -s nullglob
  shopt -s globstar
  for d in "$REPO_ROOT"/src-tauri/target/**/cef_linux_*; do
    [[ -f "$d/libcef.so" ]] && { printf '%s' "$d"; return 0; }
  done
  shopt -u globstar
  for d in "$REPO_ROOT"/src-tauri/target/release "$REPO_ROOT"/src-tauri/target/*/release; do
    [[ -f "$d/libcef.so" ]] && { printf '%s' "$d"; return 0; }
  done
  return 1
}

CEF_SRC="$(find_cef_dir)" || {
  echo "error: libcef.so not found under src-tauri/target (build the project first)" >&2
  exit 1
}

is_dropped() {
  local name="$1"
  for dropped in "${DROPPED_FILES[@]}"; do
    [[ "$name" == "$dropped" ]] && return 0
  done
  return 1
}

rm -rf "$STAGE"
mkdir -p "$STAGE"

for entry in "$CEF_SRC"/*; do
  name="$(basename "$entry")"
  is_dropped "$name" && continue
  case "$name" in
    locales)
      mkdir -p "$STAGE/locales"
      for loc in "${KEPT_LOCALES[@]}"; do
        for f in "$CEF_SRC"/locales/"$loc"*.pak; do
          [[ -f "$f" ]] && cp -f "$f" "$STAGE/locales/"
        done
      done
      ;;
    *.so*|*.pak|*.dat|*.bin|*.json|chrome-sandbox) cp -a "$entry" "$STAGE/" ;;
  esac
done

CEF_ARCH="$(basename "$CEF_SRC")"
CEF_ARCH="${CEF_ARCH##*_}"
case "$CEF_ARCH" in
  aarch64) STRIP_TOOL="aarch64-linux-gnu-strip" ;;
  x86_64|amd64|i386|i686) STRIP_TOOL="strip" ;;
  *)
    case "$(uname -m)" in
      aarch64) STRIP_TOOL="aarch64-linux-gnu-strip" ;;
      *) STRIP_TOOL="strip" ;;
    esac
    ;;
esac

if [ -n "$STRIP_TOOL" ] && command -v "$STRIP_TOOL" > /dev/null 2>&1; then
  for lib in "$STAGE"/*.so*; do
    [ -f "$lib" ] || continue
    "$STRIP_TOOL" --strip-unneeded "$lib"
  done
else
  echo "warning: $STRIP_TOOL not found, skipping CEF strip" >&2
fi
