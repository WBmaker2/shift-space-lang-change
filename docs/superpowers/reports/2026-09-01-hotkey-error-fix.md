# Windows 전역 단축키 오류 분류 수정 기록

- 기준 커밋: `a93883a`
- 구현 커밋 SHA: `591653839dd8fb95d440b6487156f8acf167e0de`
- 구현 커밋 메시지: `fix: classify Windows hotkey errors`

## 변경 내용

- `RegisterHotKey`와 `UnregisterHotKey` 실패 직후 `GetLastError()`의 raw Win32 코드를 `Win32Error`에 보존하도록 수정했습니다. Windows crate `Error`의 HRESULT 값은 더 이상 오류 코드로 사용하지 않습니다. 각 `unsafe` 호출에는 GetLastError의 유효 시점과 thread-local 전제를 주석으로 남겼습니다.
- 플랫폼 중립 `HotkeyErrorClass`와 `classify_hotkey_error` 순수 함수를 추가했습니다. `ERROR_HOTKEY_ALREADY_REGISTERED`(1409)만 `AlreadyRegistered`로 분류하고, access denied(5), invalid handle(6), 임의 오류는 `Fatal`로 분류합니다.
- 초기 등록과 fallback 등록 모두 같은 `classify_hotkey_apply_error` 경로를 사용합니다. 충돌만 fallback 또는 1회 양쪽 충돌 대화상자로 이어지고, Register/Unregister/Rollback의 다른 오류는 원래 `Win32Error`를 Fatal로 전달합니다.
- partial registration rollback, `HotkeyManager::Drop`, 기존 한국어 BothConflict 대화상자와 cleanup 경로는 유지했습니다.

## TDD 기록

1. RED: 분류 테스트를 먼저 추가한 뒤 기존 구현에서 `HotkeyErrorClass`와 `classify_hotkey_error`가 없어 `cargo test --lib hotkeys::tests::only_already_registered_is_a_conflict`가 unresolved import 컴파일 오류로 실패했습니다.
2. GREEN: 분류 enum/함수를 구현하고 동일 focused test가 통과했습니다. 1409, 5, 6, `0xdead_beef`를 각각 검증하며 실제 `RegisterHotKey`를 호출하지 않았습니다.

## 검증 결과

Rust 1.97.1 toolchain의 절대 경로와 toolchain `bin`을 PATH 앞에 두고 실행했습니다.

- `cargo fmt -- --check`: 통과
- `cargo test --all-targets`: 통과 (전체 테스트)
- `cargo clippy --all-targets -- -D warnings`: 통과
- `cargo check --target x86_64-pc-windows-msvc`: 통과
- `cargo clippy --target x86_64-pc-windows-msvc --all-targets -- -D warnings`: 통과
- `git diff --check`: 통과
- 변경 Rust 파일 줄 수: `src/hotkeys.rs` 171줄, `src/platform/windows/hotkeys.rs` 68줄, `src/platform/windows/app.rs` 425줄

## 미검증 범위

현재 호스트는 macOS이므로 Windows runtime, 실제 RegisterHotKey 충돌, Windows 설치, HVC, 트레이/UI 및 메모리 측정은 실행하지 않았습니다. 이 기록은 Windows target compile/clippy 성공을 의미하며 Windows 실기기 실행 성공을 주장하지 않습니다.
