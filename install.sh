#!/bin/bash
. deps/functions.sh
XDG_DESKTOP_DIR=$(xdg-user-dir DESKTOP)
XDG_DATA_HOME="${XDG_DATA_HOME:=${HOME}/.local/share}"
IS_STEAMOS=$(grep -qi "SteamOS" /etc/os-release && echo true || echo false)
STEAM_ROOT=$(readlink -f "$HOME/.steam/root")
STEAM_BIN=steam

echo "" > "7thDeck.log"
exec > >(tee -ia "7thDeck.log") 2>&1

echo "########################################################################"
echo "#                            7thDeck v2.5.1                            #"
echo "########################################################################"
echo "#    This script will:                                                 #"
echo "#   1. Apply patches to FF7's proton prefix to accomodate 7th Heaven   #"
echo "#   2. Install 7th Heaven to a folder of your choosing                 #"
echo "#   3. Add 7th Heaven to Steam using a custom launcher script          #"
echo "#   4. Add a custom controller config for Steam Deck, to allow mouse   #"
echo "#      control with trackpad without holding down the STEAM button     #"
echo "########################################################################"
echo "#           For support, please open an issue on GitHub,               #"
echo "#      or ask in the #ff7-linux channel of the Tsunamods Discord       #"
echo "########################################################################"
echo -e "\n"

# Handle Flatpak Steam
if grep -q Steam <(flatpak list --app) && ! command -v steam &> /dev/null; then
  echo "Using Flatpak Steam." &>> "7thDeck.log"
  STEAM_ROOT="$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam"
  STEAM_BIN="flatpak run com.valvesoftware.Steam"
elif grep -q Steam <(flatpak list --app) && command -v steam &> /dev/null; then
  echo "Both Flatpak and native Steam installations have been detected."
  echo "Press 1 to use Flatpak Steam."
  echo "Press 2 to use native Steam."
  while true; do
      read -p "" input
      if [[ $input != "1" && $input != "2" ]]; then
          continue
      fi
      case $input in
          1)
              echo "Proceeding with Flatpak Steam installation"
              STEAM_ROOT="$HOME/.var/app/com.valvesoftware.Steam/.local/share/Steam"
              STEAM_BIN="flatpak run com.valvesoftware.Steam"
              break;;
          2)
              echo "Proceeding with native Steam installation."
              break;;
          *)
              continue;;
      esac
  done
fi

