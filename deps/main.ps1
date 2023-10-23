# Reinstall FF7
Write-Host "Reinstalling FF7..."
function GetRegistryValue($keyPath) {
    return Get-ItemProperty -Path $keyPath -Name "InstallLocation" -ErrorAction SilentlyContinue | Select-Object -ExpandProperty InstallLocation
}

$regKey32 = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Steam App 39140"
$regKey64 = "HKLM:\SOFTWARE\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Steam App 39140"

$path = GetRegistryValue $regKey32
if ([string]::IsNullOrEmpty($path)) { $path = GetRegistryValue $regKey64 }
if ([string]::IsNullOrEmpty($path)) {
    Write-Output "Couldn't detect FF7 install path. Aborting."
    Exit 1
}

Remove-Item -Path "$path" -Recurse -Force
Start-Process "steam://validate/39140"

# Install .NET Desktop 7
Write-Host "Installing dependencies..."
powershell.exe -executionpolicy bypass -command ".\deps\dotnet-install.ps1 -Runtime windowsdesktop -Architecture x86 -Version 7.0.12" | Out-Null
powershell.exe -executionpolicy bypass -command ".\deps\dotnet-install.ps1 -Runtime windowsdesktop -Architecture x64 -Version 7.0.12" | Out-Null

# Download and extract 7th Heaven
Write-Host "Downloading 7th Heaven..."
New-Item -Force -ItemType "directory" .\temp | Out-Null
powershell.exe -executionpolicy bypass -command '.\deps\github-download.ps1 -Repo "tsunamods-codes/7th-Heaven" -Filter ".zip" -ExtractTo "temp/7thHeaven"' | Out-Null
Move-Item -Path .\temp\7thHeaven -Destination "$env:USERPROFILE\Documents\7th Heaven" -Force
Remove-Item -Recurse -Force -Path .\temp

# Create Desktop Shortcut
$TargetFile = "$env:USERPROFILE\Documents\7th Heaven\7th Heaven.exe"
$ShortcutFile = "$env:USERPROFILE\Desktop\7th Heaven.lnk"
$WScriptShell = New-Object -ComObject WScript.Shell
$Shortcut = $WScriptShell.CreateShortcut($ShortcutFile)
$Shortcut.TargetPath = $TargetFile
$Shortcut.Save()

Write-Host "All done!"
Pause