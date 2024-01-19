#!/bin/bash

# Locate SteamLibrary containing app_id
getSteamLibrary() {
    local app_id="$1"

    # Use awk to find the path associated with the target value
    local path=$(awk -v target="$app_id" '
        BEGIN { RS = ""; ORS = "\n" }
        $0 ~ target {
            match($0, /"path"\s+"([^"]+)"/, arr)
            print arr[1]
            exit
        }' "${HOME}/.steam/root/steamapps/libraryfolders.vdf")

    echo $path
}

# Download from GitHub
downloadDependency() {
  local REPO=$1
  local FILTER=$2
  local RETURN_VARIABLE=$3
  local RELEASE_URL=$(
    curl -s https://api.github.com/repos/"$REPO"/releases  \
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
