param(
    [string]$Repo,
    [string]$Filter,
    [string]$ExtractTo
)

New-Item -Force $ExtractTo -ItemType "directory"

$Response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/tags/canary"
$Assets = $Response.assets
$downloadUri = $Assets | Where-Object { $_.name -like "*$Filter*" } | Select-Object -ExpandProperty browser_download_url
$pathZip = Join-Path -Path $ExtractTo -ChildPath $(Split-Path -Path $downloadUri -Leaf)

Invoke-WebRequest -Uri $downloadUri -Out $pathZip
Expand-Archive -Path $pathZip -DestinationPath $ExtractTo -Force
Remove-Item $pathZip -Force
