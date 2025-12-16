@echo off

@REM Makes sure you have the right target installed
rustup target add x86_64-pc-windows-gnu

@REM Build is outputted to PROJECT_ROOT/target/debug/*
cargo build --target x86_64-pc-windows-gnu