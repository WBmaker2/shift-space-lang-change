# 한/영 전환 도우미

Windows에서 `Shift + Space`와 `Ctrl + Space`를 전역 한/영 전환 키로 바꾸어 주는 작은 시스템 트레이 프로그램입니다. Rust와 Win32 API만 사용하며 WebView, Electron, .NET 런타임, 서비스, 네트워크, 분석 기능을 포함하지 않습니다.

## 지원 환경

- Windows 10/11 x64
- 한국어 Microsoft IME가 설치된 사용자 계정
- 관리자 권한 없이 현재 사용자 범위에서 설치·실행

한국어 Microsoft IME가 없으면 앱은 실행되지만 한/영 전환 결과를 확인할 수 없습니다. 관리자 권한으로 실행 중인 프로그램은 Windows UIPI 정책 때문에 일반 권한 앱의 `SendInput`을 받지 못할 수 있습니다. 이 앱은 권한 상승을 자동 요청하지 않습니다.

## 설치와 사용

Windows package workflow가 만든 `ShiftSpaceLangChange-Setup-<version>-x64.exe`를 실행하면 `%LOCALAPPDATA%\Programs\ShiftSpaceLangChange`에 사용자 단위로 설치됩니다. 설치 과정에서 관리자 권한을 요구하지 않으며 시작 메뉴에 `한영 전환 도우미` 바로가기를 만들고 로그인 시 자동 실행을 켭니다.

설정 창에서 다음 항목을 각각 체크할 수 있습니다.

- `Shift + Space`
- `Ctrl + Space`

두 항목 중 하나 이상은 항상 켜져 있어야 합니다. 체크박스 변경은 저장 버튼 없이 즉시 적용됩니다. 충돌이나 저장 오류가 발생하면 이전의 유효한 설정으로 되돌리고 상태 문구와 트레이 알림으로 안내합니다.

창의 닫기 버튼은 프로그램을 끝내지 않고 트레이로 숨깁니다. 트레이 아이콘을 두 번 클릭하면 설정을 다시 열 수 있고, 트레이 메뉴의 `종료`를 선택해야 완전히 종료됩니다. `Windows 시작 시 자동 실행` 체크를 해제하면 다음 로그인부터 자동 실행되지 않습니다.

## 제거와 SmartScreen

시작 메뉴의 제거 프로그램을 실행하면 먼저 실행 중인 앱에 종료를 요청한 뒤 다음 항목을 정리합니다.

- 설치 폴더와 실행 파일
- 시작 메뉴 바로가기
- 현재 사용자 `Run` 자동 실행 값
- `HKCU\Software\ShiftSpaceLangChange` 설정
- 현재 사용자 제거 정보

배포 파일에는 유료 Authenticode 서명이 포함되지 않습니다. 따라서 Windows SmartScreen이 최초 실행 시 경고할 수 있으며, 배포 출처를 확인한 경우에만 사용자가 직접 허용해야 합니다.

## 개발 및 검증

로컬에서 플랫폼 독립 로직을 확인합니다.

```bash
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo check --target x86_64-pc-windows-msvc --all-targets
```

Windows에서 패키지를 직접 만들려면 Rust 1.97.1과 NSIS를 준비한 뒤 다음을 실행합니다.

```powershell
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release
New-Item -ItemType Directory -Force dist
makensis /DVERSION=0.1.0 installer\ShiftSpaceLangChange.nsi
pwsh -File scripts\verify-package.ps1 -Version 0.1.0
```

GitHub Actions의 `Windows package` workflow는 `windows-2025`에서 테스트·릴리스 빌드·NSIS 패키징을 수행하고 `shift-space-lang-change-windows-x64` artifact로 실행 파일과 설치 파일을 제공합니다. 태그는 `v0.1.0`처럼 숫자 세 부분의 버전을 사용해야 합니다.

Windows 설치, 실제 IME 전환, 트레이, 자동 실행, 제거 확인 결과는 [Windows HVC 기록지](docs/HVC-WINDOWS.md)에 기록합니다. 현재 저장소에 원격 GitHub 주소가 연결되지 않았다면 다운로드 링크가 아직 생성되지 않습니다.

## 제한 사항

앱은 지정한 두 전역 단축키만 등록하며 키 입력 내용을 기록하거나 전송하지 않습니다. 관리자 권한 앱과의 입력 전달, 한국어 IME 설치 여부, Windows 로그인 후 동작은 Windows 환경에서 별도 확인이 필요합니다. 지원하지 않는 Windows ARM64·32비트 빌드, 임의 단축키, 자동 업데이트, 음성 기능은 범위에 포함되지 않습니다.
