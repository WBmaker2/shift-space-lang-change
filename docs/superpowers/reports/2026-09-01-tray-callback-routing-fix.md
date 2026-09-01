# 트레이 콜백 라우팅 수정 구현 보고서

승인된 `2026-09-01-tray-callback-routing-fix` 계획에 따라 Shell 트레이 콜백과 앱 메시지 큐의 경로를 분리하고 프로그램 정보 메뉴를 추가했습니다. 구현 단계와 공개 준비 단계의 보고 시점에는 Cargo 버전을 변경하지 않았고 커밋·푸시·릴리스를 수행하지 않았습니다. 이후 승인된 공개 순서에 따라 v0.1.2가 공개되었으며, 최종 공개 증거를 아래에 추가했습니다.

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

## v0.1.2 릴리스 준비 상태 (공개 전 기록)

- 당시 Cargo 패키지와 잠금 파일, NSIS 기본 버전, Windows package workflow fallback을 `0.1.2`로 갱신했습니다.
- 당시 README, GitHub Pages 본문·설치기 링크, CHANGELOG, v0.1.2 릴리스 노트, HVC 기록지의 최신 대상과 릴리스 준비 상태를 갱신했습니다.
- 당시 v0.1.2 공개 릴리스, 설치기·앱 자산, 파일 크기·SHA-256, 커밋·CI 실행 결과는 생성·확정 전이었고 공개 후 HVC 기록지에 채울 자리를 마련했습니다.
- 당시 커밋·푸시·태그·GitHub Release는 수행하지 않았으며, Windows 10/11 실기기 HVC도 미검증이었습니다.

### 공개 준비 순서와 Pages 404 방지 (공개 전 기록)

당시 계획한 공개 순서는 `PR CI 통과 → feature head에 v0.1.2 tag → Windows package 성공 및 Release 자산 공개 → PR main 병합 → Pages 배포`였습니다. Release 자산 공개 전에 feature branch를 `main`에 병합하면 Pages가 v0.1.2 다운로드 링크를 먼저 배포해 404가 발생할 수 있으므로, Windows package 성공과 Release 자산 공개를 먼저 확인한 뒤 `main` 병합과 Pages 배포를 진행했습니다.

## v0.1.2 공개 릴리스 최종 증거

- PR [#6](https://github.com/WBmaker2/shift-space-lang-change/pull/6) 병합 완료
- tag commit: `4b8a25ecca4e3b59e360a8c108c385bb488c29c9`
- `main` merge commit: `793d23ee7dc2cdeb85d35531f234ab27d2f2435c`
- tag CI [run 33485263833](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485263833) 성공
- Windows package [run 33485263823](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485263823) 성공
- artifact: `shift-space-lang-change-windows-x64` (id `9791557364`, archive 197,309 bytes)
- [GitHub v0.1.2 Release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.2) 공개 및 [latest 릴리스](https://github.com/WBmaker2/shift-space-lang-change/releases/latest) 확인
- 설치기: [ShiftSpaceLangChange-Setup-0.1.2-x64.exe](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.2/ShiftSpaceLangChange-Setup-0.1.2-x64.exe), 125,247 bytes
- 설치기 SHA-256: `fcfbbe5ae920f64900f013a6fd5f6bf278666b9d2a875405500e90206f567d53`
- 앱 본체: `shift-space-lang-change.exe`, 159,232 bytes
- 앱 본체 SHA-256: `596431ddba854c3efe76d449a53990eb8a959130380f8f9873320c52e8c06c38`
- SHA256SUMS: [다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.2/SHA256SUMS.txt), 201 bytes
- main CI [run 33485642014](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485642014) 성공
- GitHub Pages [run 33485642004](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485642004) 성공, [공개 페이지](https://wbmaker2.github.io/shift-space-lang-change/) HTTP 200
- latest 설치기 `releases/latest/download` HTTP 200 확인
- Windows 10/11 실기기 HVC는 아직 미검증이며, 이 공개 증거는 실기기 HVC 통과를 뜻하지 않습니다.
