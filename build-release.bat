@echo off

@REM Makes sure you have the right target installed
rustup target add x86_64-pc-windows-msvc

@REM Release build is outputted to PROJECT_ROOT/target/release/*
cargo build --release --target x86_64-pc-windows-msvc

@REM Move JSON files into built release folder
powershell ./lib/move_json_files.ps1

@REM Move assets into built release folder
powershell ./lib/move_assets.ps1