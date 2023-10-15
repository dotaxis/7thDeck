#!/bin/bash
export STEAM_COMPAT_APP_ID=39140
export STEAM_COMPAT_DATA_PATH="WINEPATH"
export STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root")

RUNTIME="${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier/run"
PROTON="${HOME}/.local/share/Steam/steamapps/common/Proton 7.0/proton"

[ ! -f "$RUNTIME" ] && { kdialog --error  "SteamLinuxRuntime not found!"; exit 1; }

[ ! -f "$PROTON" ] && { kdialog --error  "Proton 7.0 not found!"; exit 1; }

"$RUNTIME" -- "$PROTON" waitforexitandrun "7th Heaven.exe" $*
