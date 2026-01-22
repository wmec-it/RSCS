# Repair Shop Configuration Superintendant

For use configuring machines in the West-Mec IT Security Repair Shop (Nerd Heard).

## Dev Setup on Windows

1. [Rustup](https://rustup.rs/)
   - Or run: `winget install -e --id Rustlang.Rustup`
2. Visual Studio Build Tools
   ```powershell
   winget install -e --source winget --id Microsoft.VisualStudio.2022.Community --silent --override "--wait --quiet --add ProductLang En-us --add Microsoft.VisualStudio.Workload.NativeDesktop --includeRecommended"
   ```

## Dev Setup on Linux

After setting it up, you should also install `tokei` to see the repo's code stats.
```sh
tokei .
```

### Arch

```sh
sudo paru -S mingw-w64-gcc rustup
```

### Debian

```sh
sudo apt update
sudo apt install mingw-w64
```

### Fedora

```sh
sudo dnf install mingw64-gcc
```

## Builds

Run `build.*` depending on which OS you are on (Linux is `.sh`, and Windows is `.bat`).

### Release

Run `build-release.*` depending on which OS you are on (Linux is `.sh`, and Windows is `.bat`).

## Reference PS1 Script

```ps1
$WINGET_PROGRAMS = @('Inkscape.Inkscape')

function Write-InstallSuccess {
    param (
        [string[]]$ProgramName
    )

    Write-Host "Installed $ProgramName Successfully!" -ForegroundColor Green
}

function Write-InstallFailed {
    param (
        [string[]]$ProgramName
    )

    Write-Host "Failed installing $ProgramName..." -ForegroundColor Red
}
function Write-RemoveSuccess {
    param (
        [string[]]$ProgramName
    )

    Write-Host "Removed $ProgramName Successfully!" -ForegroundColor Green
}

function Write-RemoveFailed {
    param (
        [string[]]$ProgramName
    )

    Write-Host "Failed removing $ProgramName..." -ForegroundColor Red
}
function Check-CommandStatus {
    param (
        [string[]]$Type,
        [string[]]$ProgramName
    )

    if ($Type -eq "Install") {
        if ($?) {
            Write-InstallSuccess -ProgramName $ProgramName
        } else {
            Write-InstallFailed -ProgramName $ProgramName
        }
    }

    if ($Type -eq "Remove") {
        if ($?) {
            Write-RemoveSuccess -ProgramName $ProgramName
        } else {
            Write-RemoveFailed -ProgramName $ProgramName
        }
    }
}

function Install-WingetProgram {

    param (
        [string[]]$id
    )

    $ProgramName = $id -replace '^[^.]*\.', ''

    Write-Host "Installing $ProgramName..." -ForegroundColor Blue

    Start-Process -FilePath "winget" -ArgumentList "install -e --id $id --silent --disable-interactivity --accept-package-agreements --accept-source-agreements" -WindowStyle Hidden -Wait

    Check-CommandStatus -Type Install -ProgramName $ProgramName
}
function Remove-WingetProgram {
    param (
        [string[]]$id
    )

    $ProgramName = $id -replace '^[^.]*\.', ''

    Write-Host "Removing $ProgramName..." -ForegroundColor Blue

    Start-Process -FilePath "winget" -ArgumentList "remove -e --id $id" -WindowStyle Hidden -Wait

    Check-CommandStatus -Type Remove -ProgramName $ProgramName
}

function Sequence-Programs {
    param (
        [string[]]$Type
    )

    if ($Type -eq "Install") {
        Clear-Host

        $num_programs = $WINGET_PROGRAMS.Count
        Write-Host "Installing $num_programs programs..." -ForegroundColor Cyan
        Write-Host ""

        Install-WingetProgram -id Inkscape.Inkscape
    }

    if ($Type -eq "Remove") {
        Remove-WingetProgram -id Inkscape.Inkscape
    }
}

function Sequence-RemovePrograms {
    $num_programs = $REMOVE_PROGRAMS.Count
    Write-Host "Removing $num_programs programs..." -ForegroundColor Cyan
    Write-Host ""

    Remove-WingetProgram -id Microsoft.Teams
    Remove-WingetProgram -id Google.Chrome.EXE
    Remove-WingetProgram -id Google.GoogleDrive
}

function FullInstall {
    Sequence-Programs -Type Install
    Sequence-RemovePrograms
}


function Show-Menu {
    $menuOptions = @(
        '1: Full Install'
        '----- More Options ---------------'
        '2: Install Programs'
        '3: Remove Installed Programs'
        '4: Remove Unecessary Programs (or bad ones)'
        ''
        '0: Exit'
        '----- Press CTRL + C to exit -----'
        ''
    )

    Write-Host "West-Mec Repair Shop Configuration Tool" -ForegroundColor Yellow
    Write-Host ""

    foreach ($option in $menuOptions) {
        Write-Host $option
    }

    $choice = Read-Host 'Please select an option (0-3)'

    switch ($choice) {
        '1' { FullInstall }
        '2' { Sequence-Programs -Type Install }
        '3' { Sequence-Programs -Type Remove }
        '4' { Sequence-RemovePrograms }
        '0' { Write-Host 'Exiting...'; exit }
        default { Write-Host 'Invalid choice. Please try again.' }
    }
}

Show-Menu
```
