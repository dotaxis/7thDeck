#!/bin/bash
. deps/functions.sh
export STEAM_COMPAT_MOUNTS="$(getSteamLibrary 2348590):$(getSteamLibrary 1391110):$(getSteamLibrary 39140)"
PROTON=$(LIBRARY=$(getSteamLibrary 2348590) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/Proton 8.0/proton" || echo "NONE")
RUNTIME=$(LIBRARY=$(getSteamLibrary 1391110) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/SteamLinuxRuntime_soldier/run" || echo "NONE")
FF7_LIBRARY=$(getSteamLibrary 39140 || echo "NONE")
XDG_DESKTOP_DIR=$(xdg-user-dir DESKTOP)
XDG_DATA_HOME="${XDG_DATA_HOME:=${HOME}/.local/share}"
IS_STEAMOS=$(grep -qi "SteamOS" /etc/os-release && echo true || echo false)

echo "" > "7thDeck.log"
exec > >(tee -ia "7thDeck.log") 2>&1

echo "########################################################################"
echo "#                             7thDeck v2.0                             #"
echo "########################################################################"
echo "#    This script will:                                                 #"
echo "#   1. Check for native protontricks, install flatpak if not found     #"
echo "#   2. Apply patches to FF7's proton prefix to accomodate 7th Heaven   #"
echo "#   3. Install 7th Heaven to a folder of your choosing                 #"
echo "#   4. Add 7th Heaven to Steam using a custom launcher script          #"
echo "#   5. Add a custom controller config for Steam Deck, to allow mouse   #"
echo "#      control with trackpad without holding down the STEAM button     #"
echo "########################################################################"
echo "#           For support, please open an issue on GitHub,               #"
echo "#   or ask in the #Steamdeck-Proton channel of the Tsunamods Discord   #"
echo "########################################################################"
echo -e "\n"


# Check for Proton 8
echo -n "Checking if Proton is installed... "
if [ "$PROTON" = "NONE" ]; then
  echo -e "\nNot found! Launching Steam to install."
  nohup steam steam://install/2348590 &> /dev/null &
  echo "Re-run this script when Proton 8 is done installing."
  read -p "Press Enter to close this window."
  kill -9 $PPID
fi
echo "OK!"
# Check for SteamLinuxRuntime
echo -n "Checking if SteamLinuxRuntime 2.0 is installed... "
if [ "$RUNTIME" = "NONE" ]; then
  echo -e "\nNot found! Launching Steam to install."
  nohup steam steam://install/1391110 &> /dev/null &
  echo "Re-run this script when SteamLinuxRuntime 2.0 (Soldier) is done installing."
  read -p "Press Enter to close this window."
  kill -9 $PPID
fi
echo "OK!"
# Check for FF7 and set paths
echo -n "Checking if FF7 is installed... "
if [ "$FF7_LIBRARY" = "NONE" ]; then
  echo -e "\nNot found! Launching Steam to install."
  nohup steam steam://install/39140 &> /dev/null &
  echo "Re-run this script when FINAL FANTASY VII is done installing."
  read -p "Press Enter to close this window."
  kill -9 $PPID
else
  FF7_DIR="$FF7_LIBRARY/steamapps/common/FINAL FANTASY VII"
  if [ $IS_STEAMOS = true ]; then
    WINEPATH=$(if [ -d "${HOME}/.local/share/Steam/steamapps/compatdata/39140/pfx" ]; \
    then echo "${HOME}/.local/share/Steam/steamapps/compatdata/39140/pfx"; \
    else echo "/run/media/mmcblk0p1/steamapps/compatdata/39140/pfx"; fi)
  else
    WINEPATH="$FF7_LIBRARY/steamapps/compatdata/39140/pfx"
  fi
fi
echo "OK!"
echo

# Force FF7 under Proton 8
echo "Rebuilding Final Fantasy VII under Proton 8.0..."
pkill -9 steam
cp ${XDG_DATA_HOME}/Steam/config/config.vdf ${XDG_DATA_HOME}/Steam/config/config.vdf.bak
perl -0777 -i -pe 's/"CompatToolMapping"\n\s+{/"CompatToolMapping"\n\t\t\t\t{\n\t\t\t\t\t"39140"\n\t\t\t\t\t{\n\t\t\t\t\t\t"name"\t\t"proton_8"\n\t\t\t\t\t\t"config"\t\t""\n\t\t\t\t\t\t"priority"\t\t"250"\n\t\t\t\t\t}/gs' \
${XDG_DATA_HOME}/Steam/config/config.vdf
while pgrep "steam" > /dev/null; do sleep 1; done
rm -rf "${WINEPATH%/pfx}"
echo "Sign into the Steam account that owns FF7 if prompted."
nohup steam steam://rungameid/39140 &> /dev/null &
echo "Waiting for Steam..."
while ! pgrep "FF7_Launcher" > /dev/null; do sleep 1; done
pkill -9 "FF7_Launcher"
echo

# Install protontricks and apply patches
if ! command -v protontricks > /dev/null; then
  echo "Native Protontricks is not installed. Using flatpak..."
  flatpak --system install com.github.Matoking.protontricks -y
  flatpak --system update com.github.Matoking.protontricks -y
  flatpak override --user --filesystem=host com.github.Matoking.protontricks
  shopt -s expand_aliases
  alias protontricks='flatpak run com.github.Matoking.protontricks'
fi
echo

# Ask for install path
promptUser "Choose an installation path for 7th Heaven. The folder must already exist."
while true; do
  INSTALL_PATH=$(promptDirectory "Select 7th Heaven Install Folder") || { echo "No directory selected. Exiting."; exit 1; }
  promptYesNo "7th Heaven will be installed to $INSTALL_PATH. Continue?"
  case $? in
    0) echo "Installing to $INSTALL_PATH."; break ;;
    1) echo "Select a different path." ;;
    -1) echo "An unexpected error has occurred. Exiting"; exit 1 ;;
  esac
