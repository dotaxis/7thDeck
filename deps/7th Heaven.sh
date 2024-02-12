#!/bin/bash
unset DOTNET_ROOT
export PATH=$(echo "${PATH}" | sed -e 's|:/home/deck/dotnet||')
export STEAM_COMPAT_APP_ID=39140
export STEAM_COMPAT_DATA_PATH="@WINEPATH@"
export STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root")
export STEAM_COMPAT_MOUNTS="@MOUNTS@"
export WINEDLLOVERRIDES="dinput=n,b"


RUNTIME="@RUNTIME_PATH@"
PROTON="@PROTON_PATH@"

[ ! -f "$RUNTIME" ] && { kdialog --error  "SteamLinuxRuntime not found!"; exit 1; }

[ ! -f "$PROTON" ] && { kdialog --error  "Proton 7.0 not found!"; exit 1; }

"$RUNTIME" -- "$PROTON" waitforexitandrun "$PWD/7th Heaven.exe" $*
