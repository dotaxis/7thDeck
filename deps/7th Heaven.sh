#!/bin/bash
. functions.sh

IS_STEAMOS=@STEAMOS@
if [ $IS_STEAMOS = true ] ; then
  export STEAM_COMPAT_DATA_PATH=${HOME}/.steam/steam/steamapps/compatdata/39140
else
  export STEAM_COMPAT_DATA_PATH=$(LIBRARY=$(getSteamLibrary 39140) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/compatdata/39140" || echo "NONE")
fi

export STEAM_COMPAT_APP_ID=39140
export STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root")
export STEAM_COMPAT_MOUNTS="$(getSteamLibrary 1493710):$(getSteamLibrary 1628350):${STEAM_COMPAT_DATA_PATH%/steamapps/compatdata/39140}"
export WINEDLLOVERRIDES="dinput=n,b"
export DXVK_HDR=0
export PATH=$(echo "${PATH}" | sed -e "s|:$HOME/dotnet||")
unset DOTNET_ROOT

RUNTIME=$(LIBRARY=$(getSteamLibrary 1628350) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/SteamLinuxRuntime_sniper/run" || echo "NONE")
PROTON=$(LIBRARY=$(getSteamLibrary 1493710) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/Proton - Experimental/proton" || echo "NONE")

[ ! -d "$STEAM_COMPAT_DATA_PATH" ] && { promptUser  "FF7 prefix not found!"; exit 1; }
[ "$RUNTIME" = "NONE" ] && { promptUser  "Steam Linux Runtime not found!"; exit 1; }
[ "$PROTON" = "NONE" ] && { promptUser  "Proton not found!"; exit 1; }


"$RUNTIME" -- "$PROTON" waitforexitandrun "$PWD/7th Heaven.exe" $*
