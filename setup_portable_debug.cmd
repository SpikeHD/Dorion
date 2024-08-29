@echo off

REM This script will create dirs and files that will make `pnpm tauri dev` instances of Dorion think they are portable.
REM This is helpful for  testing, as you will be able to put built shelter files in ./src-tauri/injection

set BASE_PATH=.\src-tauri\target\debug\

if not exist %BASE_PATH% (
  REM create the directory
  mkdir %BASE_PATH%
)

REM Create a .portable file
echo "" > %BASE_PATH%\.portable