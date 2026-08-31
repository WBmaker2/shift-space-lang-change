# 한/영 전환 도우미 v0.1.0 완료·배포 계획

## 목표

남아 있는 전역 단축키 오류 분류 결함을 수정하고, 실제 Windows CI에서 실행 파일과 NSIS 설치 파일을 만든 뒤 Windows 실기기 HVC를 통과한 경우에만 `main` 병합과 `v0.1.0` 릴리스를 완료한다.

## 사용자 승인

- 사용자는 2026-08-31에 추천 순서 전체 진행을 승인했다.
- 승인된 순서: 오류 수정 → Windows 패키지 CI → Windows HVC → `main` 병합 및 `v0.1.0` 릴리스.
- 원격 저장소가 없으므로 소스 노출을 피하는 안전한 기본값으로 `WBmaker2/shift-space-lang-change` 비공개 저장소를 먼저 사용한다. 공개 전환은 별도 승인 대상이다.

## 범위

### 1. 전역 단축키 오류 분류 수정

- `RegisterHotKey`/`UnregisterHotKey` 실패 직후 원시 Win32 `GetLastError()` 코드를 보존한다.
- `ERROR_HOTKEY_ALREADY_REGISTERED`일 때만 시작 시 한 단축키 fallback 또는 양쪽 충돌 안내를 사용한다.
- 접근 거부·잘못된 핸들 등 다른 오류는 `Fatal`로 전달하여 최상위 오류 대화상자에 한 번만 표시한다.
- 첫 등록과 fallback 등록에서 충돌/비충돌 분류를 각각 테스트 가능한 순수 함수로 고정한다.

### 2. 로컬 및 독립 리뷰

- 집중 테스트 후 전체 테스트, format, native clippy, Windows x64 target check/clippy, diff check, 파일당 500줄 제한을 확인한다.
- 구현 담당과 별도의 Luna 리뷰 담당이 오류 코드 보존, fallback, cleanup 회귀를 검토한다.

### 3. GitHub Windows CI와 패키지

- 원격이 없으면 GitHub 비공개 저장소를 생성하고 `origin`으로 연결한다.
- 기능 브랜치를 push하여 `CI`의 Ubuntu/Windows job을 모두 통과시킨다.
- `Windows package` workflow를 수동 실행하여 다음 artifact를 생성한다.
  - `shift-space-lang-change.exe`
  - `ShiftSpaceLangChange-Setup-0.1.0-x64.exe`
- artifact를 내려받아 파일 존재, 크기, SHA-256을 기록하고 다운로드 링크를 남긴다.

### 4. Windows 실기기 HVC

- `docs/HVC-WINDOWS.md`의 모든 항목을 Windows 10/11 x64와 한국어 Microsoft IME 환경에서 확인한다.
- GitHub Actions runner는 대화형 데스크톱 HVC를 대체하지 않는다.
- 현재 호스트가 macOS이므로 실제 설치, 로그인 자동 실행, 메모장·브라우저·Office 입력 전환, 트레이, 제거, 유휴 메모리는 사용자 또는 연결된 Windows 호스트의 증거가 필요하다.

### 5. 병합·태그·릴리스 게이트

- HVC 전체 통과 전에는 `main` 병합, `v0.1.0` 태그, GitHub Release를 수행하지 않는다.
- HVC 통과 후 기능 브랜치를 `main`에 병합하고 다시 CI를 확인한다.
- `v0.1.0` 태그를 push하고 생성된 설치 파일을 GitHub Release에 첨부한다.
- 최종 보고서에는 릴리스 주소와 설치 파일 다운로드 주소를 클릭 가능한 링크로 제공한다.

## 예상 변경 파일·시스템

- `src/platform/windows/hotkeys.rs`
- `src/platform/windows/app.rs`
- 필요한 순수 분류 함수와 테스트 파일
- `docs/HVC-WINDOWS.md` 및 구현·검증 기록
- GitHub 저장소, Actions runs, artifact, 최종 tag/release

## 불변 조건

- 두 단축키 중 최소 하나는 항상 활성화된다.
- 사용자 단위 HKCU/LOCALAPPDATA 설치이며 관리자 권한·서비스를 추가하지 않는다.
- 입력 기록, 네트워크 전송, 분석 기능을 추가하지 않는다.
- 실제 `SendInput`, 제품 mutex, 실제 Run 키를 자동 테스트에서 건드리지 않는다.
- Rust 소스 파일은 각각 500줄 미만을 유지한다.
- Windows 실기기에서 확인하지 않은 항목을 통과로 기록하지 않는다.

## 완료 조건

1. 오류 분류 수정이 독립 리뷰 승인과 전체 자동 검증을 통과한다.
2. GitHub Ubuntu/Windows CI와 Windows package workflow가 성공한다.
3. Windows artifact 두 개와 SHA-256, 다운로드 링크가 존재한다.
4. HVC 기록지의 모든 필수 항목이 실제 증거와 함께 통과한다.
5. 그 뒤에만 `main`, `v0.1.0`, GitHub Release와 다운로드 링크가 생성된다.

