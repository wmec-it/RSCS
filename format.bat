@echo off

@REM Formats code with default formatter
cargo fmt

powershell ./lib/json_formatting.ps1

powershell ./lib/convert_endings.ps1