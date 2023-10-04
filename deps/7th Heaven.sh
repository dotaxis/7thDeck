#!/bin/bash
shopt -s expand_aliases
alias protontricks='flatpak run com.github.Matoking.protontricks'
alias protontricks-launch='flatpak run --command=protontricks-launch com.github.Matoking.protontricks'
protontricks-launch --no-bwrap --appid 39140 "/home/deck/7th Heaven/7th Heaven.exe"