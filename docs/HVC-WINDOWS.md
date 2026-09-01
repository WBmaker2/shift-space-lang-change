# Windows HVC 기록지

이 문서는 Windows 10/11 x64와 한국어 Microsoft IME가 설치된 실제 사용자 계정에서 수동 확인한 결과를 기록합니다. macOS의 테스트·크로스 컴파일 확인은 Windows HVC 증거로 사용하지 않습니다.

기록 규칙: 각 항목의 `결과`는 기본값을 `미검증`으로 둡니다. 실제로 확인한 뒤에만 `통과` 또는 `실패`로 바꾸고, 확인 환경과 증거 링크를 함께 남깁니다. 링크가 없으면 `없음`이라고 적습니다.

## 최신 HVC 대상 — v0.1.1 패치 릴리스

이 기록지의 최신 HVC 대상은 v0.1.1 패치 릴리스입니다. v0.1.0 사용자 보고에서 `트레이로 숨기기` 버튼을 눌러도 설정 창이 숨겨지지 않는 문제가 확인되었고, v0.1.1에서 수정했습니다. 숨긴 뒤 트레이 아이콘을 더블 클릭해 설정 창을 복원하고, 우클릭 메뉴의 설정 열기·활성 단축키 요약·종료를 사용하는 흐름도 함께 재검증 대상입니다.

진단 결과, 버튼의 `WM_COMMAND`가 부모 창 프로시저로 동기 전달되지만 앱이 큐에서 읽는 메시지만 해석해 해당 이벤트가 소실되는 것이 원인이었습니다. 패치는 부모 프로시저가 인식된 버튼 명령을 앱 전용 큐 메시지로 전달하고, 체크박스 상태는 클릭 시점에 보존하도록 수정합니다. v0.1.1 설치기와 실제 Windows 실기기 HVC는 아직 검증하지 않았으므로 이 문서의 아래 표는 계속 `미검증`으로 유지합니다.

| 패치 재검증 항목 | 결과 | 확인 환경 | 확인일 | 증거 링크 |
| --- | --- | --- | --- | --- |
| `트레이로 숨기기` 클릭 시 창만 숨고 프로세스·트레이 유지 | 미검증 | — | — | 없음 |
| 숨김 후 트레이 아이콘 더블 클릭 시 기존 창 복원·전경 표시 | 미검증 | — | — | 없음 |
| 트레이 우클릭의 `설정 열기`가 기존 창 복원 | 미검증 | — | — | 없음 |
| 트레이 우클릭에 활성 단축키 요약과 `종료` 표시 | 미검증 | — | — | 없음 |
| `종료` 선택 시 메시지 루프와 트레이 아이콘 정상 종료 | 미검증 | — | — | 없음 |
| 체크박스 변경 즉시 반영 및 마지막 활성 단축키 보호 | 미검증 | — | — | 없음 |
| 창 닫기·ESC 숨김, 단일 인스턴스, 단축키 기능 회귀 없음 | 미검증 | — | — | 없음 |

HVC 대상 버전 메모: `v0.1.1 — 설정 창 숨김 및 트레이 복원/메뉴 상호작용 수정`. Windows 실기기 HVC는 아직 미검증이며, 설치기 공개·실기기 확인 후에만 표의 결과를 갱신합니다.

## 과거 자동화 기준선 (RC.3; 최종 HVC 대상 아님)

아래 RC.3 정보는 공개 전 자동화 패키지 검증의 과거 기준선으로 보존합니다. RC.3 artifact는 공개 후 Windows HVC에 사용할 최종 대상이 아니며, 아래 해시를 최종 v0.1.0 해시로 재사용해서는 안 됩니다.

- 버전/tag: `0.1.0-rc.3` / `v0.1.0-rc.3`
- commit: `13098466aad1dae8d4d4246064ec9cce8371422b`
- package run: `https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33413889332`
- artifact: `shift-space-lang-change-windows-x64` (artifact id `9766285006`, GitHub 로그인 필요)
- 설치기: `ShiftSpaceLangChange-Setup-0.1.0-rc.3-x64.exe`
- 설치기 SHA-256: `dbf40fcbe45715fc9892e3c307527861c8159411309ecab24d14ade1bd5a33b9`
- 앱 본체 SHA-256: `a3acb7a1caac814b6ba7521a7cdecd59667fcf2d84f0d7ff6d8c7d02f3b999f2`

## 공개 후 HVC 대상 — 최종 v0.1.0 릴리스 자산

