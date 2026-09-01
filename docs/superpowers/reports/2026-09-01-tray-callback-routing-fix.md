# 트레이 콜백 라우팅 수정 구현 보고서

승인된 `2026-09-01-tray-callback-routing-fix` 계획에 따라 Shell 트레이 콜백과 앱 메시지 큐의 경로를 분리하고 프로그램 정보 메뉴를 추가했습니다. 구현 단계에서는 Cargo 버전을 변경하지 않았고, 이어진 v0.1.2 공개 준비 단계에서 버전 메타데이터와 문서를 갱신했습니다. 커밋·푸시·릴리스는 수행하지 않았습니다.

## 구현

- `window.rs`: Shell 등록용 `WM_APP_TRAY_CALLBACK`과 앱 큐용 `WM_APP_TRAY_QUEUE`를 분리했습니다. `window_proc`는 콜백의 정수형 `wParam`·`lParam`을 `PostMessageW`로 큐 메시지에 재게시하고, 앱 루프는 큐 메시지만 `TrayIcon::read_event`로 전달합니다. 기존 종료(`WM_APP + 2`) 및 UI 명령(`WM_APP + 3`)과 충돌하지 않습니다.
- `ui_model.rs`, `tray.rs`: `UiEvent::About`/`IDM_ABOUT`을 추가하고 우클릭 메뉴에 `프로그램 정보`를 배치했습니다. 메뉴 종료 뒤 `NIM_SETFOCUS`와 `WM_NULL` 처리를 수행합니다.
- `app.rs`: 정보 메뉴 선택 시 `한/영 전환 도우미`와 `env!("CARGO_PKG_VERSION")`를 `MessageBoxW` 정보 대화상자로 표시합니다.
- 관련 테스트: About 명령 매핑, 메시지 ID 분리·예약, Shell 콜백 비소비, 내부 큐의 더블 클릭 이벤트 변환을 고정했습니다.

## 검증

- `cargo fmt --all -- --check` — 통과
- `cargo test --all-targets` — 호스트에서 40개 통과
- `cargo clippy --all-targets -- -D warnings` — 통과
- `cargo check --target x86_64-pc-windows-msvc --all-targets` — 통과
- `cargo clippy --target x86_64-pc-windows-msvc --all-targets -- -D warnings` — 통과
- 변경 Rust 파일 줄 수: `window.rs` 434줄, `app.rs` 461줄, `tray.rs` 259줄, `ui_model.rs` 77줄

검증 명령은 저장소의 Rust 1.97.1 툴체인을 사용했습니다. Windows 대상 테스트 실행과 Windows 10/11 실기기 HVC는 이 환경에서 수행할 수 없어 남아 있습니다.

## 남은 실기기 HVC

Windows 10/11에서 숨김 후 트레이 더블 클릭 복원, 우클릭 메뉴의 설정 열기·프로그램 정보·종료, 정보 대화상자의 이름/버전, 메뉴 포커스 복귀 및 기존 단축키·자동 실행 회귀를 직접 확인해야 합니다. VoiceOver, 음성 출력, 녹음 및 재생 검증은 범위에서 제외합니다.

## v0.1.2 릴리스 준비 상태

- Cargo 패키지와 잠금 파일, NSIS 기본 버전, Windows package workflow fallback을 `0.1.2`로 갱신했습니다.
- README, GitHub Pages 본문·설치기 링크, CHANGELOG, v0.1.2 릴리스 노트, HVC 기록지의 최신 대상과 릴리스 준비 상태를 갱신했습니다.
- v0.1.2 공개 릴리스, 설치기·앱 자산, 파일 크기·SHA-256, 커밋·CI 실행 결과는 아직 생성·확정되지 않았으며 공개 후 HVC 기록지에 채울 자리만 마련했습니다.
- 커밋·푸시·태그·GitHub Release는 수행하지 않았습니다. Windows 10/11 실기기 HVC도 아직 미검증입니다.

### 공개 준비 순서와 Pages 404 방지

공개 순서는 `PR CI 통과 → feature head에 v0.1.2 tag → Windows package 성공 및 Release 자산 공개 → PR main 병합 → Pages 배포`로 기록합니다. feature branch가 `main`에 병합되기 전에 Pages를 먼저 배포하면 최신 문서·설치기 링크가 반영되지 않아 404가 발생할 수 있으므로, Windows package 성공과 Release 자산 공개를 먼저 확인한 뒤 `main` 병합과 Pages 배포를 진행합니다. 현재는 순서를 기록한 준비본이며 공개 완료가 아닙니다.
