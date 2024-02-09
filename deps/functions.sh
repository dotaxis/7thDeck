#!/bin/bash

# Locate SteamLibrary containing app_id
getSteamLibrary() {
  local app_id="$1"

  local path=$(
    awk -v app_id="$app_id" '
      /^[[:space:]]*"[0-9]+"$/ {
        in_block = 1;
        block = $0;
        next;
      }
      in_block {
        block = block "\n" $0;
        if ($0 ~ /^\s*}/) {
          in_block = 0;
          if (block ~ app_id) {
            match(block, /"path"\s+"([^"]+)"/, arr);
            print arr[1];
            exit;
          }
        }
      }
    ' "${HOME}/.steam/root/steamapps/libraryfolders.vdf"
  )

  echo "$path"
}

# Download from GitHub
downloadDependency() {
  local XDG_CACHE_HOME="${XDG_CACHE_HOME:=${HOME}/.cache}"
  local REPO=$1
  local FILTER=$2
  local RETURN_VARIABLE=$3
  local RELEASE_URL=$(
    curl -s https://api.github.com/repos/"$REPO"/releases/latest  \
    | grep "browser_download_url.$FILTER" \
    | head -1 \
    | cut -d : -f 2,3 \
    | tr -d \")
  local FILENAME="${XDG_CACHE_HOME}/$(basename "$RELEASE_URL")"
  if [ -f "$FILENAME" ]; then
    echo "$FILENAME is ready to be installed."
  else
    echo "$FILENAME not found. Downloading..."
    curl -#SL -o $FILENAME $RELEASE_URL
  fi
  eval "${RETURN_VARIABLE}=\"$FILENAME\""
}

# Dialog compatibility
promptUser() {
  local message="$1"

  if command -v kdialog &> /dev/null; then
    kdialog --msgbox "$message" &> /dev/null
  elif command -v zenity &> /dev/null; then
    zenity --info --text="$message" &> /dev/null
  elif command -v dialog &> /dev/null; then
    dialog --msgbox "$message" 10 60
  fi
}

promptYesNo() {
  local message="$1"

  if command -v kdialog &> /dev/null; then
    kdialog --yesno "$message" &> /dev/null
  elif command -v zenity &> /dev/null; then
    zenity --question --text="$message" &> /dev/null
  elif command -v dialog &> /dev/null; then
    dialog --yesno "$message" 10 60
  fi
}

promptDirectory() {
  local title="$1"

  if command -v kdialog &> /dev/null; then
    cd ${HOME}
    echo $(kdialog --getexistingdirectory)
    cd - &> /dev/null
  elif command -v zenity &> /dev/null; then
    echo $(zenity --file-selection --directory)
  elif command -v dialog &> /dev/null; then
    echo $(dialog --dselect "${HOME}" 10 60 --stdout)
  fi
}
