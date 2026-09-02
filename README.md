# 한/영 전환 도우미

Windows에서 `Shift + Space`와 `Ctrl + Space`를 전역 한/영 전환 키로 바꾸어 주는 작은 시스템 트레이 프로그램입니다. Rust와 Win32 API만 사용하며 WebView, Electron, .NET 런타임, 서비스, 네트워크, 분석 기능을 포함하지 않습니다.

- [프로그램 소개 및 설치 안내](https://wbmaker2.github.io/shift-space-lang-change/)

## 지원 환경

- Windows 10/11 x64
- 한국어 Microsoft IME가 설치된 사용자 계정
- 관리자 권한 없이 현재 사용자 범위에서 설치·실행

한국어 Microsoft IME가 없으면 앱은 실행되지만 한/영 전환 결과를 확인할 수 없습니다. 관리자 권한으로 실행 중인 프로그램은 Windows UIPI 정책 때문에 일반 권한 앱의 `SendInput`을 받지 못할 수 있습니다. 이 앱은 권한 상승을 자동 요청하지 않습니다.

## 배포 방식 선택

설치형은 일반 사용자에게 권장하며 시작 메뉴·제거 프로그램·자동 실행을 한 번에 관리합니다. 포터블형은 설치기 없이 원하는 폴더에서 실행할 때 선택하세요. 두 배포본은 동일한 Windows x64 실행 파일을 사용합니다.

- [v0.1.4 설치형 EXE 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Setup-0.1.4-x64.exe)
- [v0.1.4 포터블 ZIP 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Portable-0.1.4-x64.zip)
- [GitHub 최신 릴리스와 SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/latest)

### 설치형 사용

공개 예정인 v0.1.4 설치기는 다음 링크에서 받을 수 있습니다.

- [ShiftSpaceLangChange-Setup-0.1.4-x64.exe 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Setup-0.1.4-x64.exe)
- [GitHub 최신 릴리스 페이지](https://github.com/WBmaker2/shift-space-lang-change/releases/latest)
- [SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.4/SHA256SUMS.txt)

Windows package workflow가 만들고 GitHub Release에 공개한 `ShiftSpaceLangChange-Setup-<version>-x64.exe`를 실행하면 `%LOCALAPPDATA%\Programs\ShiftSpaceLangChange`에 사용자 단위로 설치됩니다. 설치 과정에서 관리자 권한을 요구하지 않으며 시작 메뉴에 `한영 전환 도우미` 바로가기를 만들고 로그인 시 자동 실행을 켭니다.

### 포터블형 사용

포터블 ZIP은 설치 폴더·시작 메뉴 바로가기·제거 프로그램을 만들지 않습니다. [포터블 ZIP](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Portable-0.1.4-x64.zip)을 원하는 위치에 내려받은 뒤 **먼저 압축을 풀고** `ShiftSpaceLangChange.exe`를 실행하세요. ZIP 내부에서 직접 실행하면 업데이트·경로 관리가 불편할 수 있습니다. 압축 파일에는 루트 폴더 하나와 `ShiftSpaceLangChange.exe`, `README-PORTABLE.txt`만 들어 있습니다.

포터블도 완전히 무흔적인 모드는 아닙니다. 단축키 설정은 `HKCU\Software\ShiftSpaceLangChange`에 저장되고, `Windows 시작 시 자동 실행`을 켜면 현재 EXE 경로가 HKCU Run 키에 등록됩니다. 포터블 폴더를 옮긴 뒤 자동 실행을 계속 사용하려면 새 위치에서 다시 켜세요.

설정 창에서 다음 항목을 각각 체크할 수 있습니다.

- `Shift + Space`
- `Ctrl + Space`

두 항목 중 하나 이상은 항상 켜져 있어야 합니다. 체크박스 변경은 저장 버튼 없이 즉시 적용됩니다. 충돌이나 저장 오류가 발생하면 이전의 유효한 설정으로 되돌리고 상태 문구와 트레이 알림으로 안내합니다.

창의 닫기 버튼은 프로그램을 끝내지 않고 트레이로 숨깁니다. 트레이 아이콘을 두 번 클릭하면 기존 설정 창을 복원할 수 있습니다. 트레이 아이콘을 우클릭하면 `설정 열기`, 활성 단축키 요약, `프로그램 정보`, `종료`를 사용할 수 있으며, `프로그램 정보`에는 프로그램명과 현재 버전이 표시됩니다. `Windows 시작 시 자동 실행` 체크를 해제하면 다음 로그인부터 자동 실행되지 않습니다.

## 제거와 SmartScreen

v0.1.4는 포터블 ZIP과 설치형을 함께 제공하지만 Windows 실기기 HVC는 아직 수행하지 않았습니다. 자동 테스트·정적 검사·패키지 검증 결과가 있더라도 실제 Windows 10/11에서의 설치·입력 전환·트레이·자동 실행·제거 동작을 통과로 주장하지 않습니다. 실기기 HVC에서 문제가 발견되면 후속 패치 릴리스에서 수정합니다.

설치형은 시작 메뉴의 제거 프로그램을 실행하면 먼저 실행 중인 앱에 종료를 요청하고 실행 파일 잠금이 풀릴 때까지 짧게 재시도한 뒤 다음 항목을 정리합니다. 수 초 안에 종료되지 않으면 설치·제거를 중단하여 부분 변경을 피합니다. 포터블형은 자동 실행을 끄고 트레이 메뉴에서 앱을 종료한 뒤 압축을 푼 폴더를 직접 삭제하세요. 폴더 삭제만으로 HKCU 설정이 제거되지는 않습니다.

- 설치 폴더와 실행 파일
- 시작 메뉴 바로가기
- 현재 사용자 `Run` 자동 실행 값
- `HKCU\Software\ShiftSpaceLangChange` 설정
- 현재 사용자 제거 정보

배포 파일에는 Authenticode 코드 서명이 포함되지 않습니다. 따라서 Windows SmartScreen이 최초 실행 시 경고할 수 있으며, 배포 출처와 파일을 확인한 경우에만 사용자가 직접 허용해야 합니다. SmartScreen 경고가 없다는 검증도 아직 완료되지 않았습니다.

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
makensis /DVERSION=0.1.4 installer\ShiftSpaceLangChange.nsi
pwsh -File scripts\build-portable.ps1 -Version 0.1.4
pwsh -File scripts\verify-package.ps1 -Version 0.1.4
```

GitHub Actions의 `Windows package` workflow는 `windows-2025`에서 테스트·릴리스 빌드·NSIS 패키징·포터블 ZIP 생성·SHA-256 manifest 작성을 수행하고 `shift-space-lang-change-windows-x64` artifact로 원본 실행 파일, 설치기, 포터블 ZIP, `SHA256SUMS.txt`를 제공합니다. 태그는 `v0.1.4`처럼 숫자 세 부분의 버전을 사용해야 합니다.

Windows 설치, 포터블 압축 해제, 실제 IME 전환, 트레이, 자동 실행, 제거 확인 결과는 [Windows HVC 기록지](docs/HVC-WINDOWS.md)에 기록합니다. v0.1.4 공개 범위와 자동 검증·미완료 항목은 [v0.1.4 릴리스 노트](.github/release-notes-v0.1.4.md)에서 확인할 수 있습니다. v0.1.3 패키지 검증 실패 기록은 [v0.1.3 릴리스 기록](.github/release-notes-v0.1.3.md)에서 확인할 수 있습니다.

## 개발 문서

- [설계 문서](docs/superpowers/specs/2026-08-31-shift-space-lang-change-design.md)
- [구현 계획](docs/superpowers/plans/2026-08-31-shift-space-lang-change.md)
- [완료·배포 계획](docs/superpowers/plans/2026-08-31-release-completion.md)
- [전역 단축키 오류 분류 수정 기록](docs/superpowers/reports/2026-09-01-hotkey-error-fix.md)
- [트레이 상호작용 수정 계획](docs/superpowers/plans/2026-09-01-tray-interaction-fix.md)
- [트레이 상호작용 수정 구현·검증 보고서](docs/superpowers/reports/2026-09-01-tray-interaction-fix.md)
- [트레이 콜백 라우팅 수정 계획](docs/superpowers/plans/2026-09-01-tray-callback-routing-fix.md)
- [트레이 콜백 라우팅 수정 구현·검증 보고서](docs/superpowers/reports/2026-09-01-tray-callback-routing-fix.md)
- [v0.1.2 릴리스 노트](.github/release-notes-v0.1.2.md)
- [v0.1.3 패키지 검증 실패 기록](.github/release-notes-v0.1.3.md)
- [v0.1.4 릴리스 노트](.github/release-notes-v0.1.4.md)
- [배포 진행 기록](docs/superpowers/reports/2026-09-01-release-progress.md)
- [GitHub Pages 배포 계획](docs/plans/2026-09-01-github-pages-deploy.md)

## 제한 사항

앱은 지정한 두 전역 단축키만 등록하며 키 입력 내용을 기록하거나 전송하지 않습니다. 관리자 권한 앱과의 입력 전달, 한국어 IME 설치 여부, Windows 로그인 후 동작, 포터블 폴더 이동 후 자동 실행 경로는 Windows 환경에서 별도 확인이 필요합니다. 지원하지 않는 Windows ARM64·32비트 빌드, 임의 단축키, 자동 업데이트, 음성 기능은 범위에 포함되지 않습니다.
