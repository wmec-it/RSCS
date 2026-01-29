#!/bin/bash

#:& Import lib
cd lib
source lib.sh
cd ../

#:& Check if user installed rustup / cargo
if ! command -v cargo >/dev/null 2>&1; then
    echo -e "\033[33mTrying to install rustup...\033[0m"
    #:& Check what package manager is used
    if command -v apt >/dev/null 2>&1; then
        sudo apt update >/dev/null 2>&1
        sudo apt install build-essential
        #:& Pull rustup from their website, since Ubuntu was having issues
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y >/dev/null 2>&1
        rustup default stable >/dev/null 2>&1
    elif command -v pacman >/dev/null 2>&1; then
        sudo pacman -Syu --needed rustup base-devel >/dev/null 2>&1
        rustup default stable >/dev/null 2>&1
    else
        echo -e "\e[4;31mYou don't have Rustup :(\e[0m"
        exit 1
    fi
fi

#:& Check if a cc linker is found on your OS
if ! command -v cc >/dev/null 2>&1; then
    echo -e "\e[33mTrying to install "cc" linker...\e[0m"
    #:& Check what package manager is used
    if command -v apt >/dev/null 2>&1; then
        sudo apt update >/dev/null 2>&1
        sudo apt install build-essential >/dev/null 2>&1
        source "$HOME/.cargo/env"
    elif command -v pacman >/dev/null 2>&1; then
        sudo pacman -Syu --needed base-devel >/dev/null 2>&1
    elif command -v dnf >/dev/null 2>&1; then
        # WARNING: Not tested
        sudo dnf groupinstall "Development Tools"
    elif command -v yum >/dev/null 2>&1; then
        # WARNING: Not tested
        sudo yum groupinstall "Development Tools"
    elif command -v zypper >/dev/null 2>&1; then
        # WARNING: Not tested
        sudo zypper install -t pattern devel_basis
    else
        echo -e "\e[31mYou need to install your platform's "cc" linker :(\e[0m"
        exit 1
    fi
fi

# WARNING: This should be moved to a conditional
echo -e "\e[0;33mSetting Rustup Toolchain version...\e[0m"
rustup default stable >/dev/null 2>&1
echo "Set!"

# TODO: Make this better
PS3='Please enter your choice: '
options=(
    "Normal Build"
    "No LTO Build"
    "Normal Build with Bambu Labs"
    "No LTO Build with Bambu Labs"
    "Quit"
)
select opt in "${options[@]}"
do
    case $opt in
        "Normal Build")
            cargo build --release
             break;;
        "No LTO Build")
            cargo build --release --profile no-lto-release
             break;;
        "Normal Build with Bambu Labs")
            cargo build --release --features bambulabs
             break;;
        "No LTO Build with Bambu Labs")
            cargo build --release --profile no-lto-release --features bambulabs
             break;;
        "Quit")
            echo "We're done"
            break;;
        *)
            echo "Ooops" $REPLY;;
    esac
done