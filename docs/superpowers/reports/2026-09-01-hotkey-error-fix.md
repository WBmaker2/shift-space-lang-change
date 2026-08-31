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

## 독립 리뷰 수정 라운드 1

- 리뷰 findings: (1) Windows-rs BOOL wrapper 이후 GetLastError를 재조회하면 thread-local 값이 이미 wrapper 내부에서 소비된 뒤일 수 있음, (2) 논리적 active 설정만으로 Drop/rollback 대상을 정하면 부분 등록 누수가 발생하고 rollback이 첫 오류에서 중단됨, (3) 시작 정책 테스트가 등록 단계와 오류 provenance를 충분히 고정하지 못함.
- `WIN32_ERROR::from_error(&error)`로 wrapper가 반환한 `HRESULT_FROM_WIN32`를 복원하고, 비-Win32 HRESULT는 원본 code와 provenance를 보존하는 `Win32Error::from_hresult` 경로로 Fatal 처리했습니다. 따라서 숫자가 우연히 1409인 non-Win32 HRESULT도 충돌로 분류되지 않습니다.
- `HotkeyManager`에 실제 성공 등록 집합을 추가했습니다. register 성공·unregister 성공을 즉시 반영하고, rollback은 첫 오류를 저장한 채 가능한 복원/정리를 계속하며, 실패한 unregister 항목은 집합에 남겨 Drop이 재시도합니다. 초기 `new` 실패도 역순 전체 정리를 시도합니다.
- 시작 분류는 플랫폼 중립 `HotkeyErrorSource`/`classify_startup_error`가 공통으로 담당합니다. 1409 초기 등록은 fallback, 1409 fallback 등록은 BothConflict, Register 5/6/임의 오류와 Unregister/Rollback은 Fatal임을 테스트했습니다.

### TDD 및 회귀 테스트

- RED: 독립 리뷰의 새 회귀 기대(실제 등록 집합 API, rollback 후 Drop 재시도, rollback 첫 오류 이후 잔여 작업 수행)를 기존 구현에 적용하면 등록 집합이 없어 컴파일되지 않거나 첫 rollback 오류에서 반환해 기대 call trace를 충족하지 못하는 상태였습니다.
- GREEN: 구현 후 `rollback_continues_after_first_restore_error_and_cleans_added_hotkey`, `drop_retries_added_registration_left_by_rollback_failure` 및 시작 정책 focused 테스트가 통과했습니다. 실제 Win32 등록 API는 호출하지 않았습니다.

### 라운드 1 검증

- `cargo fmt -- --check`: 통과
- `cargo test --all-targets`: 통과 (전체 35개 테스트)
- `cargo clippy --all-targets -- -D warnings`: 통과
- `cargo check --target x86_64-pc-windows-msvc`: 통과
- `cargo clippy --target x86_64-pc-windows-msvc --all-targets -- -D warnings`: 통과
- `git diff --check`: 통과
- 줄 수: `src/hotkeys.rs` 375줄, `src/controller.rs` 209줄, `src/platform/windows/hotkeys.rs` 71줄, `src/platform/windows/error.rs` 50줄, `src/platform/windows/app.rs` 442줄, `tests/hotkey_manager.rs` 382줄
- 라운드 1 커밋 SHA: 커밋 후 기록

이번 라운드도 macOS에서 수행했으므로 Windows runtime/HVC, 설치·트레이·실제 단축키 충돌은 미검증입니다.
