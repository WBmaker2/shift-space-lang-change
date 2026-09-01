# 한/영 전환 도우미 v0.1.1

2026-09-01 패치 릴리스입니다. 설정 창을 트레이로 숨긴 뒤 복원하는 흐름과 트레이 메뉴 상호작용을 보강했습니다.

## Fixed

- 설정 창의 `트레이로 숨기기` 버튼에서 발생한 `WM_COMMAND`가 부모 창 프로시저에서 앱 이벤트 처리 경로로 전달되지 않던 결함을 수정했습니다.
- 이제 버튼 명령을 앱 전용 큐 메시지로 안전하게 전달하고, 체크박스 상태는 클릭 시점에 보존해 빠른 연속 클릭에서도 올바르게 처리합니다.

## Changed

- 트레이 아이콘 더블 클릭으로 기존 설정 창을 다시 표시하고 전경으로 가져오는 경로를 유지·검증했습니다.
- 트레이 우클릭 메뉴의 `설정 열기`, 현재 활성 단축키 요약, `종료` 동작 검증을 보강했습니다.
- 버튼 알림, 비-클릭 알림 무시, 체크박스 상태 보존, 트레이 명령 및 활성 단축키 요약에 대한 자동 테스트 범위를 넓혔습니다.

## Verification status

- 자동 테스트와 정적 검증은 저장소의 Rust 1.97.1 toolchain 및 CI에서 실행합니다.
- Windows 10/11 x64 실기기 HVC(설치, 한국어 Microsoft IME, 트레이 복원·메뉴, 자동 실행, 제거)는 아직 완료하지 않았습니다. 자동 테스트·패키지 검증을 실기기 HVC 통과로 간주하지 않습니다.

## Security and platform limitations

- 설치기는 Authenticode 서명이 없어 최초 실행 시 Windows SmartScreen 경고가 표시될 수 있습니다.
- 관리자 권한으로 실행 중인 프로그램에는 Windows UIPI 정책에 따라 일반 권한 앱의 `SendInput`이 전달되지 않을 수 있습니다. 앱은 권한 상승을 자동 요청하지 않습니다.

## Package

릴리스 자산은 `ShiftSpaceLangChange-Setup-0.1.1-x64.exe` 이름으로 생성됩니다.

- [v0.1.1 GitHub Release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.1)
- [v0.1.1 설치기 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.1/ShiftSpaceLangChange-Setup-0.1.1-x64.exe)
