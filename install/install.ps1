$ErrorActionPreference = "Stop"

$Repo = "HiveSofts/hive-cli"
$Binary = "hive.exe"

# Colors
$Green = [System.ConsoleColor]::Green
$Yellow = [System.ConsoleColor]::Yellow
$Cyan = [System.ConsoleColor]::Cyan
$Red = [System.ConsoleColor]::Red
$Blue = [System.ConsoleColor]::Blue

Write-Host ""
Write-Host "  🐝 Hive CLI Installer" -ForegroundColor $Cyan -NoNewline
Write-Host ""
Write-Host "  ━━━━━━━━━━━━━━━━━━━━" -ForegroundColor $Blue
Write-Host ""

Write-Host "  ⏳ Detecting latest version..." -ForegroundColor $Yellow

try {
    $Release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -ErrorAction Stop
    $Version = $Release.tag_name
} catch {
    Write-Host "  ❌ Failed to detect latest Hive version" -ForegroundColor $Red
    Write-Host "  Error: $($_.Exception.Message)" -ForegroundColor $Red
    exit 1
}

Write-Host "  ✅ Version: $Version" -ForegroundColor $Green
Write-Host ""

$Asset = "hive-windows.exe"
$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/$Asset"

Write-Host "  ⬇️ Downloading Hive..." -ForegroundColor $Yellow

$temp = Join-Path $env:TEMP $Binary

try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $temp -ErrorAction Stop
} catch {
    Write-Host "  ❌ Download failed" -ForegroundColor $Red
    Write-Host "  Error: $($_.Exception.Message)" -ForegroundColor $Red
    exit 1
}

if (!(Test-Path $temp) -or (Get-Item $temp).Length -eq 0) {
    Write-Host "  ❌ Download failed — file is empty" -ForegroundColor $Red
    exit 1
}

$InstallDir = "$env:USERPROFILE\.hive\bin"

if (!(Test-Path $InstallDir)) {
    Write-Host "  📁 Creating $InstallDir..." -ForegroundColor $Yellow
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$Target = Join-Path $InstallDir $Binary

Write-Host "  📦 Installing Hive..." -ForegroundColor $Yellow

Move-Item -Force $temp $Target

Write-Host ""
Write-Host "  ⚙️ Updating PATH..." -ForegroundColor $Yellow

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($currentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$InstallDir", "User")

    # Update current session PATH
    $env:Path += ";$InstallDir"
} else {
    Write-Host "  ℹ️ PATH already contains $InstallDir" -ForegroundColor $Blue
}

Write-Host ""
Write-Host "  ✅ Hive installed successfully!" -ForegroundColor $Green -NoNewline
Write-Host ""

# Test installation
Write-Host ""
if (Get-Command hive -ErrorAction SilentlyContinue) {
    Write-Host "  Version:" -ForegroundColor $Cyan
    hive --version 2>$null
} else {
    Write-Host "  ⚠️  Hive not found in current session PATH" -ForegroundColor $Yellow
    Write-Host ""
    Write-Host "  To use Hive in this terminal, run:" -ForegroundColor $Yellow
    Write-Host "    `$env:Path += `";$InstallDir`"" -ForegroundColor $Cyan
}

Write-Host ""
Write-Host "  🚀 Get started:" -ForegroundColor $Cyan -NoNewline
Write-Host ""
Write-Host "    hive --help" -ForegroundColor $Cyan
Write-Host ""