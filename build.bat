@echo off

@REM Makes sure you have the right target installed
rustup target add x86_64-pc-windows-msvc

@REM Build is outputted to PROJECT_ROOT/target/debug/*
cargo build --target x86_64-pc-windows-msvc

@REM Move JSON files into built release folder
powershell Unblock-File -Path ./lib/move_json_files.ps1
powershell ./lib/move_json_files.ps1

@REM Move assets into built release folder
powershell Unblock-File -Path ./lib/move_assets.ps1
powershell ./lib/move_assets.ps1

@REM Move vc_runtime.bat into built release folder
powershell Unblock-File -Path ./lib/move_vc_runtime.ps1
powershell ./lib/move_vc_runtime.ps1