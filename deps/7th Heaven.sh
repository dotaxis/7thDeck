#!/bin/bash
PROTON_DIR=$(if [ -d "${HOME}/.local/share/Steam/steamapps/common/Proton 8.0" ]; then echo "${HOME}/.local/share/Steam/steamapps/common/Proton 8.0"; else echo "/run/media/mmcblk0p1/steamapps/common/Proton 8.0"; fi)
SOLDIER_DIR=$(if [ -d "${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier" ]; then echo "${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier"; else echo "/run/media/mmcblk0p1/steamapps/common/SteamLinuxRuntime_soldier"; fi)
[ ! -d "$PROTON_DIR" ] && echo "Proton not found!"
[ ! -d "$SOLDIER_DIR" ] && echo "SteamLinuxRuntime Soldier not found!"
export LD_LIBRARY_PATH="$PROTON_DIR/dist/lib:$PROTON_DIR/dist/lib64:$LD_LIBRARY_PATH"
export STEAM_COMPAT_DATA_PATH="${HOME}/.local/share/Steam/steamapps/compatdata/39140"
export STEAM_COMPAT_CLIENT_INSTALL_PATH="${HOME}/.local/share/Steam"
export STEAM_RUNTIME=0
${HOME}/.local/share/Steam/ubuntu12_32/reaper --filesystem /home/ SteamLaunch AppId=39140 -- "$SOLDIER_DIR/_v2-entry-point" --verb=waitforexitandrun -- "$PROTON_DIR/proton" waitforexitandrun "7th Heaven.exe"