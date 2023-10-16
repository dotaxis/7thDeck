#!/bin/bash
export STEAM_COMPAT_APP_ID=39140
export STEAM_COMPAT_DATA_PATH="WINEPATH"
export STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root")

RUNTIME_HOME="${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier/run"
RUNTIME_SD="/run/media/mmcblk0p1/steamapps/common/SteamLinuxRuntime_soldier/run"
PROTON_HOME="${HOME}/.local/share/Steam/steamapps/common/Proton 7.0/proton"
PROTON_SD="/run/media/mmcblk0p1/steamapps/common/Proton 7.0/proton"

[ -f "$RUNTIME_HOME" ] && RUNTIME="$RUNTIME_HOME" || \
{ [ -f "$RUNTIME_SD" ] && RUNTIME="$RUNTIME_SD" || \
{ kdialog --error  "SteamLinuxRuntime not found!"; exit 1; }; }

[ -f "$PROTON_HOME" ] && PROTON="$PROTON_HOME" || \
{ [ -f "$PROTON_SD" ] && PROTON="$PROTON_SD" || \
{ kdialog --error  "Proton 7.0 not found!"; exit 1; }; }

"$RUNTIME" -- "$PROTON" waitforexitandrun "7th Heaven.exe" $*