# Check for Proton
while true; do
  if ! pgrep steam > /dev/null; then nohup $STEAM_BIN &> /dev/null & disown; fi
  while ! pgrep steam > /dev/null; do sleep 1; done
  PROTON=$(LIBRARY=$(steam_library "$STEAM_ROOT" 2805730) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/Proton 9.0 (Beta)/proton" || echo "NONE")
  echo -n "Checking if Proton 9 is installed... "
  if [ "$PROTON" = "NONE" ]; then
    echo -e "\nNot found! Launching Steam to install."
    nohup $STEAM_BIN steam://install/2805730 &> /dev/null &
    read -p "Press Enter when Proton 9 is done installing."
    killall -9 steam
    while pgrep steam >/dev/null; do sleep 1; done
    rm $STEAM_ROOT/steamapps/libraryfolders.vdf &>> "7thDeck.log"
    rm $STEAM_ROOT/config/libraryfolders.vdf &>> "7thDeck.log"
  else
    echo "OK!"
    echo "Found Proton at $PROTON!"
    echo
    break
  fi
done

# Check for SteamLinuxRuntime
while true; do
  if ! pgrep steam > /dev/null; then nohup $STEAM_BIN &> /dev/null; fi
  while ! pgrep steam > /dev/null; do sleep 1; done
  RUNTIME=$(LIBRARY=$(steam_library "$STEAM_ROOT" 1628350) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/SteamLinuxRuntime_sniper/run" || echo "NONE")
  echo -n "Checking if Steam Linux Runtime is installed... "
  if [ "$RUNTIME" = "NONE" ]; then
    echo -e "\nNot found! Launching Steam to install."
    nohup $STEAM_BIN steam://install/1628350 &> /dev/null &
    read -p "Press Enter when Steam Linux Runtime 3.0 (sniper) is done installing."
    killall -9 steam
    while pgrep steam >/dev/null; do sleep 1; done
    rm $STEAM_ROOT/steamapps/libraryfolders.vdf &>> "7thDeck.log"
    rm $STEAM_ROOT/config/libraryfolders.vdf &>> "7thDeck.log"
  else
    echo "OK!"
    echo "Found SLR at $RUNTIME!"
    echo
    break
  fi
done

# Check for FF7 and set paths
while true; do
  if ! pgrep steam > /dev/null; then nohup $STEAM_BIN &> /dev/null; fi
  while ! pgrep steam > /dev/null; do sleep 1; done
  echo -n "Checking if FF7 is installed... "
  FF7_LIBRARY=$(steam_library "$STEAM_ROOT" 39140 || echo "NONE")
  if [ "$FF7_LIBRARY" = "NONE" ]; then
    echo -e "\nNot found! Launching Steam to install."
    nohup $STEAM_BIN steam://install/39140 &> /dev/null &
    read -p "Press Enter when FINAL FANTASY VII is done installing."
    killall -9 steam
    while pgrep steam > /dev/null; do sleep 1; done
    rm $STEAM_ROOT/steamapps/libraryfolders.vdf &>> "7thDeck.log"
    rm $STEAM_ROOT/config/libraryfolders.vdf &>> "7thDeck.log"
  else
    echo "OK!"
    echo "Found FF7 at $FF7_LIBRARY!"
    echo
    break
  fi
done

# Kill Steam for next steps
echo "Closing Steam..."
while pidof "steam" > /dev/null; do
  killall -9 steam &>> "7thDeck.log"
  sleep 1
done
echo

# Set paths and compat_mounts after libraries have been properly detected
FF7_DIR="$FF7_LIBRARY/steamapps/common/FINAL FANTASY VII"
WINEPATH="$FF7_LIBRARY/steamapps/compatdata/39140/pfx"
[ $IS_STEAMOS = true ] && WINEPATH="${HOME}/.steam/steam/steamapps/compatdata/39140/pfx"
export STEAM_COMPAT_MOUNTS="$(steam_library "$STEAM_ROOT" 2805730):$(steam_library "$STEAM_ROOT" 1628350):$(steam_library "$STEAM_ROOT" 39140)"

# Force FF7 under Proton 9
echo "Rebuilding Final Fantasy VII under Proton 9..."
while pidof "steam" > /dev/null; do
  killall -9 steam &>> "7thDeck.log"
  sleep 1
done
cp $STEAM_ROOT/config/config.vdf $STEAM_ROOT/config/config.vdf.bak
perl -0777 -i -pe 's/"CompatToolMapping"\n\s+{/"CompatToolMapping"\n\t\t\t\t{\n\t\t\t\t\t"39140"\n\t\t\t\t\t{\n\t\t\t\t\t\t"name"\t\t"proton_9"\n\t\t\t\t\t\t"config"\t\t""\n\t\t\t\t\t\t"priority"\t\t"250"\n\t\t\t\t\t}/gs' \
$STEAM_ROOT/config/config.vdf
[ "${WINEPATH}" = */compatdata/39140/pfx ] && rm -rf "${WINEPATH%/pfx}"/*
echo "Sign into the Steam account that owns FF7 if prompted."
nohup $STEAM_BIN steam://rungameid/39140 &> /dev/null &
echo "Waiting for Steam... If nothing happens here, launch FF7 via Steam."
while ! pgrep "FF7_Launcher" > /dev/null; do sleep 1; done
killall -9 "FF7_Launcher.exe"
echo

# Fix infinite loop on "Verifying installed game is compatible"
[ -L "$FF7_DIR/FINAL FANTASY VII" ] && unlink "$FF7_DIR/FINAL FANTASY VII"

# Ask for install path
echo "Waiting for you to select an installation path..."
prompt_message "Choose an installation path for 7th Heaven. The folder must already exist."
while true; do
  INSTALL_PATH=$(prompt_destination "Select 7th Heaven Install Folder") || { echo "No directory selected. Exiting."; exit 1; }
  prompt_confirm "7th Heaven will be installed to $INSTALL_PATH. Continue?"
  case $? in
    0) echo "Installing to $INSTALL_PATH."; break ;;
    1) echo "Select a different path." ;;
    -1) echo "An unexpected error has occurred. Exiting"; exit 1 ;;
  esac
done
echo

# Download 7th Heaven from Github
echo "Downloading 7th Heaven..."
download_dependency "tsunamods-codes/7th-Heaven" "*.exe" SEVENTH_INSTALLER
echo

# Install 7th Heaven using EXE
echo "Installing 7th Heaven..."
mkdir -p "${WINEPATH}/drive_c/ProgramData" # fix vcredist install - infirit
STEAM_COMPAT_APP_ID=39140 STEAM_COMPAT_DATA_PATH="${WINEPATH%/pfx}" \
STEAM_COMPAT_CLIENT_INSTALL_PATH="$STEAM_ROOT" \
"$RUNTIME" -- "$PROTON" waitforexitandrun \
"$SEVENTH_INSTALLER" /VERYSILENT /DIR="Z:$INSTALL_PATH" /LOG="7thHeaven.log" &>> "7thDeck.log"
cp launch_options.txt "$INSTALL_PATH/"
echo

# Tweaks to 7th Heaven install directory
echo "Applying patches to 7th Heaven..."
mkdir -p "$INSTALL_PATH/7thWorkshop/profiles"
cp -f "deps/7th Heaven.sh" "$INSTALL_PATH/"
cp -f "deps/functions.sh" "$INSTALL_PATH/"
cp -f "deps/dxvk.conf" "$INSTALL_PATH/"
cp -f deps/settings.xml "$INSTALL_PATH/7thWorkshop/"
[ ! -f "$INSTALL_PATH/7thWorkshop/profiles/Default.xml" ] && cp "deps/Default.xml" "$INSTALL_PATH/7thWorkshop/profiles/" &>> "7thDeck.log"
sed -i "s|@STEAM_ROOT@|$STEAM_ROOT|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|@STEAMOS@|$IS_STEAMOS|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|<LibraryLocation>REPLACE_ME</LibraryLocation>|<LibraryLocation>Z:$INSTALL_PATH/mods</LibraryLocation>|" "$INSTALL_PATH/7thWorkshop/settings.xml"
sed -i "s|<FF7Exe>REPLACE_ME</FF7Exe>|<FF7Exe>Z:$FF7_DIR/ff7_en.exe</FF7Exe>|" "$INSTALL_PATH/7thWorkshop/settings.xml"
echo

# Tweaks to game
echo "Applying patches to FF7..."
cp -f "deps/timeout.exe" "$WINEPATH/drive_c/windows/system32/"
echo "FF7DISC1" > "$WINEPATH/drive_c/.windows-label"
echo "44000000" > "$WINEPATH/drive_c/.windows-serial"
echo

# SteamOS only
if [ $IS_STEAMOS = true ]; then
  # Steam Deck Auto-Config (mod)
  mkdir -p "$INSTALL_PATH/mods"
  cp -rf deps/SteamDeckSettings "$INSTALL_PATH/mods/"

  # This allows moving and clicking the mouse by using the right track-pad without holding down the STEAM button
  echo "Adding controller config..."
  cp -f deps/controller_neptune_gamepad+mouse+click.vdf $STEAM_ROOT/controller_base/templates/controller_neptune_gamepad+mouse+click.vdf
  for CONTROLLERCONFIG in $STEAM_ROOT/steamapps/common/Steam\ Controller\ Configs/*/config/configset_controller_neptune.vdf ; do
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
update-desktop-database ${XDG_DATA_HOME}/applications &>> "7thDeck.log"
echo

# Add launcher to Steam
echo "Adding 7th Heaven to Steam..."
deps/steamos-add-to-steam "${INSTALL_PATH}/7th Heaven.sh" "$STEAM_BIN" &>> "7thDeck.log"
sleep 5
echo

echo -e "All done!\nYou can close this window and launch 7th Heaven from Steam or the desktop now."
