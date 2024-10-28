# 7thDeck
Installer &amp; launcher for 7th Heaven on SteamOS and other Linux distros

<br>

## Purpose
This script will:
* Apply patches to FF7's proton prefix to accomodate 7th Heaven
* Install 7th Heaven to a folder of your choosing (you must create this directory beforehand)
* Add 7th Heaven to Steam and the KDE Launcher using a custom wrapper
* (SteamOS Only) Add a custom controller config, so you can control the mouse with the trackpad without holding down the STEAM button
* (SteamOS Only) Install a mod to 7th Heaven which automatically sets recommended graphics settings

<br>

## Requirements
* A fresh installation of FF7 via Steam

<br>

## Prerequisites

### Ubuntu (23.10 and later)
`bwrap` needs to have the proper permissions in AppArmor. Open the terminal and follow these steps:
1. Install `apparmor-profiles`
```bash
sudo apt install apparmor-profiles
```
2. Link the `bwrap` profile to AppArmor
```bash
sudo ln -s /usr/share/apparmor/extra-profiles/bwrap-userns-restrict /etc/apparmor.d/
```
3. Load the profile into AppArmor
```bash
sudo apparmor_parser /etc/apparmor.d/bwrap-userns-restrict
```

<br>

### Pop!_OS
`gawk` needs to be installed or you will encounter a syntax error when 7thDeck tries to detect FF7's installation. Open the terminal and follow these steps:
1. Install `gawk`
```bash
sudo apt install gawk
```

<br>

## Usage
1. Download and extract to a folder of your choosing
2. Run install.sh (Right-click -> Run in Konsole)
3. Launch 7th Heaven from Steam or Desktop Shortcut

<br>

## Support
* [Video Guide](https://www.youtube.com/watch?v=wNguRldtIqk)
* [Tsunamods Discord](https://discord.gg/tsunamods-community-277610501721030656)

<br>

## Donate
* https://ko-fi.com/dotaxis
