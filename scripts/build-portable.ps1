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
$dist = Join-Path $repoRoot 'dist'
$staging = Join-Path $dist 'portable-staging'
$packageName = "ShiftSpaceLangChange-Portable-$Version-x64"
$packageRoot = Join-Path $staging $packageName
$zipPath = Join-Path $dist "$packageName.zip"

if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    throw "Missing release binary: $binary"
}
if ((Get-Item -LiteralPath $binary).Length -le 0) {
    throw 'Release binary is empty'
}

New-Item -ItemType Directory -Force -Path $dist | Out-Null

# This is a dedicated generated directory. It is safe to recreate, and no
# source or user-selected path is ever removed by this script.
if (Test-Path -LiteralPath $staging) {
    Remove-Item -LiteralPath $staging -Recurse -Force
}
if (Test-Path -LiteralPath $zipPath) {
    Remove-Item -LiteralPath $zipPath -Force
}
New-Item -ItemType Directory -Force -Path $packageRoot | Out-Null

Copy-Item -LiteralPath $binary -Destination (Join-Path $packageRoot 'ShiftSpaceLangChange.exe')

$portableReadme = @"
한/영 전환 도우미 포터블 버전

1. 이 ZIP 안에서 직접 실행하지 말고, 먼저 원하는 위치에 압축을 풀어 주세요.
2. 압축을 푼 폴더의 ShiftSpaceLangChange.exe를 실행해 주세요.

지원 환경
- Windows 10/11 x64
- 한국어 Microsoft IME가 설치된 사용자 계정
- 설치·관리자 권한·별도 런타임이 필요하지 않습니다.

사용 및 종료
- 실행 후 작업 표시줄 알림 영역(트레이)에서 단축키를 설정합니다.
- 창의 X는 숨기기입니다. 완전히 종료하려면 트레이 메뉴의 '종료'를 사용하세요.

설정과 자동 실행
- 설정은 현재 사용자 레지스트리 HKCU\Software\ShiftSpaceLangChange에 저장됩니다.
- 'Windows 시작 시 자동 실행'을 켜면 HKCU\Software\Microsoft\Windows\CurrentVersion\Run에
  현재 실행 파일 경로가 등록됩니다.
- 자동 실행을 켠 뒤 포터블 폴더를 옮기면 경로가 달라질 수 있으므로, 새 위치에서 다시 설정하세요.

삭제
- 자동 실행을 끄고 트레이 메뉴에서 앱을 종료한 뒤 압축을 푼 폴더를 삭제하세요.
- 앱 폴더를 삭제해도 이미 저장된 사용자 설정은 남을 수 있습니다. 설정 창에서 자동 실행을 끄고,
  필요하면 HKCU\Software\ShiftSpaceLangChange 설정을 직접 정리하세요.

안전 안내
- 코드 서명이 없어 Windows SmartScreen 경고가 표시될 수 있습니다.
- 출처가 WBmaker2/shift-space-lang-change GitHub Release인지 확인하고 SHA-256을 대조한 뒤 사용하세요.
"@

Set-Content -LiteralPath (Join-Path $packageRoot 'README-PORTABLE.txt') -Value $portableReadme -Encoding utf8
Compress-Archive -Path $packageRoot -DestinationPath $zipPath -CompressionLevel Optimal

if (-not (Test-Path -LiteralPath $zipPath -PathType Leaf) -or (Get-Item -LiteralPath $zipPath).Length -le 0) {
    throw "Portable ZIP was not created: $zipPath"
}

Write-Host "Created portable package: $zipPath"