done
echo

# Install dependencies and patch dinput
echo "Installing dependencies..."
echo "Please follow the installation prompts that appear."
echo "The script may appear to hang here. Be patient."
[ -f "$WINEPATH/drive_c/windows/syswow64/dinput.dll" ] && rm "$WINEPATH/drive_c/windows/syswow64/dinput.dll"
[ -f "$WINEPATH/drive_c/windows/system32/dinput.dll" ] && rm "$WINEPATH/drive_c/windows/system32/dinput.dll"
protontricks 39140 dinput &>> "7thDeck.log"
echo

# Download 7th Heaven from Github
echo "Downloading 7th Heaven..."
downloadDependency "tsunamods-codes/7th-Heaven" "*.exe" SEVENTH_INSTALLER
echo

# Install 7th Heaven using EXE
echo "Installing 7th Heaven..."
STEAM_COMPAT_APP_ID=39140 STEAM_COMPAT_DATA_PATH="${WINEPATH%/pfx}" \
STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root") \
"$RUNTIME" -- "$PROTON" waitforexitandrun \
$SEVENTH_INSTALLER /SILENT /DIR="Z:$INSTALL_PATH" &>> "7thDeck.log"

# Tweaks to 7th Heaven and FF7 directories
echo "Applying patches..."
mkdir "$INSTALL_PATH/7thWorkshop"
cp -f "$INSTALL_PATH/Resources/FF7_1.02_Eng_Patch/ff7.exe" "$FF7_DIR/ff7.exe"
cp -f deps/dxvk.conf "$INSTALL_PATH/"
cp -f "deps/7th Heaven.sh" "$INSTALL_PATH/"
cp -f deps/settings.xml "$INSTALL_PATH/7thWorkshop/"
sed -i "s|@RUNTIME_PATH@|$RUNTIME|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|@PROTON_PATH@|$PROTON|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|@MOUNTS@|$STEAM_COMPAT_MOUNTS|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|@WINEPATH@|${WINEPATH%/pfx}|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|<LibraryLocation>REPLACE_ME</LibraryLocation>|<LibraryLocation>Z:$INSTALL_PATH/mods</LibraryLocation>|" "$INSTALL_PATH/7thWorkshop/settings.xml"
sed -i "s|<FF7Exe>REPLACE_ME</FF7Exe>|<FF7Exe>Z:$FF7_DIR/ff7.exe</FF7Exe>|" "$INSTALL_PATH/7thWorkshop/settings.xml"
# Tweaks to proton prefix
cp -f "deps/timeout.exe" "$WINEPATH/drive_c/windows/system32/"
echo "FF7DISC1" > "$WINEPATH/drive_c/.windows-label"
echo "44000000" > "$WINEPATH/drive_c/.windows-serial"
echo

