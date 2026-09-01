# 한/영 전환 도우미 v0.1.2

2026-09-01 공개 패치 릴리스입니다. 트레이 콜백 라우팅을 보강해 설정 창 복원과 트레이 메뉴 상호작용을 보완하고, 프로그램 정보 메뉴를 추가했습니다.

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
- Windows package workflow [run 33485263823](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485263823)가 성공했고 NSIS 패키징과 산출물 검증을 완료했습니다.
- Windows 10/11 x64 실기기 HVC는 아직 완료하지 않았습니다. 설치, 한국어 Microsoft IME 입력 전환, 트레이 더블 클릭·우클릭 메뉴, 자동 실행, 제거 결과를 자동 검증 결과만으로 통과 처리하지 않습니다.
- 실제 HVC 결과는 [Windows HVC 기록지](https://github.com/WBmaker2/shift-space-lang-change/blob/main/docs/HVC-WINDOWS.md)에 환경·확인일·증거 링크와 함께 기록합니다.

## Security and platform limitations

- 설치기는 Authenticode 코드 서명이 없어 최초 실행 시 Windows SmartScreen 경고가 표시될 수 있습니다. 출처와 파일을 확인한 경우에만 사용자가 직접 허용해야 합니다.
- 관리자 권한으로 실행 중인 프로그램에는 Windows UIPI 정책에 따라 일반 권한 앱의 `SendInput`이 전달되지 않을 수 있습니다. 앱은 권한 상승을 자동 요청하지 않습니다.

## Public release evidence

v0.1.2 공개 릴리스와 공개 페이지 배포를 확인했습니다. Windows 실기기 HVC 결과만 아직 미검증입니다.

- PR: [#6](https://github.com/WBmaker2/shift-space-lang-change/pull/6)
- tag commit: `4b8a25ecca4e3b59e360a8c108c385bb488c29c9`
- main merge commit: `793d23ee7dc2cdeb85d35531f234ab27d2f2435c`
- tag CI: [run 33485263833](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485263833) (성공)
- Windows package: [run 33485263823](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485263823) (성공)
- artifact: `shift-space-lang-change-windows-x64` (id `9791557364`, archive 197,309 bytes)
- Release: [v0.1.2 GitHub Release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.2)
- latest 릴리스: [GitHub latest release](https://github.com/WBmaker2/shift-space-lang-change/releases/latest) 확인
- 설치기 파일명: `ShiftSpaceLangChange-Setup-0.1.2-x64.exe`
- 설치기: [직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.2/ShiftSpaceLangChange-Setup-0.1.2-x64.exe), 125,247 bytes
- 설치기 SHA-256: `fcfbbe5ae920f64900f013a6fd5f6bf278666b9d2a875405500e90206f567d53`
- 앱 본체: `shift-space-lang-change.exe`, 159,232 bytes
- 앱 본체 SHA-256: `596431ddba854c3efe76d449a53990eb8a959130380f8f9873320c52e8c06c38`
- SHA256SUMS: [다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.2/SHA256SUMS.txt), 201 bytes
- main CI: [run 33485642014](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485642014) (성공)
- GitHub Pages: [run 33485642004](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485642004) (성공), [공개 페이지](https://wbmaker2.github.io/shift-space-lang-change/) HTTP 200
- latest 설치기: `releases/latest/download` HTTP 200 확인
