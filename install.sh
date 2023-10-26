#!/bin/bash
shopt -s expand_aliases
alias winetricks='flatpak --command=winetricks run com.github.Matoking.protontricks'
WINEPATH=$(if [ -d "${HOME}/.local/share/Steam/steamapps/compatdata/39140/pfx" ]; then echo "${HOME}/.local/share/Steam/steamapps/compatdata/39140/pfx"; else echo "/run/media/mmcblk0p1/steamapps/compatdata/39140/pfx"; fi)
FF7_DIR=$(if [ -d "${HOME}/.local/share/Steam/steamapps/common/FINAL FANTASY VII" ]; then echo "${HOME}/.local/share/Steam/steamapps/common/FINAL FANTASY VII"; else echo "/run/media/mmcblk0p1/steamapps/common/FINAL FANTASY VII"; fi)
PROTON_HOME="${HOME}/.local/share/Steam/steamapps/common/Proton 7.0/proton"
PROTON_SD="/run/media/mmcblk0p1/steamapps/common/Proton 7.0/proton"
PROTON=""
RUNTIME_HOME="${HOME}/.local/share/Steam/steamapps/common/SteamLinuxRuntime_soldier/run"
RUNTIME_SD="/run/media/mmcblk0p1/steamapps/common/SteamLinuxRuntime_soldier/run"
RUNTIME=""

[ ! -d "temp" ] && mkdir temp

echo "########################################################################"
echo "#                             7thDeck v1.1                             #"
echo "########################################################################"
echo "#    This script will:                                                 #"
echo "#    1. Install protontricks from the Discover store                   #"
echo "#    2. Apply patches to FF7's protonprefix to accomodate 7th Heaven   #"
echo "#    3. Install 7th Heaven to a folder of your choosing                #"
echo "#    4. Add 7th Heaven to Steam using a custom launcher script         #"
echo "#           For support, please open an issue on GitHub,               #"
echo "#   or ask in the #Steamdeck-Proton channel of the Tsunamods Discord   #"
echo "########################################################################"
echo -e "\n"

# Check for Proton 7 and SteamLinuxRuntime
echo -n "Checking if Proton 7 is installed... "
while [ -z "$PROTON" ]; do
  if [ -f "$PROTON_HOME" ]; then
    PROTON="$PROTON_HOME"
  elif [ -f "$PROTON_SD" ]; then
    PROTON="$PROTON_SD"
  else
    echo -e "\nNot found! Launching Steam to install."
    steam steam://install/1887720 &> /dev/null
    read -p "Press Enter when Proton 7 is done installing."
  fi
done
echo "OK!"
echo -n "Checking if SteamLinuxRuntime 2.0 is installed... "
while [ -z "$RUNTIME" ]; do
  if [ -f "$RUNTIME_HOME" ]; then
    RUNTIME="$RUNTIME_HOME"
  elif [ -f "$RUNTIME_SD" ]; then
    RUNTIME="$RUNTIME_SD"
  else
    echo -e "\nNot found! Launching Steam to install."
    steam steam steam://install/1391110 &> /dev/null
    read -p "Press Enter when SteamLinuxRuntime 2.0 (Soldier) is done installing."
  fi
done
echo "OK!"
echo

# Downgrade FF7 prefix to Proton 7.0
echo "Downgrading FF7 to Proton 7.0..."
[ ! -d $WINEPATH ] && { echo "FF7 proton prefix not found! Have you run the game before? Exiting."; exit 1; }
STEAM_COMPAT_APP_ID=39140 STEAM_COMPAT_DATA_PATH="${WINEPATH%/pfx}" \
STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root") "$PROTON" run &> /dev/null
echo

# Ask for install path
kdialog --msgbox "Choose an installation path for 7th Heaven. The folder must already exist."
cd ${HOME}
while true; do
  INSTALL_PATH=$(kdialog --getexistingdirectory "Select 7th Heaven Install Folder") || { echo "No directory selected. Exiting."; exit 1; }
  kdialog --yesno "7th Heaven will be installed to $INSTALL_PATH. Continue?"
  case $? in
    0) echo "Installing to $INSTALL_PATH."; break ;;
    1) echo "Select a different path." ;;
    -1) echo "An unexpected error has occurred. Exiting"; exit 1 ;;
  esac
done
cd - &> /dev/null
echo

# Install protontricks and apply patches
echo "Installing Protontricks..."
flatpak --system install com.github.Matoking.protontricks -y
flatpak --system update com.github.Matoking.protontricks -y
flatpak override --user --filesystem=host com.github.Matoking.protontricks
echo

# Install dependencies and patch dinput
echo "Installing dependencies..."
echo "Please follow the installation prompts that appear."
echo "The script may appear to hang here. Be patient."
[ -f "$WINEPATH/drive_c/windows/syswow64/dinput.dll" ] && rm "$WINEPATH/drive_c/windows/syswow64/dinput.dll"
[ -f "$WINEPATH/drive_c/windows/system32/dinput.dll" ] && rm "$WINEPATH/drive_c/windows/system32/dinput.dll"
WINEPREFIX="$WINEPATH" WINESERVER="${PROTON%/proton}/dist/bin/wineserver" \
WINE="${PROTON%/proton}/dist/bin/wine" winetricks dinput dotnetdesktop7 &> /dev/null
echo

