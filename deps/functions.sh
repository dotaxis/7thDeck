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
  local REPO=$1
  local FILTER=$2
  local RETURN_VARIABLE=$3
  local RELEASE_URL=$(
    curl -s https://api.github.com/repos/"$REPO"/releases/latest  \
    | grep "browser_download_url.$FILTER" \
    | head -1 \
    | cut -d : -f 2,3 \
    | tr -d \")
  local FILENAME="temp/$(basename "$RELEASE_URL")"
  if [ -f "$FILENAME" ]; then
    echo "$FILENAME is ready to be installed."
  else
    echo "$FILENAME not found. Downloading..."
    curl -#SL -o $FILENAME $RELEASE_URL
  fi
  eval "${RETURN_VARIABLE}=\"$FILENAME\""
}
