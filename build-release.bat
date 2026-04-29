@echo off

@REM Install missing dependencies needed to build
powershell Write-Host "Installing build dependencies..."
CALL install_deps.bat

@REM Makes sure you have the right target installed
powershell Write-Host "Adding correct target..."
rustup target add x86_64-pc-windows-gnu

@REM Release build is outputted to PROJECT_ROOT/target/release/*
powershell Write-Host "Building target..."
cargo build --release --target x86_64-pc-windows-gnu

@REM Move JSON files into built release folder
setlocal
set JSON_SCRIPT=./lib/move_json_files.ps1
endlocal
powershell Write-Host "Moving json files..."
powershell Unblock-File -Path %JSON_SCRIPT%
powershell %JSON_SCRIPT%

@REM Move assets into built release folder
setlocal
set ASSETS_SCRIPT=./lib/move_assets.ps1
endlocal
powershell Write-Host "Moving assets..."
powershell Unblock-File -Path %ASSETS_SCRIPT%
powershell %ASSETS_SCRIPT%

@REM Move vc_runtime.bat into built release folder
setlocal
set VC_RUNTIME_SCRIPT=./lib/move_vc_runtime.ps1
endlocal
powershell Write-Host "Adding vc_runtime.bat to release folder..."
powershell Unblock-File -Path %VC_RUNTIME_SCRIPT%
powershell %VC_RUNTIME_SCRIPT%
