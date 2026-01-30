#!/usr/bin/bash

# WARNING:  This needs to be changed if we are going to make this utility configure Linux machines.
#:& Makes sure you have the right target installed
rustup target add x86_64-pc-windows-gnu

# WARNING:  This needs to be changed if we are going to make this utility configure Linux machines.
#:& Release build is outputted to PROJECT_ROOT/target/release/*
cargo build --release --target x86_64-pc-windows-gnu

# WARNING:  This needs to be changed if we are going to make this utility configure Linux machines.
#:& Move JSON files into built release folder
bash ./lib/move_json_files.sh

# WARNING:  This needs to be changed if we are going to make this utility configure Linux machines.
#:& Move assets into built release folder
bash ./lib/move_assets.sh