#!/bin/bash
export STEAM_COMPAT_DATA_PATH="WINEPATH"
export STEAM_COMPAT_CLIENT_INSTALL_PATH="${HOME}/.local/share/Steam"
export STEAM_RUNTIME=0

REAPER="${HOME}/.local/share/Steam/ubuntu12_32/reaper"
SNIPER_HOME="${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_sniper/_v2-entry-point"
SNIPER_SD="/run/media/mmcblk0p1/steamapps/common/SteamLinuxRuntime_sniper/_v2-entry-point"
PROTON_HOME="${HOME}/.local/share/Steam/steamapps/common/Proton 8.0/proton"
PROTON_SD="/run/media/mmcblk0p1/steamapps/common/Proton 8.0/proton"

[ ! -f "$REAPER" ] && { kdialog --error  "Reaper not found!"; exit 1; }

[ -f "$SNIPER_HOME" ] && SNIPER="$SNIPER_HOME" || \
{ [ -f "$SNIPER_SD" ] && SNIPER="$SNIPER_SD" || \
{ kdialog --error  "Sniper not found!"; exit 1; }; }

[ -f "$PROTON_HOME" ] && PROTON="$PROTON_HOME" || \
{ [ -f "$PROTON_SD" ] && PROTON="$PROTON_SD" || \
{ kdialog --error  "Proton not found!"; exit 1; }; }

"$REAPER" SteamLaunch AppId=39140 -- "$SNIPER" -- "$PROTON" waitforexitandrun \
"7th Heaven.exe" $*