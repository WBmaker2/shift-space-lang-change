# Windows HVC 기록지

이 문서는 Windows 10/11 x64와 한국어 Microsoft IME가 설치된 실제 사용자 계정에서 수동 확인한 결과를 기록합니다. macOS의 테스트·크로스 컴파일 확인은 Windows HVC 증거로 사용하지 않습니다.

기록 규칙: 각 항목의 `결과`는 기본값을 `미검증`으로 둡니다. 실제로 확인한 뒤에만 `통과` 또는 `실패`로 바꾸고, 확인 환경과 증거 링크를 함께 남깁니다. 링크가 없으면 `없음`이라고 적습니다.

## 최신 HVC 대상 — 공개 v0.1.4 설치형·포터블형

v0.1.4는 기존 설치형과 `ShiftSpaceLangChange-Portable-0.1.4-x64.zip` 포터블형을 공개 제공합니다. 아래 항목은 Windows 10/11 x64 실기기에서 확인하기 전까지 모두 `미검증`으로 유지합니다. Windows runner의 ZIP 구조·해시 검증은 이 표의 실기기 통과를 대신하지 않습니다.

### v0.1.4 공개 자산 및 자동 검증 증거

- 병합 PR: [#10](https://github.com/WBmaker2/shift-space-lang-change/pull/10)
- main merge commit 및 annotated tag 대상: `84e9232de7346fced47647c568e1f896764c3f65` / `v0.1.4`
- tag CI [run 33638803179](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638803179) 성공
- Windows package [run 33638803323](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638803323) 성공
- artifact `shift-space-lang-change-windows-x64` (id `9849985527`, archive 285,229 bytes)
- [GitHub v0.1.4 Release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.4) 및 [latest](https://github.com/WBmaker2/shift-space-lang-change/releases/latest) 공개
- 설치기: [직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.4/ShiftSpaceLangChange-Setup-0.1.4-x64.exe), 125,233 bytes, SHA-256 `8bdf8dcf7392b5d72a5aa0e395b577e9c7096c06db05339116550f69ea993780`
- 포터블 ZIP: [직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.4/ShiftSpaceLangChange-Portable-0.1.4-x64.zip), 87,439 bytes, SHA-256 `2a297c539aa9468547d5115f1a9cae6855a38954ebb936ed08d38e4cfbf85a12`
- 원본 EXE: [직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.4/shift-space-lang-change.exe), 159,232 bytes, SHA-256 `45b8f4daff1b21c5651707ef0d89b985baf654b842f0ececc8a81b812a12fa56`
- [SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.4/SHA256SUMS.txt), 314 bytes, SHA-256 `36d45d6f2f23d0330415a00fab152140ea0f7d1c104818fc65f4157ded4bdcd6`
- [Pages 배포 run 33638754916](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638754916) 성공, [공개 페이지](https://wbmaker2.github.io/shift-space-lang-change/) HTTP 200

위 자동화·정적·브라우저 검증은 Windows 실기기 HVC 통과를 뜻하지 않습니다.

포터블 ZIP은 먼저 압축을 풀고 `ShiftSpaceLangChange.exe`를 실행합니다. 설치 폴더, 시작 메뉴 바로가기, 제거 프로그램은 만들지 않지만 설정은 `HKCU\Software\ShiftSpaceLangChange`에 저장됩니다. 자동 실행을 켜면 현재 EXE 경로가 HKCU Run 키에 등록되므로, 폴더를 옮긴 뒤에는 자동 실행을 다시 설정해야 합니다.

| 포터블 재검증 항목 | 결과 | 확인 환경 | 확인일 | 증거 링크 |
| --- | --- | --- | --- | --- |
| ZIP 압축 해제 후 EXE가 관리자 권한 없이 실행 | 미검증 | — | — | 없음 |
| ZIP 내부 루트 폴더에 EXE와 README-PORTABLE.txt만 존재 | 미검증 | — | — | 없음 |
| 포터블 실행만으로 설치 폴더·시작 메뉴·제거 프로그램이 생성되지 않음 | 미검증 | — | — | 없음 |
| 포터블에서 Shift + Space·Ctrl + Space와 한국어 Microsoft IME 전환 | 미검증 | — | — | 없음 |
| 설정 변경 즉시 반영 및 HKCU 설정 저장 | 미검증 | — | — | 없음 |
| 자동 실행을 켜지 않으면 HKCU Run 항목이 생성되지 않음 | 미검증 | — | — | 없음 |
| 자동 실행을 켜면 현재 포터블 EXE 경로가 Run에 등록됨 | 미검증 | — | — | 없음 |
| 포터블 폴더 이동 후 자동 실행 경로를 다시 설정할 수 있음 | 미검증 | — | — | 없음 |
| 자동 실행 해제 후 Run 항목이 제거됨 | 미검증 | — | — | 없음 |
| 트레이 메뉴 종료 후 포터블 폴더를 삭제할 수 있음 | 미검증 | — | — | 없음 |
| SmartScreen 경고와 GitHub 출처·SHA-256 안내 확인 | 미검증 | — | — | 없음 |

포터블 HVC는 설치형 HVC와 별도로 기록합니다. macOS에서의 PowerShell 실행 불가나 cross-target 빌드 성공은 위 항목을 `통과`로 바꾸는 근거가 아닙니다.

## 이전 HVC 대상 — v0.1.2 공개 릴리스

현재 최신 HVC 대상은 공개된 v0.1.2 릴리스입니다. 트레이 Shell 콜백을 앱 이벤트 큐로 전달하는 경로를 수정하고, 트레이 더블 클릭 복원·우클릭 메뉴의 `설정 열기`, 활성 단축키 요약, `프로그램 정보`, `종료`를 포함합니다. 공개 릴리스와 자동 배포 증거는 아래에 기록했으며, Windows 10/11 실기기 HVC 표의 결과는 아직 모두 `미검증`입니다.

### 공개 v0.1.2 자산 및 자동 배포 증거

- 병합 PR: [#6](https://github.com/WBmaker2/shift-space-lang-change/pull/6)
- tag commit: `4b8a25ecca4e3b59e360a8c108c385bb488c29c9`
- `main` merge commit: `793d23ee7dc2cdeb85d35531f234ab27d2f2435c`
- tag CI [run 33485263833](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485263833) 성공
- Windows package [run 33485263823](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485263823) 성공
- artifact: `shift-space-lang-change-windows-x64` (id `9791557364`, archive 197,309 bytes)
- [GitHub v0.1.2 release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.2) 공개 및 [latest 릴리스](https://github.com/WBmaker2/shift-space-lang-change/releases/latest) 확인
- 설치기 파일명: `ShiftSpaceLangChange-Setup-0.1.2-x64.exe`
- 설치기: [직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.2/ShiftSpaceLangChange-Setup-0.1.2-x64.exe), 125,247 bytes
- 설치기 SHA-256: `fcfbbe5ae920f64900f013a6fd5f6bf278666b9d2a875405500e90206f567d53`
- 앱 본체: `shift-space-lang-change.exe`, 159,232 bytes
- 앱 본체 SHA-256: `596431ddba854c3efe76d449a53990eb8a959130380f8f9873320c52e8c06c38`
- SHA256SUMS: [다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.2/SHA256SUMS.txt), 201 bytes
- main CI [run 33485642014](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485642014) 성공
- GitHub Pages [run 33485642004](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33485642004) 성공, [공개 페이지](https://wbmaker2.github.io/shift-space-lang-change/) HTTP 200
- latest 설치기 `releases/latest/download` HTTP 200 확인

### v0.1.2 실기기 HVC 체크리스트

아직 Windows 10/11 실기기 HVC를 수행하지 않았습니다. 자동 테스트·Windows 대상 빌드 확인·패키지 검증·공개 페이지 확인은 실기기 HVC 통과 증거가 아닙니다.

| 패치 재검증 항목 | 결과 | 확인 환경 | 확인일 | 증거 링크 |
| --- | --- | --- | --- | --- |
| 관리자 권한 없는 v0.1.2 설치 및 실행 | 미검증 | — | — | 없음 |
| 한국어 Microsoft IME에서 두 단축키 입력 전환 | 미검증 | — | — | 없음 |
| 설정 창 숨김 후 트레이 더블 클릭으로 기존 창 복원·전경 표시 | 미검증 | — | — | 없음 |
| 트레이 우클릭의 설정 열기·활성 단축키 요약 표시 | 미검증 | — | — | 없음 |
| 트레이 우클릭의 프로그램 정보에서 프로그램명·v0.1.2 버전 표시 | 미검증 | — | — | 없음 |
| 트레이 우클릭의 종료 선택 시 프로세스·아이콘 정상 종료 | 미검증 | — | — | 없음 |
| 메뉴 취소·종료 뒤 트레이 포커스 복귀 및 아이콘 정상 유지/삭제 | 미검증 | — | — | 없음 |
| 자동 실행·체크박스 즉시 반영·단일 인스턴스·제거 회귀 없음 | 미검증 | — | — | 없음 |

### v0.1.2 제한 사항 기록

- 설치기는 Authenticode 코드 서명이 없어 SmartScreen 경고가 표시될 수 있으며, 경고 없음 여부도 아직 검증하지 않았습니다.
- 관리자 권한 앱에는 Windows UIPI 정책에 따라 일반 권한 앱의 `SendInput`이 전달되지 않을 수 있습니다. 이 제한은 실기기 HVC에서 별도로 확인해야 합니다.

v0.1.2 HVC 대상 버전 메모: `v0.1.2 — 트레이 콜백 라우팅·더블 클릭 복원·우클릭 프로그램 정보 추가`. 공개 릴리스는 완료되었지만 실기기 HVC가 완료되기 전까지 모든 실기기 결과를 `미검증`으로 유지합니다.

## 공개 v0.1.1 HVC 기록 — 과거 대상 및 공개 증거 보존

당시 HVC 대상은 v0.1.1 패치 릴리스였습니다. v0.1.0 사용자 보고에서 `트레이로 숨기기` 버튼을 눌러도 설정 창이 숨겨지지 않는 문제가 확인되었고, v0.1.1에서 수정했습니다. 숨긴 뒤 트레이 아이콘을 더블 클릭해 설정 창을 복원하고, 우클릭 메뉴의 설정 열기·활성 단축키 요약·종료를 사용하는 흐름도 함께 재검증 대상이었습니다.

진단 결과, 버튼의 `WM_COMMAND`가 부모 창 프로시저로 동기 전달되지만 앱이 큐에서 읽는 메시지만 해석해 해당 이벤트가 소실되는 것이 원인이었습니다. 패치는 부모 프로시저가 인식된 버튼 명령을 앱 전용 큐 메시지로 전달하고, 체크박스 상태는 클릭 시점에 보존하도록 수정합니다. v0.1.1 설치기의 공개 URL·파일 크기·SHA-256과 자동 배포 결과는 아래에 기록했지만, 실제 Windows 실기기 HVC는 아직 검증하지 않았으므로 이 문서의 아래 표는 계속 `미검증`으로 유지합니다.

### 후속 패치 후보 — 트레이 콜백 라우팅 및 프로그램 정보

v0.1.1 공개 설치기 대상 사용자 점검에서 트레이 더블 클릭·우클릭 콜백이 창 프로시저에서 앱 이벤트 루프로 전달되지 않는 문제를 제기했고, 코드 경로 대조로 두 번째 결함을 확인했습니다. 후속 패치는 Shell 콜백 메시지와 앱 내부 큐 메시지를 분리하고, 창 프로시저가 정수형 `wParam`·`lParam`을 내부 메시지로 재게시하도록 수정합니다. 우클릭 메뉴에는 `프로그램 정보`를 추가해 `한/영 전환 도우미`와 Cargo 패키지 버전을 네이티브 정보 대화상자로 표시하며, 메뉴 종료 뒤 `NIM_SETFOCUS`와 `WM_NULL` 처리를 보강합니다.

당시 구현 단계는 Cargo 버전을 올리지 않은 검증용 변경이었으며, v0.1.2 공개 준비에서 버전 메타데이터를 갱신했습니다. Windows 10/11 실기기에서 다음 항목을 다시 확인하기 전까지 결과는 `미검증`으로 유지합니다.

- Shell 콜백 재게시 후 더블 클릭으로 기존 창 복원·전경 표시
- 우클릭 메뉴의 `설정 열기`, 활성 단축키 요약, `프로그램 정보`, `종료`
- `프로그램 정보` 대화상자의 프로그램명과 현재 패키지 버전 표시
- 메뉴 취소·종료 뒤 트레이 포커스 복귀와 아이콘 정상 유지/삭제

### 공개 v0.1.1 자산 및 자동 배포 증거

- 병합 PR: [#4](https://github.com/WBmaker2/shift-space-lang-change/pull/4)
- `main` 및 `v0.1.1` tag commit: `3771661647cc693c6cfe9198e5ab3d4bccccbb47`
- `main` CI [run 33472643491](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33472643491) 성공
- GitHub Pages [run 33472643488](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33472643488) 성공
- `v0.1.1` tag CI [run 33472797525](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33472797525) 성공
- Windows package [run 33472797570](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33472797570) 성공
- [GitHub v0.1.1 release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.1) 공개 및 latest 릴리스 확인
- 설치기: [ShiftSpaceLangChange-Setup-0.1.1-x64.exe](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.1/ShiftSpaceLangChange-Setup-0.1.1-x64.exe), 125,062 bytes
- 설치기 SHA-256: `43f7c26eff70dee505314bd8d0d61a78751cddab3e76b763fbe4efabb3c02407`
- 앱 본체: 159,232 bytes
- 앱 본체 SHA-256: `bb85f70cabdf147dcd4a96f379a331d05fb4930341e90018a7d2da6f04ec004e`
- [SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.1/SHA256SUMS.txt) 공개 확인
- [GitHub Pages](https://wbmaker2.github.io/shift-space-lang-change/) HTTP 200 및 v0.1.1 링크·본문 확인

| 패치 재검증 항목 | 결과 | 확인 환경 | 확인일 | 증거 링크 |
| --- | --- | --- | --- | --- |
| `트레이로 숨기기` 클릭 시 창만 숨고 프로세스·트레이 유지 | 미검증 | — | — | 없음 |
| 숨김 후 트레이 아이콘 더블 클릭 시 기존 창 복원·전경 표시 | 미검증 | — | — | 없음 |
| 트레이 우클릭의 `설정 열기`가 기존 창 복원 | 미검증 | — | — | 없음 |
| 트레이 우클릭에 활성 단축키 요약과 `종료` 표시 | 미검증 | — | — | 없음 |
| `종료` 선택 시 메시지 루프와 트레이 아이콘 정상 종료 | 미검증 | — | — | 없음 |
| 체크박스 변경 즉시 반영 및 마지막 활성 단축키 보호 | 미검증 | — | — | 없음 |
| 창 닫기·ESC 숨김, 단일 인스턴스, 단축키 기능 회귀 없음 | 미검증 | — | — | 없음 |

HVC 대상 버전 메모: `v0.1.1 — 설정 창 숨김 및 트레이 복원/메뉴 상호작용 수정`. 설치기 공개와 자동 배포 검증은 완료했지만 Windows 실기기 HVC는 아직 미검증이며, 실기기 확인 후에만 표의 결과를 갱신합니다.

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