# SteamOS only
if [ $IS_STEAMOS = true ]; then
  # Steam Deck Auto-Config (mod)
  mkdir "$INSTALL_PATH/mods"
  cp -rf deps/SteamDeckSettings "$INSTALL_PATH/mods/"

  # This allows moving and clicking the mouse by using the right track-pad without holding down the STEAM button
  echo "Adding controller config..."
  cp -f deps/controller_neptune_gamepad+mouse+click.vdf ${HOME}/.steam/steam/controller_base/templates/controller_neptune_gamepad+mouse+click.vdf
  for CONTROLLERCONFIG in ${HOME}/.steam/steam/steamapps/common/Steam\ Controller\ Configs/*/config/configset_controller_neptune.vdf ; do
    if grep -q "\"39140\"" "$CONTROLLERCONFIG"; then
      perl -0777 -i -pe 's/"39140"\n\s+\{\n\s+"template"\s+"controller_neptune_gamepad_fps.vdf"\n\s+\}/"39140"\n\t\{\n\t\t"template"\t\t"controller_neptune_gamepad+mouse+click.vdf"\n\t\}\n\t"7th heaven"\n\t\{\n\t\t"template"\t\t"controller_neptune_gamepad+mouse+click.vdf"\n\t\}/gs' "$CONTROLLERCONFIG"
    else
      perl -0777 -i -pe 's/"controller_config"\n\{/"controller_config"\n\{\n\t"39140"\n\t\{\n\t\t"template"\t"controller_neptune_gamepad+mouse+click.vdf"\n\t\}\n\t"7th heaven"\n\t\{\n\t\t"template"\t"controller_neptune_gamepad+mouse+click.vdf"\n\t\}/' "$CONTROLLERCONFIG"
    fi
  done
  echo
fi

# Add shortcut to Desktop/Launcher
echo "Adding 7th Heaven to Desktop and Launcher..."
xdg-icon-resource install deps/7th-heaven.png --size 64 --novendor
mkdir -p "${XDG_DATA_HOME}/applications" &>> "7thDeck.log"
# Launcher
rm -r "${XDG_DATA_HOME}/applications/7th Heaven.desktop" 2> /dev/null
echo "#!/usr/bin/env xdg-open
[Desktop Entry]
Name=7th Heaven
Icon=7th-heaven
Exec=\"$INSTALL_PATH/7th Heaven.sh\"
Path=$INSTALL_PATH
Categories=Game;
Terminal=false
Type=Application
StartupNotify=false" > "${XDG_DATA_HOME}/applications/7th Heaven.desktop"
chmod +x "${XDG_DATA_HOME}/applications/7th Heaven.desktop"
# Desktop
rm -r "${XDG_DESKTOP_DIR}/7th Heaven.desktop" 2> /dev/null
echo "#!/usr/bin/env xdg-open
[Desktop Entry]
Name=7th Heaven
Icon=7th-heaven
Exec=\"$INSTALL_PATH/7th Heaven.sh\"
Path=$INSTALL_PATH
Categories=Game;
Terminal=false
Type=Application
StartupNotify=false" > "${XDG_DESKTOP_DIR}/7th Heaven.desktop"
chmod +x "${XDG_DESKTOP_DIR}/7th Heaven.desktop"
update-desktop-database ~/.local/share/applications &>> "7thDeck.log"
echo

# Add launcher to Steam
echo "Adding 7th Heaven to Steam..."
deps/steamos-add-to-steam "${XDG_DATA_HOME}/applications/7th Heaven.desktop" &>> "7thDeck.log"
sleep 5
echo

# Remove installer
rm $SEVENTH_INSTALLER

echo -e "All done!\nYou can close this window and launch 7th Heaven from Steam or the desktop now."
