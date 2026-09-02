# 한/영 전환 도우미 v0.1.3 (미공개 실패 기록)

2026-09-02 포터블 배포를 준비했으나 Windows package 검증 실패로 GitHub Release를 공개하지 않았습니다. 기존 설치형은 v0.1.2를 유지하며, 수정된 포터블 배포는 v0.1.4에서 제공합니다.

## Added

- `ShiftSpaceLangChange-Portable-0.1.3-x64.zip` 생성을 시도했습니다.
- 포터블 ZIP에는 다음 두 파일만 들어 있습니다.

  ```text
  ShiftSpaceLangChange-Portable-0.1.3-x64/
  ├── ShiftSpaceLangChange.exe
  └── README-PORTABLE.txt
  ```

- Windows package workflow는 파일을 생성했지만 `Verify package outputs`의 EXE 비교가 실패했습니다.
- 설치형·포터블형 UI와 안내는 v0.1.4에서 수정된 검증 로직과 함께 공개합니다.

## Portable scope

포터블은 설치기 없이 압축을 풀어 실행하는 방식입니다. 설치 폴더, 시작 메뉴 바로가기, 제거 프로그램은 만들지 않으며 별도 런타임이나 관리자 권한도 필요하지 않습니다. Windows 10/11 x64와 한국어 Microsoft IME가 필요합니다.

다만 완전히 무흔적인 모드는 아닙니다. 설정은 `HKCU\Software\ShiftSpaceLangChange`에 저장되고, 사용자가 `Windows 시작 시 자동 실행`을 켜면 현재 EXE 경로가 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`에 등록됩니다. 포터블 폴더를 옮긴 뒤에는 자동 실행을 새 위치에서 다시 설정하고, 삭제 전 자동 실행을 끄고 트레이 메뉴에서 종료해야 합니다.

## Verification status

- 로컬에서 `cargo fmt --all -- --check`, `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`를 실행합니다.
- Windows package workflow에서 PowerShell 포터블 생성, ZIP 파일 구조, EXE 바이트 동일성, 설치기·ZIP·EXE SHA-256 manifest를 확인합니다.
- `scripts/verify-pages-site.sh`가 설치형·포터블형 직접 다운로드 URL, 무설치 안내, `README-PORTABLE.txt`, SmartScreen, reduced-motion 계약을 확인합니다.
- Windows 10/11 x64 실기기 HVC는 아직 미검증입니다. 자동 CI·정적 검증·패키지 검증은 실제 IME·트레이·자동 실행·삭제 동작의 통과 증거가 아닙니다.

## 공개 자산 없음

v0.1.3은 검증 실패로 공개 자산과 GitHub Release가 없습니다.

- Windows package 실패 run: [33637305886](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33637305886)
- 실패 단계: `Verify package outputs`
- 후속 수정 릴리스: v0.1.4

## Known limitations

- Authenticode 코드 서명이 없어 SmartScreen 경고가 표시될 수 있습니다. 출처와 SHA-256을 확인한 뒤 사용하세요.
- 관리자 권한 앱에는 Windows UIPI 정책에 따라 전환 입력이 전달되지 않을 수 있습니다.
- Windows ARM64·32비트, 임의 단축키, 자동 업데이트, 음성·녹음·재생 기능은 지원하지 않습니다.
- macOS 개발 환경에서는 PowerShell 압축 생성·Windows 트레이·한국어 IME·자동 실행 레지스트리를 완전히 검증할 수 없습니다.

자세한 실기기 확인 항목은 [Windows HVC 기록지](https://github.com/WBmaker2/shift-space-lang-change/blob/main/docs/HVC-WINDOWS.md)를 참고하세요.
