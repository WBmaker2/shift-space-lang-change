# 한/영 전환 도우미 포터블 버전 구현 계획

- 작성일: 2026-09-02
- 계획 기준: `origin/main` (`986eb69`, v0.1.2 포함)
- 상태: 사용자 승인 완료, v0.1.4 공개 릴리스·Pages 배포 완료 — Windows 실기기 HVC 미검증

## 1. 목표

Windows 10/11 x64 사용자가 설치 프로그램을 실행하지 않고 압축을 푼 뒤 실행 파일을 더블 클릭해 바로 사용할 수 있는 포터블 배포본을 추가한다. 기존 설치형 배포는 유지하고, GitHub Release와 공개 홍보페이지에서 설치형과 포터블형 중 원하는 방식을 명확하게 선택해 다운로드할 수 있게 한다.

## 2. 현재 상태와 구현 판단

- Windows package workflow는 이미 `target/release/shift-space-lang-change.exe`와 NSIS 설치기를 만든다.
- GitHub Release에는 앱 본체 실행 파일이 포함되어 있지만, 포터블 전용 ZIP·사용 안내·고정된 파일 구조가 없어 일반 사용자가 무설치 배포본으로 인식하기 어렵다.
- 홍보페이지는 설치기 다운로드와 설치 방법만 강조한다.
- 앱은 Rust/Win32 단일 실행 파일이므로 WebView, .NET, 별도 런타임, DLL 묶음이 필요하지 않다. 따라서 앱 로직을 복제하거나 별도 에디션을 만들기보다 검증된 동일 바이너리를 포터블 ZIP으로 패키징하는 방식이 가장 작고 안전하다.

### 이번 계획에서 말하는 포터블의 범위

포터블은 **설치기 없이 압축 해제 후 즉시 실행**되는 배포 방식이다. 설치 폴더, 시작 메뉴 바로가기, 제거 프로그램은 만들지 않는다.

다만 현재 앱은 단축키 설정을 `HKCU\\Software\\ShiftSpaceLangChange`에 저장하며, 사용자가 `Windows 시작 시 자동 실행`을 직접 켜면 현재 실행 파일 경로를 HKCU Run 항목에 기록한다. 즉, 이번 범위는 “무설치”이며 “레지스트리를 전혀 사용하지 않는 무흔적 모드”는 아니다. 이를 다운로드 화면과 포터블 안내문에 투명하게 설명한다.

## 3. 사용자 흐름

```mermaid
flowchart LR
    A[홍보페이지] --> B{배포 방식 선택}
    B -->|일반 사용자 추천| C[설치형 EXE]
    B -->|설치 없이 사용| D[포터블 ZIP]
    D --> E[압축 풀기]
    E --> F[ShiftSpaceLangChange.exe 실행]
    F --> G[트레이에서 단축키 설정]
```

포터블 ZIP 내부 구조는 다음으로 고정한다.

```text
ShiftSpaceLangChange-Portable-<version>-x64/
├── ShiftSpaceLangChange.exe
└── README-PORTABLE.txt
```

공개 파일명은 `ShiftSpaceLangChange-Portable-<version>-x64.zip`으로 한다. v0.1.3 후보의 패키지 검증 실패를 보존하고, 수정된 첫 공개 릴리스 후보는 `v0.1.4`로 잡는다.

## 4. 구현 범위

### 4.1 포터블 패키지 생성

새 PowerShell 스크립트 `scripts/build-portable.ps1`을 추가한다.

1. 입력 버전을 기존 workflow와 같은 SemVer 규칙으로 검증한다.
2. release 바이너리를 포터블 폴더의 `ShiftSpaceLangChange.exe`로 복사한다.
3. `README-PORTABLE.txt`를 함께 넣는다.
4. `Compress-Archive`로 버전이 포함된 ZIP을 `dist`에 생성한다.
5. 이전 산출물이 남아 있어 결과에 섞이지 않도록 지정된 임시 패키지 폴더만 안전하게 재생성한다.

`README-PORTABLE.txt`에는 다음을 짧게 기록한다.

- ZIP 안에서 직접 실행하지 말고 먼저 압축을 풀 것
- Windows 10/11 x64와 한국어 Microsoft IME가 필요함
- 설치·관리자 권한·별도 런타임이 필요하지 않음
- 완전 종료는 트레이 메뉴의 `종료`를 사용할 것
- 설정은 현재 사용자 레지스트리에 저장됨
- 자동 실행을 켠 뒤 폴더를 옮기면 경로가 달라지므로 다시 설정할 것
- 삭제 전 자동 실행을 끄고 앱을 종료한 뒤 폴더를 지울 것
- 코드 서명이 없어 SmartScreen 경고가 표시될 수 있음

