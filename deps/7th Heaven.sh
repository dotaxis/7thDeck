#!/bin/bash
export STEAM_COMPAT_APP_ID=39140
export STEAM_COMPAT_DATA_PATH="@WINEPATH@"
export STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root")
export STEAM_COMPAT_MOUNTS="@MOUNTS@"

RUNTIME="@RUNTIME_PATH@"
PROTON="@PROTON_PATH@"

[ ! -f "$RUNTIME" ] && { kdialog --error  "SteamLinuxRuntime not found!"; exit 1; }

[ ! -f "$PROTON" ] && { kdialog --error  "Proton not found!"; exit 1; }

"$RUNTIME" -- "$PROTON" waitforexitandrun "$PWD/7th Heaven.exe" $*