# Download 7th Heaven from Github
downloadDependency() {
  local REPO=$1
  local FILTER=$2
  local RETURN_VARIABLE=$3
  local RELEASE_URL=$(
    curl -s https://api.github.com/repos/"$REPO"/releases/tags/canary  \
    | grep "browser_download_url.$FILTER" \
    | head -1 \
    | cut -d : -f 2,3 \
    | tr -d \")
  local FILENAME="temp/$(basename "$RELEASE_URL")"
  if [ -f "$FILENAME" ]; then
    echo "$FILENAME is ready to be installed."
  else
    echo "$FILENAME not found. Downloading..."
    wget --show-progress -q -O $FILENAME $RELEASE_URL
  fi
  eval "${RETURN_VARIABLE}=\"$FILENAME\""
}
echo "Downloading 7th Heaven..."
downloadDependency "tsunamods-codes/7th-Heaven" "*.zip" ZIPFILE
echo

# Copy dxvk.conf and settings.xml
echo "Copying settings..."
mkdir -p "temp/7th Heaven/mods"
cp -rf deps/SteamDeckSettings "temp/7th Heaven/mods"
mkdir -p "temp/7th Heaven/7thWorkshop"
cp -f deps/settings.xml "temp/7th Heaven/7thWorkshop"
cp -f deps/dxvk.conf "temp/7th Heaven"
sed -i "s|<LibraryLocation>REPLACE_ME</LibraryLocation>|<LibraryLocation>Z:$INSTALL_PATH/mods</LibraryLocation>|" "temp/7th Heaven/7thWorkshop/settings.xml"
sed -i "s|<FF7Exe>REPLACE_ME</FF7Exe>|<FF7Exe>Z:$FF7_DIR/ff7.exe</FF7Exe>|" "temp/7th Heaven/7thWorkshop/settings.xml"
echo

# Extract 7th Heaven to chosen path
echo "Extracting 7th Heaven..."
unzip $ZIPFILE -d "temp/7th Heaven/" > /dev/null
cp -f "temp/7th Heaven/Resources/FF7_1.02_Eng_Patch/ff7.exe" "$FF7_DIR/ff7.exe"
cp -rf "temp/7th Heaven"/* "$INSTALL_PATH"
cp -f "deps/7th Heaven.sh" "$INSTALL_PATH"
sed -i "s|7th Heaven.exe|$INSTALL_PATH/7th Heaven.exe|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|WINEPATH|${WINEPATH%/pfx}|" "$INSTALL_PATH/7th Heaven.sh"
cp -f "deps/timeout.exe" "$WINEPATH/drive_c/windows/system32/"
echo

# No-CD Fix
echo "FF7DISC1" > "$WINEPATH/drive_c/.windows-label"
echo "44000000" > "$WINEPATH/drive_c/.windows-serial"

# Add shortcut to Desktop/Launcher
echo "Adding 7th Heaven to Desktop and Launcher"
xdg-icon-resource install deps/7th-heaven.png --size 64 --novendor
mkdir -p "${HOME}/.local/share/applications" &> /dev/null
# Launcher
rm -r "${HOME}/.local/share/applications/7th Heaven.desktop" 2> /dev/null
echo "#!/usr/bin/env xdg-open
[Desktop Entry]
Name=7th Heaven
Icon=7th-heaven
Exec=\"$INSTALL_PATH/7th Heaven.sh\"
Path=$INSTALL_PATH
Categories=Game;
Terminal=false
Type=Application
StartupNotify=false" > "${HOME}/.local/share/applications/7th Heaven.desktop"
chmod +x "${HOME}/.local/share/applications/7th Heaven.desktop"
# Desktop
rm -r "${HOME}/Desktop/7th Heaven.desktop" 2> /dev/null
echo "#!/usr/bin/env xdg-open
[Desktop Entry]
Name=7th Heaven
Icon=7th-heaven
Exec=\"$INSTALL_PATH/7th Heaven.sh\"
Path=$INSTALL_PATH
Categories=Game;
Terminal=false
Type=Application
StartupNotify=false" > "${HOME}/Desktop/7th Heaven.desktop"
chmod +x "${HOME}/Desktop/7th Heaven.desktop"
update-desktop-database ~/.local/share/applications
echo

# Add launcher to Steam
echo "Adding 7th Heaven to Steam..."
steamos-add-to-steam "${HOME}/.local/share/applications/7th Heaven.desktop" &> /dev/null
sleep 5
echo

# Force FF7 under Proton 7
echo "Forcing Final Fantasy VII to run under Proton 7.0"
kill $(ps aux | grep '[s]team -steamdeck' | awk '{print $2}')
sleep 10
cp ${HOME}/.local/share/Steam/config/config.vdf ${HOME}/.local/share/Steam/config/config.vdf.bak
perl -0777 -i -pe 's/"CompatToolMapping"\n\s+{/"CompatToolMapping"\n\t\t\t\t{\n\t\t\t\t\t"39140"\n\t\t\t\t\t{\n\t\t\t\t\t\t"name"\t\t"proton_7"\n\t\t\t\t\t\t"config"\t\t""\n\t\t\t\t\t\t"priority"\t\t"250"\n\t\t\t\t\t}/gs' \
${HOME}/.local/share/Steam/config/config.vdf
# Thanks ChatGPT
nohup steam &> /dev/null &
echo

# Clean up files
rm -r temp

echo -e "All done!\nYou can close this window and launch 7th Heaven from Steam or the desktop now."
