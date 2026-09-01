# 한/영 전환 도우미 v0.1.2

2026-09-01 패치 릴리스 준비 기록입니다. 트레이 콜백 라우팅을 보강해 설정 창 복원과 트레이 메뉴 상호작용을 보완하고, 프로그램 정보 메뉴를 추가했습니다. 이 문서는 공개 전 준비본이며, 공개 완료를 뜻하지 않습니다.

## Fixed

- Windows Shell이 트레이 아이콘에 보낸 콜백을 앱 이벤트 루프로 전달하지 못하던 경로를 수정했습니다.
- 트레이 아이콘을 두 번 클릭하면 기존 설정 창을 복원하고 전경으로 가져오도록 했습니다.
- 트레이 메뉴를 연 뒤 포커스가 정상적으로 돌아오도록 알림 영역 포커스 처리를 보강했습니다.

## Added

- 트레이 아이콘 우클릭 메뉴에 `설정 열기`, 활성 단축키 요약, `프로그램 정보`, `종료`를 제공합니다.
- `프로그램 정보`에서 `한/영 전환 도우미`와 현재 패키지 버전을 표시합니다.
- 트레이 콜백과 앱 내부 큐 메시지의 분리, 더블 클릭·메뉴 명령·프로그램 정보 매핑에 대한 자동 테스트를 추가했습니다.

## Verification status

- 로컬 Rust 1.97.1 `cargo fmt --all -- --check`가 통과했습니다.
- 로컬 호스트 `cargo test --all-targets` 40개와 호스트 `cargo clippy --all-targets -- -D warnings`가 통과했습니다.
- Windows 대상 `cargo check --target x86_64-pc-windows-msvc --all-targets`와 Windows 대상 clippy가 통과했습니다.
- `scripts/verify-pages-site.sh` Pages 사이트 검증이 통과했습니다.
- Windows package workflow/NSIS 패키징은 아직 실행·검증하지 않았습니다.
- Windows 10/11 x64 실기기 HVC는 아직 완료하지 않았습니다. 설치, 한국어 Microsoft IME 입력 전환, 트레이 더블 클릭·우클릭 메뉴, 자동 실행, 제거 결과를 자동 검증 결과만으로 통과 처리하지 않습니다.
- 공개 후 실제 HVC 결과는 [Windows HVC 기록지](https://github.com/WBmaker2/shift-space-lang-change/blob/main/docs/HVC-WINDOWS.md)에 환경·확인일·증거 링크와 함께 기록합니다.

## Security and platform limitations

- 설치기는 Authenticode 코드 서명이 없어 최초 실행 시 Windows SmartScreen 경고가 표시될 수 있습니다. 출처와 파일을 확인한 경우에만 사용자가 직접 허용해야 합니다.
- 관리자 권한으로 실행 중인 프로그램에는 Windows UIPI 정책에 따라 일반 권한 앱의 `SendInput`이 전달되지 않을 수 있습니다. 앱은 권한 상승을 자동 요청하지 않습니다.

## Package status

v0.1.2 자산은 아직 생성 전입니다. 아래 값은 공개 후 실제 결과로 채웁니다.

- 릴리스: 공개 후 기입
- tag commit: 공개 후 기입
- Windows package workflow run: 공개 후 기입
- artifact: 공개 후 기입
- 설치기 파일명: `ShiftSpaceLangChange-Setup-0.1.2-x64.exe`
- 설치기 크기: 공개 후 기입
- 설치기 SHA-256: 공개 후 기입
- 앱 본체 크기: 공개 후 기입
- 앱 본체 SHA-256: 공개 후 기입
- SHA256SUMS: 공개 후 기입
