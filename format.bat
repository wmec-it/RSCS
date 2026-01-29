@echo off

@REM Formats code with default formatter
cargo fmt

powershell ./lib/json_formatting.ps1