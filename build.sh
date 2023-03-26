#!/bin/bash

# Move to the directory of this script.
SCRIPT_DIR=$(cd $(dirname ${BASH_SOURCE:-$0}); pwd)
pushd $SCRIPT_DIR

cargo build --release
mkdir -p bin
cp -f target/release/golden-dawn.exe bin/golden-dawn.exe
cp -rf resource bin/

cargo doc

# Back to the saved directory.
popd
