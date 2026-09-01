# 트레이 상호작용 수정 구현·검증 보고서

## 범위

승인된 `2026-09-01-tray-interaction-fix` 계획에 따라 설정 창 버튼 이벤트 전달과 트레이 상호작용을 보강했습니다. Git commit, push, release 및 Windows 실기기 HVC는 수행하지 않았습니다.

## 원인

버튼 클릭의 `WM_COMMAND`는 부모 창 프로시저로 동기 전달됩니다. 기존 메시지 루프는 큐에서 꺼낸 메시지만 `read_ui_event`로 해석했기 때문에 `트레이로 숨기기`와 체크박스 명령이 이벤트 처리기에 도달하지 않았습니다.

## 구현

- `window.rs`: `WM_APP_UI_COMMAND`(`WM_APP + 3`)를 추가했습니다. 부모 프로시저가 인식된 버튼의 `WM_COMMAND`를 큐 메시지로 재게시합니다. 앱 종료 요청(`WM_APP + 2`)과의 충돌을 피했습니다.
- `ui_model.rs`: 네이티브 `WM_COMMAND` 알림과 큐 payload를 각각 타입화된 `UiEvent`로 변환하도록 했습니다. 체크박스 상태는 동기 수신 시 bit 16에 보존해 빠른 연속 클릭에서도 상태가 섞이지 않도록 했습니다.
- `tray.rs`: 기존 더블 클릭(설정 창 표시·전경 이동), 우클릭 메뉴(설정 열기·활성 단축키 요약·종료) 경로를 유지하고 `TrackPopupMenu` 이후 권장 `WM_NULL` 보조 메시지를 추가했습니다.
- `tests/ui_model.rs`, `tray.rs` 테스트: 버튼 알림, 비-클릭 알림, 큐 상태 보존, 활성 단축키 요약을 검증했습니다.
- `docs/HVC-WINDOWS.md`: v0.1.0 사용자 보고 결함과 패치 후보의 실기기 재검증 항목을 추가했습니다.

## 검증 결과

1. `cargo fmt --check` — 초기 포맷 차이를 `cargo fmt`로 정리한 뒤 통과.
2. `cargo test --all-targets` — 39개 통과.
3. `cargo clippy --all-targets -- -D warnings` — 통과.
4. `cargo check --target x86_64-pc-windows-msvc --all-targets` — 통과.
5. `cargo clippy --target x86_64-pc-windows-msvc --all-targets -- -D warnings` — 통과.
6. `git diff --check` — 통과.
7. 변경 Rust 파일 줄 수 — 모두 500줄 미만.

기본 PATH의 Homebrew Rust 1.95로 처음 테스트했을 때 `rust-version = 1.97` 오류가 발생했습니다. Rustup 1.97.1 toolchain을 PATH 앞에 두고 같은 테스트·검증을 다시 실행해 위 결과를 얻었습니다.

## 남은 Windows 실기기 HVC

Windows 10/11 x64에서 패치 빌드 또는 승인된 패치 릴리스 자산으로 다음을 직접 확인해야 합니다.

- `트레이로 숨기기`가 설정 창만 숨기고 프로세스와 트레이 아이콘을 유지하는지
- 트레이 더블 클릭과 `설정 열기`가 기존 창을 복원하고 전경으로 이동하는지
- 우클릭 메뉴의 활성 단축키 요약과 `종료`가 표시되고 정상 종료하는지
- 체크박스 즉시 반영, 마지막 활성 단축키 보호, 창 닫기·ESC 숨김, 단일 인스턴스 및 기존 단축키가 회귀하지 않는지

VoiceOver, 음성 출력, 녹음 및 재생 검증은 범위에서 제외합니다.
