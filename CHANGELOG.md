# 변경 기록

## v0.1.3 — 2026-09-02

### Added

- 설치기 없이 압축을 풀어 실행할 수 있는 `ShiftSpaceLangChange-Portable-0.1.3-x64.zip` 배포 채널을 추가했습니다.
- 포터블 ZIP 내부를 `ShiftSpaceLangChange.exe`와 `README-PORTABLE.txt`로 고정하고, 원본 release EXE와의 SHA-256 동일성을 Windows workflow에서 검증합니다.
- 홍보 페이지에서 설치형(추천)과 포터블형(무설치)을 비교해 선택할 수 있는 다운로드 카드를 추가했습니다.
- Windows package artifact에 설치기, 포터블 ZIP, 원본 EXE와 `SHA256SUMS.txt`를 함께 제공합니다.

### Changed

- 포터블은 설치 폴더·시작 메뉴·제거 프로그램을 만들지 않지만, 설정은 `HKCU`에 저장되고 선택적 자동 실행은 `HKCU Run` 키를 사용한다는 범위를 안내합니다.
- 포터블 폴더를 먼저 압축 해제하고 실행해야 하며, 이동 후 자동 실행을 다시 설정하고 삭제 전 앱을 종료해야 한다는 절차를 문서화했습니다.
- 다운로드 강조 애니메이션에 `prefers-reduced-motion` 대응을 추가했습니다.

### Verification status

- macOS에서는 PowerShell `Compress-Archive`와 Windows 실기기 동작을 완전히 검증할 수 없습니다. Windows runner가 패키지 생성·ZIP 구조·바이너리 동일성·SHA-256을 검증합니다.
- Windows 10/11 x64 실기기 HVC는 아직 미검증입니다. 포터블 실행, 한국어 IME 전환, 트레이, 자동 실행 경로, 폴더 삭제는 [Windows HVC 기록지](docs/HVC-WINDOWS.md)에 별도로 기록합니다.

## v0.1.2 — 2026-09-01

### Fixed

- 트레이 아이콘에서 발생한 Shell 콜백이 앱 이벤트 루프에 도달하지 않던 경로를 수정해 더블 클릭 복원과 우클릭 메뉴가 동작하도록 했습니다.

### Added

- 트레이 아이콘 더블 클릭으로 기존 설정 창을 복원하고 전경으로 가져오는 기능을 추가했습니다.
- 트레이 우클릭 메뉴에 `설정 열기`, 활성 단축키 요약, `프로그램 정보`, `종료`를 제공하고, `프로그램 정보`에서 프로그램명과 현재 버전을 표시합니다.

### Verification status

- Windows 10/11 실기기 HVC는 아직 미검증입니다. 자동 테스트·패키지 검증을 실기기 통과로 간주하지 않습니다.
- 서명되지 않은 설치기의 SmartScreen 경고 가능성과 관리자 권한 앱에 대한 UIPI 입력 제한은 계속 적용됩니다.

## v0.1.1 — 2026-09-01

### Fixed

- 설정 창의 `트레이로 숨기기` 버튼이 부모 창의 동기 `WM_COMMAND`에서 유실되던 결함을 수정해 창 숨김 이벤트가 앱 메시지 처리 경로에 도달하도록 했습니다.

### Changed

- 트레이 아이콘 더블 클릭으로 기존 설정 창을 복원하고 전경으로 가져오는 경로와 우클릭 메뉴(`설정 열기`, 활성 단축키 요약, `종료`)의 검증 범위를 보강했습니다.
- 버튼 알림·체크박스 클릭 시점 상태 보존·트레이 명령 변환에 대한 자동 테스트를 추가했습니다. Windows 10/11 실기기 HVC는 아직 완료하지 않았습니다.
- 서명되지 않은 설치기는 SmartScreen 경고가 표시될 수 있고, UIPI 정책에 따라 관리자 권한 앱에는 `SendInput`이 전달되지 않을 수 있다는 제한을 릴리스 문서에 명시했습니다.

## v0.1.0 Pages — 2026-09-01

### Added

- 프로그램 설명, 설치·사용 방법, 지원 범위, 업데이트 내역을 제공하는 정적 홍보 페이지를 추가했습니다.
- 공식 v0.1.0 설치기 직접 다운로드와 GitHub 최신 릴리스 링크를 추가했습니다.
- GitHub Actions 기반 Pages 배포 및 정적 파일 검증 절차를 추가했습니다.

## v0.1.0 — 2026-09-01

### Added

- Rust와 Win32 API만 사용하는 Windows 시스템 트레이 앱을 추가했습니다. `Shift + Space`와 `Ctrl + Space`를 전역 한/영 전환 키로 등록하고, 한국어 Microsoft IME에 `VK_HANGUL` 입력을 전달합니다.
- 설정 창, 트레이 메뉴, 창 숨기기, 중복 실행 방지, 로그인 시 사용자 단위 자동 실행을 추가했습니다.
- HKCU 설정 저장, 최소 하나의 단축키를 유지하는 설정 규칙, 즉시 적용과 실패 시 복구를 추가했습니다.
- 관리자 권한 없이 설치·실행하는 NSIS 사용자 설치기와 안전한 종료 대기·제거 정리를 추가했습니다.
- Ubuntu/Windows GitHub Actions CI와 Windows x64 package workflow를 추가했습니다.

### Changed

- 보조키와 Space가 놓인 뒤 한 번만 입력을 전달하고, 5초 안에 해제되지 않으면 요청을 취소하도록 전환 흐름을 고정했습니다.
- 실제 등록된 단축키 집합을 기준으로 추가·삭제·롤백·종료 정리를 수행하도록 바꾸고, 부분 등록 및 재시도 상태를 보존합니다.
- Win32 오류의 원본 코드와 provenance를 보존하며 `ERROR_HOTKEY_ALREADY_REGISTERED`만 충돌·fallback으로 처리하고 그 외 오류는 치명적 오류로 안내합니다.
- Windows package workflow가 신뢰할 수 있는 경로에서 `makensis.exe`를 찾고, NSIS shell context 범위와 최종 실행 파일·설치기 출력을 검증하도록 했습니다.

### Fixed

- 초기 등록과 fallback 등록에서 access denied, invalid handle 및 임의 오류가 충돌로 잘못 분류되던 문제를 수정했습니다.
- 등록 실패 후 rollback이 첫 오류에서 중단되거나 실제 등록되지 않은 상태를 기준으로 정리하던 문제를 수정했습니다.
- Windows package workflow에서 Chocolatey 설치 직후 `makensis`를 찾지 못하던 문제와 NSIS 전역 shell context 오류를 수정했습니다.
