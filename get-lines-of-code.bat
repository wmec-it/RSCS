@echo off

@REM Counts lines of .rs file code.
Get-ChildItem -Filter "*.rs" -Recurse | Get-Content | Where-Object { $_.Trim() -notlike "//" -and $_.Trim() -ne "" } | Measure-Object -Line