### 4.2 Windows package workflow 확장

`.github/workflows/windows-package.yml`에서 release EXE와 NSIS 설치기를 만든 다음 포터블 빌드 스크립트를 실행한다.

- workflow artifact에 다음 세 산출물을 포함한다.
  - `shift-space-lang-change.exe` — 원본 진단/검증용
  - `ShiftSpaceLangChange-Setup-<version>-x64.exe` — 설치형
  - `ShiftSpaceLangChange-Portable-<version>-x64.zip` — 포터블형
- 태그 빌드와 수동 빌드 모두 같은 버전 규칙·파일 구조를 사용한다.
- GitHub Release 공개 전에는 설치기와 포터블 ZIP의 SHA-256을 함께 생성하고 기록한다.

### 4.3 패키지 검증 강화

`scripts/verify-package.ps1`을 확장해 다음 조건을 실패로 처리한다.

- 포터블 ZIP이 없거나 크기가 0임
- ZIP 루트 폴더명 또는 파일명이 버전 계약과 다름
- ZIP에 실행 파일과 안내문 이외의 예상하지 못한 파일이 포함됨
- ZIP 내부 실행 파일이 원본 release EXE와 바이트 단위로 다름
- 설치기 또는 포터블 ZIP의 체크섬 생성에 실패함

가능하면 패키징 규칙을 순수 파일 검증으로 분리해 Windows runner에서 반복 실행 가능하게 하고, 실제 IME/트레이 동작 검증과 혼동하지 않는다.

### 4.4 홍보페이지 다운로드 선택 UI

`site/index.html`과 `site/styles.css`의 다운로드 영역을 “설치형 / 포터블형” 선택 카드로 바꾼다.

- 설치형은 `추천` 표시와 기존 설명을 유지한다.
- 포터블형은 `무설치` 표시, ZIP 파일명, `압축 풀기 → 실행` 2단계 안내를 보여준다.
- 포터블 직접 다운로드 주소는 다음 패턴을 사용한다.
  - `https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Portable-<version>-x64.zip`
- 모바일에서는 두 선택지가 세로로 쌓이고, 375px 폭에서 가로 스크롤이나 파일명 잘림이 없어야 한다.
- 핵심 다운로드 버튼에는 현재 페이지의 `gi-pulse` 계열 강조 효과를 적용하되 `prefers-reduced-motion`에서는 애니메이션을 끈다.
- 설치 방법, FAQ, 지원 범위, 업데이트 내역에도 포터블의 무설치 범위·삭제 방법·레지스트리 설정·SmartScreen 안내를 반영한다.
- 업데이트 내역에 `2026-09-02 포터블 ZIP 배포 및 다운로드 선택 추가`를 기록한다.

### 4.5 정적 페이지 검증

`scripts/verify-pages-site.sh`에 다음 계약을 추가한다.

- 설치형과 포터블형 직접 다운로드 URL이 모두 존재함
- `무설치`, `압축 풀기`, `README-PORTABLE.txt`, SmartScreen 안내가 존재함
- 포터블형을 설치형으로 오인하게 하는 문구가 없음
- 프로젝트 하위 경로를 깨뜨리는 root-relative URL이 없음
- `prefers-reduced-motion` 대응 스타일이 존재함

### 4.6 문서와 릴리스 기록

- `README.md`: 설치형과 포터블형 사용법, 차이, 삭제 방법, 직접 다운로드 링크를 구분한다.
- `CHANGELOG.md`: 포터블 배포 채널과 홍보페이지 선택 UI를 기록한다.
- `.github/release-notes-v0.1.4.md`: 자동 검증, 알려진 제한, 실기기 HVC 상태, 설치형/포터블형 해시를 기록한다.
- `docs/HVC-WINDOWS.md`: 설치형과 별도로 포터블 실행·이동·자동 실행·삭제 확인 항목을 추가한다.
- 공개 후 보고에는 GitHub Release, 설치형 직접 다운로드, 포터블 직접 다운로드, GitHub Pages 홍보페이지 링크를 모두 클릭 가능하게 제공한다.

## 5. 예상 변경 파일