공개 후 HVC는 GitHub Release의 최종 `v0.1.0` 설치기에서 수행합니다. 아래에는 공개 릴리스와 자동 검증에서 확인한 최종 자산 정보를 기록했으며, 이 기록은 Windows 실기기 HVC 통과를 뜻하지 않습니다. HVC 표와 실기기 항목은 실제 Windows 환경에서 확인하기 전까지 `미검증`으로 유지합니다.

- 저장소: [WBmaker2/shift-space-lang-change](https://github.com/WBmaker2/shift-space-lang-change)
- Release: [GitHub v0.1.0 release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.0)
- 최종 설치기 직접 다운로드: [ShiftSpaceLangChange-Setup-0.1.0-x64.exe](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.0/ShiftSpaceLangChange-Setup-0.1.0-x64.exe)
- 버전/tag: `v0.1.0` (최종 공개 확인)
- 최종 package run: [33445159572](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33445159572) (성공)
- 최종 tag commit: `38133bbb52771418dcd68566e4ca93550da21cf0`
- 최종 artifact: `shift-space-lang-change-windows-x64` (id `9777782027`, archive 196,697 bytes, 만료 예정 `2026-11-29T22:13:01Z`)
- 설치기 파일명: `ShiftSpaceLangChange-Setup-0.1.0-x64.exe`
- 설치기 크기: 124,937 bytes
- 설치기 SHA-256: `290fbf96855a5bfcef3d26bed2ec37d1c0f582d937f4306e64188b36b0c181f6`
- 앱 본체 크기: 158,720 bytes
- 앱 본체 SHA-256: `139960bfd1dd658df3fda616f089f5868ba7b1f0d6b98214ae2ecfe0957977e8`
- SHA256SUMS: [SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.0/SHA256SUMS.txt)

## 권장 확인 순서

1. Windows 10/11 x64 일반 사용자 계정에서 최종 v0.1.0 release asset을 내려받고 최종 설치기 SHA-256을 확인합니다.
2. 관리자 권한 없이 설치하고, Microsoft 한국어 IME가 있는 메모장·브라우저·Office에서 두 단축키를 각각 확인합니다.
3. 설정 창에서 두 체크박스를 하나씩 단독 활성화하고 마지막 활성 단축키를 끌 수 없는지 확인합니다.
4. 창 닫기·트레이·중복 실행·로그인 자동 실행·자동 실행 해제를 확인합니다.
5. 다른 프로그램이 단축키를 점유한 상태의 롤백과 작업 관리자 유휴 메모리를 확인합니다.
6. 제거 후 설치 파일, 시작 메뉴 바로가기, HKCU Run 값, 앱 설정이 삭제되는지 확인합니다.
7. 아래 표에 Windows 버전, 결과, 확인일, 화면 또는 영상 증거 링크를 기록합니다.

| 항목 | 결과 | Windows 버전 | 앱 버전 | 확인일 | 증거 링크 |
| --- | --- | --- | --- | --- | --- |
| 관리자 권한 없는 설치 | 미검증 | — | — | — | 없음 |
| 메모장·브라우저·Office에서 두 단축키 | 미검증 | — | — | — | 없음 |
| 단축키 하나만 활성화 | 미검증 | — | — | — | 없음 |
| 마지막 단축키 비활성 거부 | 미검증 | — | — | — | 없음 |
| 창 닫기 후 트레이 상주 | 미검증 | — | — | — | 없음 |
| 로그인 후 background 자동 실행 | 미검증 | — | — | — | 없음 |
| 자동 실행 해제 | 미검증 | — | — | — | 없음 |
| 중복 실행 방지 | 미검증 | — | — | — | 없음 |
| 단축키 충돌 롤백 | 미검증 | — | — | — | 없음 |
| 제거 후 파일·바로가기·Run·설정 삭제 | 미검증 | — | — | — | 없음 |
| 작업 관리자 유휴 메모리 | 미검증 | — | — | — | 없음 |

## 확인 메모

- 한국어 Microsoft IME 설치 상태: 미검증
- 실행 파일·설치 파일의 SHA-256: 최종 v0.1.0 공개 release asset과 [SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.0/SHA256SUMS.txt) 대조 완료; Windows 실기기 HVC는 미검증
- SmartScreen 서명 경고와 사용 안내: 미검증
- 관리자 권한 프로그램 전면 상태의 UIPI 제한 안내: 미검증
- Windows HVC 담당자/확인 환경: 미기록

VoiceOver, 음성 출력, 녹음 및 재생 검증은 이 제품의 범위에서 제외합니다.
