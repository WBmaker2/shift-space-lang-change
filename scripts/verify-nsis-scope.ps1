param(
    [string]$ScriptPath = (Join-Path $PSScriptRoot '..\installer\ShiftSpaceLangChange.nsi')
)

$ErrorActionPreference = 'Stop'

if (-not (Test-Path -LiteralPath $ScriptPath -PathType Leaf)) {
    throw "Missing NSIS script: $ScriptPath"
}

$scope = $null
$contextSections = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::OrdinalIgnoreCase)

foreach ($line in Get-Content -LiteralPath $ScriptPath) {
    $code = ($line -replace ';.*$', '').Trim()
    if (-not $code) {
        continue
    }

    if ($code -match '^Section\s+"([^"]+)"') {
        $scope = $Matches[1]
        continue
    }
    if ($code -match '^Function\s+\S+') {
        $scope = 'Function'
        continue
    }
    if ($code -match '^(SectionEnd|FunctionEnd)\b') {
        $scope = $null
        continue
    }

    if ($code -match '^SetShellVarContext\b') {
        if (-not $scope) {
            throw "SetShellVarContext must be inside a Section or Function: $line"
        }
        if ($code -notmatch '^SetShellVarContext\s+current\s*$') {
            throw "Unexpected shell context: $line"
        }
        if ($scope -eq 'Install' -or $scope -eq 'Uninstall') {
            $contextSections.Add($scope) | Out-Null
        }
    }
}

foreach ($required in @('Install', 'Uninstall')) {
    if (-not $contextSections.Contains($required)) {
        throw "Missing SetShellVarContext current in $required section"
    }
}

Write-Host 'Verified SetShellVarContext scope for Install and Uninstall sections.'
