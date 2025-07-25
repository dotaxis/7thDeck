#!/bin/bash
readonly SCRIPT="$(readlink -f "$0")"
readonly SCRIPT_DIR="$(dirname "$SCRIPT")"
readonly IS_STEAMOS=@STEAMOS@
STEAM_ROOT=@STEAM_ROOT@

. "$SCRIPT_DIR"/functions.sh

if [ $IS_STEAMOS = true ] ; then
  export STEAM_COMPAT_DATA_PATH=${HOME}/.steam/steam/steamapps/compatdata/39140
else
  export STEAM_COMPAT_DATA_PATH=$(LIBRARY=$(steam_library "$STEAM_ROOT" 39140) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/compatdata/39140" || echo "NONE")
fi

export STEAM_COMPAT_APP_ID=39140
export STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root")
export STEAM_COMPAT_MOUNTS="$(steam_library "$STEAM_ROOT" 2805730):$(steam_library "$STEAM_ROOT" 1628350):${STEAM_COMPAT_DATA_PATH%/steamapps/compatdata/39140}"
export WINEDLLOVERRIDES="dinput=n,b"
export DXVK_HDR=0
export PATH=$(echo "${PATH}" | sed -e "s|:$HOME/dotnet||")
unset DOTNET_ROOT

LIBRARY=$(steam_library "$STEAM_ROOT" 1628350)
if [ -n "$LIBRARY" ]; then
  for path in \
    "$LIBRARY/steamapps/common/SteamLinuxRuntime_sniper/run" \
    "$LIBRARY/steamapps/common/SteamLinuxRuntime_sniper-arm64/run"
  do
    [ -f "$path" ] && RUNTIME="$path" && break
  done
fi
PROTON=$(LIBRARY=$(steam_library "$STEAM_ROOT" 2805730) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/Proton 9.0 (Beta)/proton")

[ ! -d "$STEAM_COMPAT_DATA_PATH" ] && { prompt_message  "FF7 prefix not found!"; exit 1; }
[ -z "$RUNTIME" ] && { prompt_message  "Steam Linux Runtime not found!"; exit 1; }
[ -z "$PROTON" ] && { prompt_message  "Proton not found!"; exit 1; }


"$RUNTIME" -- "$PROTON" waitforexitandrun "$SCRIPT_DIR/7th Heaven.exe" $*
