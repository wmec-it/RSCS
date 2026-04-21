@echo off

:: Makes sure you have the right target installed
powershell Write-Host "Adding correct target..."
rustup target add stable-x86_64-pc-windows-gnu

:: Build is outputted to PROJECT_ROOT/target/debug/*
powershell Write-Host "Building target..."
cargo build --target stable-x86_64-pc-windows-gnu

:: Move JSON files into built release folder
powershell Write-Host "Moving json files..."
powershell Unblock-File -Path ./lib/move_json_files.ps1
powershell ./lib/move_json_files.ps1

:: Move assets into built release folder
powershell Write-Host "Moving assets..."
powershell Unblock-File -Path ./lib/move_assets.ps1
powershell ./lib/move_assets.ps1

:: Move vc_runtime.bat into built release folder
powershell Write-Host "Adding vc_runtime.bat to release folder..."
powershell Unblock-File -Path ./lib/move_vc_runtime.ps1
powershell ./lib/move_vc_runtime.ps1