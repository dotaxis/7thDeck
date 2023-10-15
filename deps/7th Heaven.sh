#!/bin/bash
export LD_LIBRARY_PATH="$PROTON_DIR/dist/lib:$PROTON_DIR/dist/lib64:$LD_LIBRARY_PATH"
export STEAM_COMPAT_DATA_PATH="WINEPATH"
export STEAM_COMPAT_CLIENT_INSTALL_PATH="${HOME}/.local/share/Steam"
export STEAM_RUNTIME=0

PROTON_HOME="${HOME}/.local/share/Steam/steamapps/common/Proton 8.0"
PROTON_SD="/run/media/mmcblk0p1/steamapps/common/Proton 8.0"
SOLDIER_HOME="${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier"
SOLDIER_SD="/run/media/mmcblk0p1/steamapps/common/SteamLinuxRuntime_soldier"

[ -d "$PROTON_HOME" ] && PROTON_DIR="$PROTON_HOME" || \
{ [ -d "$PROTON_SD" ] && PROTON_DIR="$PROTON_SD" || \
{ echo "Proton not found!"; exit 1; }; }

[ -d "$SOLDIER_HOME" ] && SOLDIER_DIR="$SOLDIER_HOME" || \
{ [ -d "$SOLDIER_SD" ] && SOLDIER_DIR="$SOLDIER_SD" || \
{ echo "Soldier not found!"; exit 1; }; }

${HOME}/.local/share/Steam/ubuntu12_32/reaper SteamLaunch AppId=39140 -- \
"$SOLDIER_DIR/_v2-entry-point" --verb=waitforexitandrun -- \
"$PROTON_DIR/proton" waitforexitandrun "7th Heaven.exe" $*