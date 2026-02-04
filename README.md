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