| 파일 | 변경 내용 |
|---|---|
| `scripts/build-portable.ps1` | 포터블 폴더와 ZIP 생성 |
| `scripts/verify-package.ps1` | ZIP 구조·내용·동일 바이너리 검증 |
| `.github/workflows/windows-package.yml` | 포터블 생성 및 artifact 포함 |
| `site/index.html` | 설치형/포터블형 다운로드 선택, 안내, 업데이트 내역 |
| `site/styles.css` | 다운로드 카드, 모바일, reduced-motion 스타일 |
| `scripts/verify-pages-site.sh` | 포터블 링크·문구·접근성 계약 검증 |
| `README.md` | 두 배포 방식과 포터블 삭제/자동 실행 안내 |
| `CHANGELOG.md` | 사용자에게 보이는 변경 기록 |
| `.github/release-notes-v0.1.4.md` | 새 릴리스 검증·제한·해시 기록 |
| `docs/HVC-WINDOWS.md` | 포터블 실기기 HVC 체크리스트 |

기존 앱 동작이 그대로 재사용되므로 특별한 결함이 발견되지 않는 한 `src/**`는 변경하지 않는다. 모든 코드 파일은 500줄 미만 규칙을 유지한다.

## 6. 수용 기준

### 자동 검증

1. `cargo test --all-targets`, fmt, clippy, Windows x64 check가 통과한다.
2. Windows package workflow가 설치기와 포터블 ZIP을 모두 생성한다.
3. ZIP을 새 폴더에 풀었을 때 지정한 두 파일만 존재하고 실행 파일 해시가 원본과 일치한다.
4. Pages 정적 검증이 설치형·포터블형 링크와 필수 안내를 확인한다.
5. 사이트의 375px/데스크톱 레이아웃, 키보드 포커스, reduced-motion, 콘솔 오류 0건을 확인한다.

### Windows 실기기 HVC

1. 설치되지 않은 Windows 10/11 x64 계정에서 ZIP 압축 해제 후 EXE가 관리자 권한 없이 실행된다.
2. `Shift + Space`, `Ctrl + Space`, 설정 즉시 반영, 트레이 숨김/복원/종료가 동작한다.
3. 포터블 실행만으로 설치 폴더, 시작 메뉴 항목, 제거 프로그램이 생성되지 않는다.
4. 자동 실행을 켜지 않은 상태에서는 Run 항목이 생성되지 않는다.
5. 자동 실행을 켜면 현재 포터블 EXE 경로로 등록되고, 끄면 제거된다.
6. 앱 종료 후 포터블 폴더를 삭제할 수 있다.
7. 실제 기기에서 확인하지 못한 항목은 자동 CI 통과와 분리해 `미검증`으로 남긴다.

VoiceOver 구현·검증은 범위에서 제외한다. 음성, 녹음, TTS 기능도 추가하지 않는다.

## 7. 위험과 대응

| 위험 | 대응 |
|---|---|
| 사용자가 “포터블”을 무흔적 실행으로 이해할 수 있음 | 페이지와 안내문에 HKCU 설정 저장 및 선택적 Run 항목을 명시 |
| ZIP 안에서 직접 EXE를 실행해 경로·업데이트가 혼란스러움 | `먼저 압축 풀기`를 버튼 주변과 안내문 첫 줄에 반복 |
| 포터블 폴더 이동 후 자동 실행 경로가 깨짐 | 자동 실행 전 고정 위치 배치 및 이동 후 재설정 안내 |
| 미서명 EXE/ZIP의 SmartScreen 경고 | 저장소·릴리스 출처와 SHA-256 제공, 경고 가능성을 숨기지 않음 |
| 페이지 버전과 latest asset 파일명이 불일치 | 릴리스 버전·페이지·검증 스크립트를 같은 변경으로 묶고 공개 후 HTTP 200 확인 |
| macOS 개발 환경에서 실제 Windows 동작을 과대평가 | Windows runner 자동 검증과 Windows 10/11 실기기 HVC를 별도 게이트로 유지 |

## 8. 구현 순서와 승인 게이트

