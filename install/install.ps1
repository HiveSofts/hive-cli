$ErrorActionPreference = "Stop"


$Repo = "HiveSofts/hive-cli"
$Binary = "hive.exe"


Write-Host ""
Write-Host "🐝 Hive CLI Installer"
Write-Host "━━━━━━━━━━━━━━━━━━━━"
Write-Host ""


$Release = Invoke-RestMethod `
"https://api.github.com/repos/$Repo/releases/latest"


$Version = $Release.tag_name


Write-Host "Version: $Version"
Write-Host ""


$Asset = "hive-windows.exe"


$DownloadUrl = `
"https://github.com/$Repo/releases/download/$Version/$Asset"



$temp = Join-Path `
$env:TEMP `
$Binary



Write-Host "⬇️ Downloading Hive..."

Invoke-WebRequest `
-Uri $DownloadUrl `
-OutFile $temp



$InstallDir = "$env:USERPROFILE\.hive\bin"


if (!(Test-Path $InstallDir)) {
    New-Item `
    -ItemType Directory `
    -Path $InstallDir | Out-Null
}



$Target = Join-Path `
$InstallDir `
$Binary



Move-Item `
-Force `
$temp `
$Target



Write-Host ""
Write-Host "⚙️ Updating PATH..."



$currentPath = [Environment]::GetEnvironmentVariable(
"Path",
"User"
)


if ($currentPath -notlike "*$InstallDir*") {

    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$InstallDir",
        "User"
    )

}



Write-Host ""
Write-Host "✅ Hive installed successfully"
Write-Host ""

Write-Host "Restart terminal then run:"
Write-Host ""

Write-Host "  hive --help"
