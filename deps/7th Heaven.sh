#!/bin/bash
export STEAM_COMPAT_APP_ID=39140
export STEAM_COMPAT_DATA_PATH="WINEPATH"
export STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root")

SNIPER_HOME="${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier/run"
SNIPER_SD="/run/media/mmcblk0p1/steamapps/common/SteamLinuxRuntime_soldier/run"
PROTON_HOME="${HOME}/.local/share/Steam/steamapps/common/Proton 7.0/proton"
PROTON_SD="/run/media/mmcblk0p1/steamapps/common/Proton 7.0/proton"

[ -f "$SNIPER_HOME" ] && SNIPER="$SNIPER_HOME" || \
{ [ -f "$SNIPER_SD" ] && SNIPER="$SNIPER_SD" || \
{ kdialog --error  "Sniper not found!"; exit 1; }; }

[ -f "$PROTON_HOME" ] && PROTON="$PROTON_HOME" || \
{ [ -f "$PROTON_SD" ] && PROTON="$PROTON_SD" || \
{ kdialog --error  "Proton not found!"; exit 1; }; }

"$SNIPER" -- "$PROTON" waitforexitandrun "7th Heaven.exe" $*