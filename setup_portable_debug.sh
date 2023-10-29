#!/bin/bash

# This script will create dirs and files that will make `pnpm tauri dev` instances of Dorion think they are portable.
# This is helpful for shelter testing, as you will be able to put built shelter files in ./src-tauri/injection

BASE_PATH=./src-tauri/target/debug/

# Create config.json with empty JSON oject
echo "{}" > $BASE_PATH/config.json