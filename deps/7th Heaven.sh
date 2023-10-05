#!/bin/bash
export LD_LIBRARY_PATH="${HOME}/.local/share/Steam/steamapps/common/Proton 8.0/dist/lib:${HOME}/.local/share/Steam/steamapps/common/Proton 8.0/dist/lib64:$LD_LIBRARY_PATH"
export STEAM_COMPAT_DATA_PATH="${HOME}/.local/share/Steam/steamapps/compatdata/39140"
export STEAM_COMPAT_CLIENT_INSTALL_PATH="${HOME}/.local/share/Steam"
export STEAM_RUNTIME=0
/home/deck/.local/share/Steam/ubuntu12_32/reaper --filesystem /home/ SteamLaunch AppId=39140 -- '/home/deck/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier/_v2-entry-point' --verb=waitforexitandrun -- '/home/deck/.local/share/Steam/steamapps/common/Proton 8.0/proton' waitforexitandrun "7th Heaven.exe"