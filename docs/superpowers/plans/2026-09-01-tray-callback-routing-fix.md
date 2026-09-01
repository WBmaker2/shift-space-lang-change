# 트레이 콜백 라우팅 수정 계획

## 목표

Windows 트레이 아이콘의 더블 클릭과 우클릭 콜백이 실제 창 프로시저에서 앱 이벤트 루프로 전달되도록 수정한다. 더블 클릭은 기존 설정 창을 복원하고, 우클릭은 `설정 열기`, 활성 단축키 요약, `프로그램 정보`, `종료` 메뉴를 표시해야 한다. `프로그램 정보`는 프로그램 이름과 빌드에 포함된 현재 버전을 Windows 정보 대화상자로 보여준다.

## 확인된 원인

- `Shell_NotifyIconW`는 `NOTIFYICONDATAW.hWnd`에 지정한 창의 window procedure로 `uCallbackMessage`를 보낸다.
- 현재 트레이 콜백 ID `WM_APP_TRAY`는 `message_loop`와 `read_ui_event`에서만 처리한다.
- 실제 콜백을 받는 `window_proc`에는 `WM_APP_TRAY` 처리 분기가 없어 `DefWindowProcW`로 전달되고 이벤트가 소실된다.
- 따라서 같은 콜백 경로를 사용하는 더블 클릭과 우클릭이 모두 작동하지 않는 사용자 HVC 결과와 코드 경로가 일치한다.

## 수정 범위

- `src/platform/windows/ui/window.rs`
  - Shell 전용 트레이 콜백 메시지와 앱 큐 전용 트레이 이벤트 메시지를 분리한다.
  - `window_proc`가 Shell 콜백의 정수형 `wParam`·`lParam`을 앱 큐 전용 메시지로 재게시한다.
  - 큐 기반 UI 해석기에서 원본 Shell 콜백을 직접 소비하지 않도록 정리한다.
- `src/platform/windows/app.rs`
  - 앱 큐 전용 트레이 이벤트만 `TrayIcon::read_event`로 전달한다.
  - `프로그램 정보` 이벤트를 처리해 프로그램 이름과 `CARGO_PKG_VERSION` 기반 버전을 네이티브 정보 대화상자로 표시한다.
- `src/platform/windows/ui/tray.rs`
  - Shell에는 원본 콜백 ID를 등록한다.
  - 우클릭 메뉴에 `프로그램 정보` 항목을 추가한다.
  - 메뉴 종료 후 알림 영역에 포커스를 돌려주는 Windows 권장 처리를 보강한다.
- `src/ui_model.rs`
  - `프로그램 정보` UI 이벤트와 충돌하지 않는 메뉴 명령 ID를 추가한다.
- 관련 테스트
  - 원본 콜백이 window procedure 전달 대상으로 분류되는지 검증한다.
  - 큐 전용 이벤트만 앱 트레이 처리기로 들어가는지 검증한다.
  - `NOTIFYICON_VERSION_4`의 LOWORD 이벤트 해석, 더블 클릭, 우클릭 명령 매핑을 유지한다.
  - `프로그램 정보` 메뉴 명령이 전용 UI 이벤트로 매핑되는지 검증한다.
- `docs/HVC-WINDOWS.md` 및 구현 보고서
  - v0.1.1 HVC에서 확인된 두 번째 결함과 패치 재검증 항목을 기록한다.

## 수용 기준

1. 트레이 아이콘 더블 클릭 시 숨겨진 기존 설정 창이 다시 표시되고 전경으로 이동한다.
2. 트레이 아이콘 우클릭 시 `설정 열기`, 활성 단축키 요약, `프로그램 정보`, `종료` 메뉴가 나타난다.
3. `프로그램 정보` 선택 시 프로그램 이름과 현재 패키지 버전이 표시된다. 버전의 단일 기준은 `CARGO_PKG_VERSION`이다.
4. `설정 열기`와 `종료`가 각각 기존 창 복원과 정상 프로세스 종료를 수행한다.
5. `트레이로 숨기기`, 체크박스 즉시 반영, 창 닫기·ESC 숨김, 단축키 기능이 회귀하지 않는다.
6. Shell 콜백과 앱 내부 큐 메시지 ID는 종료 요청 및 기존 UI 명령과 충돌하지 않는다.
7. 변경 Rust 파일은 각각 500줄 미만을 유지한다.

## 검증 계획

- 라우팅 회귀 테스트를 먼저 추가하고 수정 전 실패·수정 후 통과를 확인한다.
- `cargo fmt --all -- --check`
- `cargo test --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo check --target x86_64-pc-windows-msvc --all-targets`
- `cargo clippy --target x86_64-pc-windows-msvc --all-targets -- -D warnings`
- Windows CI에서 Windows 전용 테스트와 릴리스 빌드를 확인한다.
- 실제 Windows 10/11에서 숨김 → 더블 클릭 복원 → 우클릭 메뉴 → 프로그램 정보/설정 열기/종료를 다시 HVC한다.

## 공개 준비 순서

공개 순서는 `PR CI 통과 → feature head에 v0.1.2 tag → Windows package 성공 및 Release 자산 공개 → PR main 병합 → Pages 배포`로 진행한다. feature branch가 아직 `main`에 병합되지 않은 상태에서 Pages를 먼저 배포하면 최신 문서·링크가 반영되지 않아 404가 발생할 수 있으므로, Windows package와 Release 자산을 먼저 공개한 뒤 `main` 병합과 Pages 배포를 진행한다. 이 문서는 준비 계획이며 공개 완료를 뜻하지 않는다.

## 제약과 위험

- `window_proc`에서는 controller나 tray 객체를 직접 참조하지 않고, 정수 메시지만 앱 큐로 넘긴다.
- 원본 콜백과 내부 전달 메시지를 분리해 재게시 루프를 방지한다.
- 새 창을 만들지 않고 기존 설정 창 핸들을 재사용한다.
- 이번 구현 단계에서는 커밋·푸시·공개 재배포를 수행하지 않는다. 정보 대화상자의 버전은 Cargo 패키지 버전을 자동으로 사용하며, 검증 후 릴리스 준비에서 패치 버전을 `0.1.2`로 올리고 공개 재배포 여부를 별도로 승인받는다.

## 작업 방식과 기록

- 승인 후 Luna 구현 에이전트가 테스트 우선으로 수정한다.
- 별도 Luna 리뷰 에이전트가 Win32 메시지 흐름과 회귀 위험을 독립 검토한다.
- 구현, 테스트, 리뷰, 남은 실기기 HVC를 후속 보고서에 기록한다.
