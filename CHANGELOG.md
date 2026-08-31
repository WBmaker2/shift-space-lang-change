# 변경 기록

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