1. 사용자가 본 계획의 포터블 정의와 범위를 검토·승인한다.
2. 최신 `origin/main`을 기준으로 안전한 작업 브랜치/워크트리를 준비한다.
3. 포터블 패키지 생성·검증을 먼저 구현하고 Windows workflow에서 확인한다.
4. 검증된 파일명과 URL을 기준으로 홍보페이지와 문서를 수정한다.
5. 전체 자동 검증과 독립 리뷰를 수행한다.
6. 사용자 승인 후에만 커밋·푸시·태그·GitHub Release·Pages 배포를 진행한다.
7. 공개 후 설치형/포터블 다운로드 HTTP 200, 실제 ZIP 내용, Pages의 좁은 화면과 콘솔을 다시 확인한다.

구현 작업은 계획 승인 후 `gpt-5.6-luna`가 담당하고, 계획 대비 검토와 최종 릴리스 판정은 오케스트레이터가 수행한다.

## 9. 계획 완료 정의

이 문서가 사용자에게 검토되고, 다음 두 경계가 합의되면 구현 계획 수립이 완료된다.

- 포터블은 설치 없이 실행되지만 설정 저장을 위해 현재 사용자 레지스트리를 사용한다.
- 설치형은 계속 권장 기본값으로 유지하고 포터블형을 동등한 공개 다운로드 선택지로 추가한다.

## 구현·검증 기록

- 2026-09-02 사용자 승인 후 `codex/v0.1.4-release` worktree에서 포터블 ZIP 생성·검증, Windows workflow, 설치형/포터블형 다운로드 UI, reduced-motion, Pages 정적 검증, README·CHANGELOG·v0.1.4 릴리스 노트·Windows HVC 문서를 구현했습니다.
- `cargo fmt --all -- --check`, `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`, Windows 대상 `cargo check`·clippy, `bash scripts/verify-pages-site.sh`를 통과했습니다.
- 브라우저에서 375px·1440px 레이아웃의 가로 overflow 없음, 키보드 포커스 outline, 콘솔 오류 0건을 확인했습니다.
- macOS에서는 PowerShell 포터블 ZIP 생성·Windows runner 패키지 실행과 Windows 10/11 x64 실기기 HVC를 검증하지 못했습니다. 해당 항목은 미검증으로 유지합니다.
- 계획의 예상 변경 파일 외에 앱 버전 표시 정합성을 위해 `Cargo.toml`, `Cargo.lock`, NSIS 기본 버전 메타데이터를 0.1.4로 갱신했습니다(plan drift). v0.1.3은 패키지 검증 실패 기록으로 보존합니다.
- v0.1.3 태그는 실패 증거 보존을 위해 유지합니다. v0.1.4는 PR [#10](https://github.com/WBmaker2/shift-space-lang-change/pull/10), merge SHA `84e9232de7346fced47647c568e1f896764c3f65`, annotated tag, [GitHub Release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.4), [Windows package run 33638803323](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638803323), [Pages run 33638754916](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638754916)까지 공개·검증 완료했습니다.

## 2026-09-03 후속 승인 addendum — 320px 모바일 다운로드 카드

- 사용자 승인: 공개 CUA에서 320px 보완 수정을 진행합니다. 기존 v0.1.4 바이너리·GitHub Release·Windows 패키지는 변경하지 않고 Pages CSS/문서만 수정합니다.
- 관찰된 원인: 375px viewport에서 `documentElement.clientWidth`가 360px인데 `scrollWidth`가 369px이었고, `.download-choice.recommended` 및 `.download-choice.portable`이 grid parent 폭(320px)을 넘어섰습니다. 모바일 grid의 `1fr` track과 카드의 기본 `min-width: auto`가 포터블 안내 목록의 긴 min-content 문구를 줄이지 못해 가로 overflow를 만들었습니다. 앞선 CSS cache-bust 이후에도 이 콘텐츠 기반 intrinsic minimum이 남아 있음을 독립 CUA로 확인했습니다.
- 최소 수정 범위: `.download-choice`와 내부 다운로드 버튼의 최소 너비를 0으로 허용하고, 모바일 grid track을 `minmax(0, 1fr)`로 고정하며, 긴 버튼/배지 문구가 안전하게 줄바꿈되도록 필요한 CSS만 보완합니다. 업데이트 내역과 CHANGELOG에는 Pages 후속 수정으로 기록합니다.
- 수용 기준: 공개 Pages에서 캐시가 제거된 상태로 320px·375px·1440px 각각 `scrollWidth <= clientWidth`(가로 overflow 없음), 설치형·포터블 버튼 표시, hero 이미지 로드, 콘솔 오류 0건을 확인합니다. 앱 버전과 Release 자산은 0.1.4 그대로 유지합니다.
