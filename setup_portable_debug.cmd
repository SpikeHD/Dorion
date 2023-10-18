@echo off

REM This script will create dirs and files that will make `pnpm tauri dev` instances of Dorion think they are portable.
REM This is helpful for Vencordorion testing, as you will be able to put built Vencordorion files in ./src-tauri/injection

set BASE_PATH=./src-tauri/target/debug

if not exist %BASE_PATH% (
  echo "Error: %BASE_PATH% does not exist. Build Dorion at least once first!"
  exit /b 1
)

REM Create config.json with an empty JSON object
echo {} > %BASE_PATH%/config.json