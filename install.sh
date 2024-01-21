#!/bin/bash
. deps/functions.sh
export PRESSURE_VESSEL_FILESYSTEMS_RW="$(getSteamLibrary 1887720)"
PROTON=$(LIBRARY=$(getSteamLibrary 1887720) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/Proton 7.0/proton" || echo "NONE")
RUNTIME=$(LIBRARY=$(getSteamLibrary 1391110) && [ -n "$LIBRARY" ] && echo "$LIBRARY/steamapps/common/SteamLinuxRuntime_soldier/run" || echo "NONE")
XDG_DESKTOP_DIR=$(xdg-user-dir DESKTOP)
XDG_DATA_HOME="${XDG_DATA_HOME:=${HOME}/.local/share}"

echo "" > "7thDeck.log"
exec > >(tee -ia "7thDeck.log") 2>&1

echo "########################################################################"
echo "#                          7thDeck v1.2 (KDE)                          #"
echo "########################################################################"
echo "#    This script will:                                                 #"
echo "#    1. Verify protontricks is installed                               #"
echo "#    2. Apply patches to FF7's protonprefix to accomodate 7th Heaven   #"
echo "#    3. Install 7th Heaven to a folder of your choosing                #"
echo "#    4. Add 7th Heaven to Steam using a custom launcher script         #"
echo "#           For support, please open an issue on GitHub,               #"
echo "#   or ask in the #Steamdeck-Proton channel of the Tsunamods Discord   #"
echo "########################################################################"
echo -e "\n"

# Check for Proton 7 and SteamLinuxRuntime
echo -n "Checking if Proton 7 is installed... "
if [ "$PROTON" = "NONE" ]; then
  echo -e "\nNot found! Launching Steam to install."
  nohup steam steam://install/1887720 &> /dev/null &
  echo "Re-run this script when Proton 7 is done installing."
  read -p "Press Enter to close this window."
  kill -9 $PPID
fi
echo "OK!"
echo -n "Checking if SteamLinuxRuntime 2.0 is installed... "
if [ "$RUNTIME" = "NONE" ]; then
  echo -e "\nNot found! Launching Steam to install."
  nohup steam steam steam://install/1391110 &> /dev/null &
  echo "Re-run this script when SteamLinuxRuntime 2.0 (Soldier) is done installing."
  read -p "Press Enter to close this window."
  kill -9 $PPID
fi
echo "OK!"
echo

# Find FF7 and prefix
[ -d $(getSteamLibrary 39140)"/steamapps/compatdata/39140/pfx" ] && WINEPATH=$(getSteamLibrary 39140)"/steamapps/compatdata/39140/pfx" \
|| read -p "Enter the path to FF7's proton prefix (should end in '/39140/pfx'): " WINEPATH
[ ! -d "$WINEPATH" ] && { echo "Invalid proton prefix!"; exit 1; }

[ -d $(getSteamLibrary 39140)"/steamapps/common/FINAL FANTASY VII" ] && FF7_DIR=$(getSteamLibrary 39140)"/steamapps/common/FINAL FANTASY VII" \
|| read -p "Enter the path to your FF7 installation: " FF7_DIR
[ ! -d "$FF7_DIR" ] && { echo "Invalid FF7 path!"; exit 1; }

# Check if protontricks is installed
[ ! command -v protontricks &> /dev/null ] && { echo "Protontricks is not installed. Exiting."; exit 1; }

# Downgrade FF7 prefix to Proton 7.0
echo "Downgrading FF7 to Proton 7.0..."
STEAM_COMPAT_APP_ID=39140 STEAM_COMPAT_DATA_PATH="${WINEPATH%/pfx}" \
STEAM_COMPAT_CLIENT_INSTALL_PATH=$(readlink -f "$HOME/.steam/root") "$PROTON" run &>> "7thDeck.log"
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

# Apply patches to 7th Heaven and FF7
echo "Applying patches..."
mkdir "$INSTALL_PATH/7thWorkshop"
cp -f "$INSTALL_PATH/Resources/FF7_1.02_Eng_Patch/ff7.exe" "$FF7_DIR/ff7.exe"
cp -f deps/dxvk.conf "$INSTALL_PATH/"
cp -f "deps/7th Heaven.sh" "$INSTALL_PATH/"
cp -f deps/settings.xml "$INSTALL_PATH/7thWorkshop/"
sed -i "s|@RUNTIME_PATH@|$RUNTIME|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|@PROTON_PATH@|$PROTON|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|@MOUNTS@|$PRESSURE_VESSEL_FILESYSTEMS_RW|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|@WINEPATH@|${WINEPATH%/pfx}|" "$INSTALL_PATH/7th Heaven.sh"
sed -i "s|<LibraryLocation>REPLACE_ME</LibraryLocation>|<LibraryLocation>Z:$INSTALL_PATH/mods</LibraryLocation>|" "$INSTALL_PATH/7thWorkshop/settings.xml"
sed -i "s|<FF7Exe>REPLACE_ME</FF7Exe>|<FF7Exe>Z:$FF7_DIR/ff7.exe</FF7Exe>|" "$INSTALL_PATH/7thWorkshop/settings.xml"
cp -f "deps/timeout.exe" "$WINEPATH/drive_c/windows/system32/"
echo

# No-CD Fix
echo "FF7DISC1" > "$WINEPATH/drive_c/.windows-label"
echo "44000000" > "$WINEPATH/drive_c/.windows-serial"

# Add shortcut to Desktop/Launcher
echo "Adding 7th Heaven to Desktop and Launcher"
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

# Force FF7 under Proton 7
echo "Forcing Final Fantasy VII to run under Proton 7.0"
pkill -9 steam
cp ${XDG_DATA_HOME}/Steam/config/config.vdf ${XDG_DATA_HOME}/Steam/config/config.vdf.bak
perl -0777 -i -pe 's/"CompatToolMapping"\n\s+{/"CompatToolMapping"\n\t\t\t\t{\n\t\t\t\t\t"39140"\n\t\t\t\t\t{\n\t\t\t\t\t\t"name"\t\t"proton_7"\n\t\t\t\t\t\t"config"\t\t""\n\t\t\t\t\t\t"priority"\t\t"250"\n\t\t\t\t\t}/gs' \
${XDG_DATA_HOME}/Steam/config/config.vdf
# Thanks ChatGPT
nohup steam &> /dev/null &
echo

echo -e "All done!\nYou can close this window and launch 7th Heaven from Steam or the desktop now."
