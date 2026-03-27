@echo off

winget install -e --source winget --id Microsoft.VCRedist.2015+.x86 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements
winget install -e --source winget --id Microsoft.VCRedist.2015+.x64 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements
winget install -e --source winget --id Microsoft.VCRedist.2013.x64 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements
winget install -e --source winget --id Microsoft.VCRedist.2013.x86 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements

winget upgrade Microsoft.VCRedist.2015+.x64
winget upgrade Microsoft.VCRedist.2015+.x86

winget install -e --source winget --id waterlan.dos2unix --silent --disable-interactivity --accept-package-agreements --accept-source-agreements
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
