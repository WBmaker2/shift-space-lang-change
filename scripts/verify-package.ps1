param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)

$ErrorActionPreference = 'Stop'

if ($Version -notmatch '^[0-9]+\.[0-9]+\.[0-9]+(?:[-+][0-9A-Za-z.-]+)?$') {
    throw "Invalid package version: $Version"
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$binary = Join-Path $repoRoot 'target\release\shift-space-lang-change.exe'
$installer = Join-Path $repoRoot "dist\ShiftSpaceLangChange-Setup-$Version-x64.exe"
$portable = Join-Path $repoRoot "dist\ShiftSpaceLangChange-Portable-$Version-x64.zip"
$checksums = Join-Path $repoRoot 'dist\SHA256SUMS.txt'
$packageName = "ShiftSpaceLangChange-Portable-$Version-x64"

foreach ($path in @($binary, $installer, $portable)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing package output: $path"
    }
    if ((Get-Item -LiteralPath $path).Length -le 0) {
        throw "Package output is empty: $path"
    }
}

function Test-EntryMatchesFile {
    param(
        [Parameter(Mandatory = $true)]
        [object]$Entry,
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    $entryStream = $Entry.Open()
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try {
        # ComputeHash consumes the complete stream and is safe when a
        # DeflateStream returns partial reads.
        $entryHash = [BitConverter]::ToString($sha.ComputeHash($entryStream)).Replace('-', '').ToLowerInvariant()
        $fileHash = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
        return $entryHash -eq $fileHash
    }
    finally {
        $entryStream.Dispose()
        $sha.Dispose()
    }
}

Add-Type -AssemblyName System.IO.Compression.FileSystem
$archive = [System.IO.Compression.ZipFile]::OpenRead($portable)
try {
    $entries = @($archive.Entries)
    $fileEntries = @($entries | Where-Object { -not $_.FullName.EndsWith('/') })
    if ($fileEntries.Count -ne 2) {
        throw "Portable ZIP must contain exactly two files; found $($fileEntries.Count)"
    }

    $expectedFiles = @(
        "$packageName/ShiftSpaceLangChange.exe",
        "$packageName/README-PORTABLE.txt"
    )
    foreach ($entry in $entries) {
        $entryName = $entry.FullName.Replace('\', '/')
        $trimmed = $entryName.TrimEnd('/')
        if ($trimmed -eq $packageName) {
            continue
        }
        if (-not $entryName.StartsWith("$packageName/", [System.StringComparison]::Ordinal)) {
            throw "Portable ZIP contains an unexpected root path: $($entry.FullName)"
        }
        if (-not $entry.FullName.EndsWith('/') -and $expectedFiles -notcontains $entryName) {
            throw "Portable ZIP contains an unexpected file: $($entry.FullName)"
        }
        if ($entry.FullName.EndsWith('/') -and $trimmed -ne $packageName) {
            throw "Portable ZIP contains an unexpected directory: $($entry.FullName)"
        }
    }

    foreach ($expected in $expectedFiles) {
        if (-not ($fileEntries.FullName.Replace('\', '/') -contains $expected)) {
            throw "Portable ZIP is missing: $expected"
        }
    }

    $readmeEntry = $entries | Where-Object {
        $_.FullName.Replace('\', '/') -eq "$packageName/README-PORTABLE.txt"
    } | Select-Object -First 1
    if (-not $readmeEntry -or $readmeEntry.Length -le 0) {
        throw 'Portable ZIP README-PORTABLE.txt is missing or empty'
    }

    $binaryEntry = $entries | Where-Object {
        $_.FullName.Replace('\', '/') -eq "$packageName/ShiftSpaceLangChange.exe"
    } | Select-Object -First 1
    if (-not $binaryEntry) {
        throw 'Portable ZIP executable entry was not found'
    }
    if ($binaryEntry.Length -ne (Get-Item -LiteralPath $binary).Length) {
        throw 'Portable ZIP executable length differs from the release binary'
    }

    if (-not (Test-EntryMatchesFile -Entry $binaryEntry -Path $binary)) {
        throw 'Portable ZIP executable differs from the release binary (byte comparison failed)'
    }
}
finally {
    $archive.Dispose()
}

$hashLines = foreach ($path in @($binary, $installer, $portable)) {
    $hash = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
    Write-Host "SHA-256 $([System.IO.Path]::GetFileName($path)): $hash"
    "$hash  $([System.IO.Path]::GetFileName($path))"
}

Set-Content -LiteralPath $checksums -Value $hashLines -Encoding ascii
if (-not (Test-Path -LiteralPath $checksums -PathType Leaf) -or (Get-Item -LiteralPath $checksums).Length -le 0) {
    throw "Checksum manifest was not created: $checksums"
}

Write-Host "Verified installer, portable ZIP structure, identical executable, and checksums (version $Version)"
