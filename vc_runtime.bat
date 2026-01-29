winget install -e --source winget --id Microsoft.VCRedist.2015+.x86 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements
winget install -e --source winget --id Microsoft.VCRedist.2015+.x64 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements
winget install -e --source winget --id Microsoft.VCRedist.2013.x64 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements
winget install -e --source winget --id Microsoft.VCRedist.2013.x86 --silent --disable-interactivity --accept-package-agreements --accept-source-agreements

winget upgrade Microsoft.VCRedist.2015+.x64
winget upgrade Microsoft.VCRedist.2015+.x86