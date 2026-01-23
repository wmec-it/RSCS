#!/usr/bin/bash

# WARNING:  This needs to be changed if we are going to make this utility configure Linux machines.
#:& Makes sure you have the right target installed
rustup target add x86_64-pc-windows-gnu

# WARNING:  This needs to be changed if we are going to make this utility configure Linux machines.
#:& Build is outputted to PROJECT_ROOT/target/debug/*
cargo build --target x86_64-pc-windows-gnu