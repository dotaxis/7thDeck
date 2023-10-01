#!/bin/bash
shopt -s expand_aliases
alias protontricks='flatpak run com.github.Matoking.protontricks'
WINEPATH="/home/deck/.local/share/Steam/steamapps/compatdata/39140/pfx"
FF7_DIR=$(if [ -d "${HOME}/.local/share/Steam/steamapps/common/FINAL FANTASY VII" ]; then echo "${HOME}/.local/share/Steam/steamapps/common/FINAL FANTASY VII"; else echo "/run/media/mmcblk0p1/steamapps/common/FINAL FANTASY VII"; fi)
mkdir temp

echo -e "7thDeck v0.1 \n"
echo -e "########################################################################"
echo -e "#    This script will:"
echo -e "#    1. Install protontricks from the Discover store"
echo -e "#    2. Apply patches to FF7's protonprefix to accomodate 7th Heaven"
echo -e "#    3. Install 7th Heaven to a folder of your choosing
#       Default: '/home/deck/7th Heaven'"
echo -e "#    4. Add 7th Heaven to Steam using a custom launcher script"
echo -e "#    For support, please open an issue on GitHub,
#    or ask in the #Steamdeck-Proton channel of the Tsunamods Discord"
echo -e "########################################################################\n\n"


# Ask for install path
while true; do
    read -p "Where would you like to install 7th Heaven? Press enter for default: " INSTALL_PATH
    if [ -z "$INSTALL_PATH" ]; then
        INSTALL_PATH="${HOME}/7th Heaven"
    fi

    if [ -e "$INSTALL_PATH" ]; then
        echo "The installation path '$INSTALL_PATH' already exists."
        echo "Please choose a different path or delete '$INSTALL_PATH'"
    else
        break
    fi
done
echo -e "7th Heaven will be installed to $INSTALL_PATH\n"

# Install protontricks and apply patches
echo "Installing Protontricks..."
flatpak --system install com.github.Matoking.protontricks -y
flatpak override --user --filesystem=/home/ com.github.Matoking.protontricks
flatpak override --user --filesystem=/run/media/mmcblk0p1 com.github.Matoking.protontricks
echo

# Install dependencies and patch dinput
echo "Installing dependencies..."
echo "Please follow the installation prompts that appear."
echo "The script may appear to hang here. Be patient."
[ -f "$WINEPATH/drive_c/windows/syswow64/dinput.dll" ] && rm "$WINEPATH/drive_c/windows/syswow64/dinput.dll"
[ -f "$WINEPATH/drive_c/windows/system32/dinput.dll" ] && rm "$WINEPATH/drive_c/windows/system32/dinput.dll"
protontricks 39140 dinput dxvk dotnetdesktop7 &> /dev/null
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
[ -f "$INSTALL_PATH" ] && rm "$INSTALL_PATH"
cp -rf "temp/7th Heaven" "$INSTALL_PATH"
cp -f "deps/7th Heaven.sh" "$INSTALL_PATH"
sed -i "s|7th Heaven.exe|$INSTALL_PATH/7th Heaven.exe|" "$INSTALL_PATH/7th Heaven.sh"
cp -f "deps/timeout.exe" "$WINEPATH/drive_c/windows/system32/"
echo

# No-CD Fix
echo "FF7DISC1" > "$WINEPATH/drive_c/.windows-label"
echo "44000000" > "$WINEPATH/drive_c/.windows-serial"

# Add shortcut to Desktop/Launcher
echo "Adding 7th Heaven to Desktop and Launcher"
xdg-icon-resource install deps/7th-heaven.png --size 64 --novendor
# Launcher
rm -r "${HOME}/.local/share/applications/7th Heaven.desktop" 2> /dev/null
echo "#!/usr/bin/env xdg-open
[Desktop Entry]
Name=7th Heaven
Icon=7th-heaven
Exec=\"$INSTALL_PATH/7th Heaven.sh\"
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

# Clean up files
rm -r temp

echo -e "All done!\nYou can close this window and launch 7th Heaven from Steam or the desktop now."