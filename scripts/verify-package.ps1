param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)

$ErrorActionPreference = 'Stop'

# The version is used in a file name, so reject path separators and shell-like
# input before interpolating it into a path.
if ($Version -notmatch '^[0-9]+\.[0-9]+\.[0-9]+(?:[-+][0-9A-Za-z.-]+)?$') {
    throw "Invalid package version: $Version"
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$binary = Join-Path $repoRoot 'target\release\shift-space-lang-change.exe'
$installer = Join-Path $repoRoot "dist\ShiftSpaceLangChange-Setup-$Version-x64.exe"

if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    throw "Missing release binary: $binary"
}

if (-not (Test-Path -LiteralPath $installer -PathType Leaf)) {
    throw "Missing installer: $installer"
}

if ((Get-Item -LiteralPath $binary).Length -le 0) {
    throw 'Release binary is empty'
}

if ((Get-Item -LiteralPath $installer).Length -le 0) {
    throw 'Installer is empty'
}

Write-Host "Verified $binary and $installer (version $Version)"
