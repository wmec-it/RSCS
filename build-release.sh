#!/usr/bin/bash

#:& Makes sure you have the right target installed
rustup target add x86_64-pc-windows-gnu

#:& Release build is outputted to PROJECT_ROOT/target/release/*
cargo build --release --target x86_64-pc-windows-gnu