@echo off

@REM Formats code with default formatter
cargo fmt

powershell Unblock-File -Path ./lib/json_formatting.ps1
powershell ./lib/json_formatting.ps1

powershell Unblock-File -Path ./lib/convert_endings.ps1
powershell ./lib/convert_endings.ps1