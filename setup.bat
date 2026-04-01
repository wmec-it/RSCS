:::::::::::::::::::::::
:: --- Dev Setup --- ::
:::::::::::::::::::::::

@echo off

:: Install Rustup dependencies
CALL :WINGET_INSTALL "winget" "Microsoft.VisualStudio.BuildTools"
CALL :WINGET_INSTALL "winget" "Microsoft.VCRedist.2015+.x86"
CALL :WINGET_INSTALL "winget" "Microsoft.VCRedist.2015+.x64"
CALL :WINGET_INSTALL "winget" "Microsoft.VCRedist.2013.x64"
CALL :WINGET_INSTALL "winget" "Microsoft.VCRedist.2013.x86"

:: Formatting dependencies
CALL :WINGET_INSTALL "winget" "waterlan.dos2unix"

:: Configure Rustup
CALL :RUSTUP_SETUP

:: Configure Rustup target
CALL :RUSTUP_TARGETS

:: Build
CALL build.bat

:: Finish
EXIT /b

:WINGET_INSTALL
winget install -e --source "%~1" --id "%~2" --silent --disable-interactivity --accept-package-agreements --accept-source-agreements >nul 2>&1
GOTO :EOF

:RUSTUP_SETUP
rustup toolchain install stable-x86_64-pc-windows-gnu >nul 2>&1
:: rustup toolchain install stable-x86_64-pc-windows-msvc >nul 2>&1
rustup default rustup default stable-x86_64-pc-windows-gnu >nul 2>&1
GOTO :EOF

:RUSTUP_TARGETS
rustup target add stable-x86_64-pc-windows-gnu >nul 2>&1
GOTO :EOF