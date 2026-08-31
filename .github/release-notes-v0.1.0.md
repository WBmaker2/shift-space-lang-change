# 한/영 전환 도우미 v0.1.0

2026-09-01 공개 선행 릴리스입니다. Windows 실기기 HVC는 공개 후 후속 검증으로 진행하며, 문제가 확인되면 패치 릴리스로 수정합니다.

## 설치

- [ShiftSpaceLangChange-Setup-0.1.0-x64.exe 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Setup-0.1.0-x64.exe)
- [GitHub 최신 릴리스](https://github.com/WBmaker2/shift-space-lang-change/releases/latest)

릴리스 자산 이름은 `ShiftSpaceLangChange-Setup-0.1.0-x64.exe`입니다. 설치기는 관리자 권한을 요구하지 않으며 `%LOCALAPPDATA%\Programs\ShiftSpaceLangChange`에 현재 사용자 범위로 설치합니다.

## 지원 환경

- Windows 10/11 x64
- 한국어 Microsoft IME가 설치된 사용자 계정
- 관리자 권한 없이 실행하는 일반 사용자 프로그램

## 주요 기능

- `Shift + Space`와 `Ctrl + Space` 전역 한/영 전환
- 두 단축키를 각각 켜고 끄는 네이티브 설정 창과 즉시 적용
- 최소 하나의 단축키 유지, 충돌 시 유효한 설정 보존 및 안내
- 시스템 트레이 상주, 창 닫기 시 숨김, 트레이 메뉴 종료
- Windows 로그인 시 자동 실행 설정과 사용자 단위 제거
- 단일 Rust 실행 파일과 Win32 API 기반의 낮은 의존성 구조

## 검증 완료

- 기능 브랜치 CI `33411398369`: Ubuntu core와 Windows job 통과
- RC.3 Windows package run `33413889332`: Windows 테스트·clippy, release EXE, NSIS 컴파일·범위 검사·출력 검증·artifact 업로드 통과
- RC.3 artifact에서 Windows x64 PE 실행 파일과 NSIS 설치기 형식·크기·SHA-256 독립 확인
- 공개 릴리스에 사용할 설치기 파일명과 직접 다운로드 경로를 문서화

## 아직 완료하지 않은 검증 및 제한

- Windows 10/11 x64 실기기 HVC: 미완료. 실제 설치, 한국어 IME 전환, 메모장·브라우저·Office, 트레이, 자동 실행, 충돌 롤백, 제거, 유휴 메모리는 공개 후 확인합니다.
- Authenticode: 서명하지 않은 설치기입니다. 따라서 최초 실행 시 Windows SmartScreen 경고가 표시될 수 있으며, 서명·SmartScreen 신뢰 상태는 보장하지 않습니다.
- UIPI: 관리자 권한으로 실행 중인 프로그램에는 일반 권한 앱의 `SendInput`이 전달되지 않을 수 있습니다. 앱은 관리자 권한을 자동 요청하지 않습니다.

## 알려진 범위

한국어 Microsoft IME가 없으면 앱은 실행되지만 한/영 전환 결과를 확인할 수 없습니다. Windows ARM64·32비트, 임의 단축키, 다른 언어 레이아웃 순환, 자동 업데이트, 네트워크·계정·분석, 음성 기능은 지원하지 않습니다.
