$InstallPath = "$env:USERPROFILE\Documents\7th Heaven"

function GetRegistryValue($keyPath) {
    return Get-ItemProperty -Path $keyPath -Name "InstallLocation" -ErrorAction SilentlyContinue | Select-Object -ExpandProperty InstallLocation
}

$regKey32 = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Steam App 39140"
$regKey64 = "HKLM:\SOFTWARE\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Steam App 39140"

$gamePath = GetRegistryValue $regKey32
if ([string]::IsNullOrEmpty($gamePath)) { $gamePath = GetRegistryValue $regKey64 }
if ([string]::IsNullOrEmpty($gamePath)) {
    Write-Output "Couldn't detect FF7 install path. Aborting."
    Exit 1
}

# Reinstall FF7
Write-Host "Reinstalling FF7..."
Remove-Item -Path "$gamePath" -Recurse -Force
Start-Process "steam://validate/39140"

# Install .NET Desktop 7
Write-Host "Installing dependencies..."
powershell.exe -executionpolicy bypass -command '.\deps\dotnet-install.ps1 -Runtime windowsdesktop -Architecture x86 -Version 7.0.12' | Out-Null
powershell.exe -executionpolicy bypass -command '.\deps\dotnet-install.ps1 -Runtime windowsdesktop -Architecture x64 -Version 7.0.12' | Out-Null

# Download and extract 7th Heaven
Write-Host "Downloading 7th Heaven..."
New-Item -ItemType Directory ".\temp" -Force | Out-Null
powershell.exe -executionpolicy bypass -command '.\deps\github-download.ps1 -Repo "tsunamods-codes/7th-Heaven" -Filter ".zip" -ExtractTo "temp/7thHeaven"' | Out-Null
New-Item -ItemType Directory ".\temp\7thHeaven\7thWorkshop" -Force | Out-Null
Copy-Item -Path ".\deps\settings.xml" -Destination ".\temp\7thHeaven\7thWorkshop\settings.xml" -Force
(Get-Content -Path ".\temp\7thHeaven\7thWorkshop\settings.xml" -Raw) -replace "<LibraryLocation>REPLACE_ME</LibraryLocation>","<LibraryLocation>$InstallPath\mods</LibraryLocation>" `
| Set-Content -Path ".\temp\7thHeaven\7thWorkshop\settings.xml"
(Get-Content -Path ".\temp\7thHeaven\7thWorkshop\settings.xml" -Raw) -replace "<FF7Exe>REPLACE_ME</FF7Exe>","<FF7Exe>$gamePath\ff7.exe</FF7Exe>" `
| Set-Content -Path ".\temp\7thHeaven\7thWorkshop\settings.xml"
Copy-Item -Path ".\temp\7thHeaven\Resources\FF7_1.02_Eng_Patch\ff7.exe" -Destination "$gamePath\ff7.exe" -Force
Move-Item -Path ".\temp\7thHeaven" -Destination "$InstallPath" -Force
Remove-Item -Path ".\temp" -Recurse -Force

# Create Desktop Shortcut
Write-Host "Creating desktop shortcut..."
$TargetFile = "$env:USERPROFILE\Documents\7th Heaven\7th Heaven.exe"
$ShortcutFile = "$env:USERPROFILE\Desktop\7th Heaven.lnk"
$WScriptShell = New-Object -ComObject WScript.Shell
$Shortcut = $WScriptShell.CreateShortcut($ShortcutFile)
$Shortcut.TargetPath = $TargetFile
$Shortcut.Save()

Write-Host "All done!"
Pause
