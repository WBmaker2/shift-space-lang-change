# 한/영 전환 도우미 v0.1.3

2026-09-02 포터블 배포 채널을 추가한 패치 릴리스입니다. 기존 설치형은 계속 유지하며, GitHub Release와 [공개 홍보 페이지](https://wbmaker2.github.io/shift-space-lang-change/)에서 두 방식을 선택할 수 있습니다.

## Added

- `ShiftSpaceLangChange-Portable-0.1.3-x64.zip`을 추가했습니다.
- 포터블 ZIP에는 다음 두 파일만 들어 있습니다.

  ```text
  ShiftSpaceLangChange-Portable-0.1.3-x64/
  ├── ShiftSpaceLangChange.exe
  └── README-PORTABLE.txt
  ```

- Windows package workflow가 설치기·포터블 ZIP·원본 EXE·`SHA256SUMS.txt`를 생성하고, 포터블 EXE가 원본 release EXE와 같은지 확인합니다.
- 설치형(추천)과 포터블형(무설치)의 차이, 압축 해제 순서, SmartScreen 안내를 홍보 페이지와 README에 추가했습니다.

## Portable scope

포터블은 설치기 없이 압축을 풀어 실행하는 방식입니다. 설치 폴더, 시작 메뉴 바로가기, 제거 프로그램은 만들지 않으며 별도 런타임이나 관리자 권한도 필요하지 않습니다. Windows 10/11 x64와 한국어 Microsoft IME가 필요합니다.

다만 완전히 무흔적인 모드는 아닙니다. 설정은 `HKCU\Software\ShiftSpaceLangChange`에 저장되고, 사용자가 `Windows 시작 시 자동 실행`을 켜면 현재 EXE 경로가 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`에 등록됩니다. 포터블 폴더를 옮긴 뒤에는 자동 실행을 새 위치에서 다시 설정하고, 삭제 전 자동 실행을 끄고 트레이 메뉴에서 종료해야 합니다.

## Verification status

- 로컬에서 `cargo fmt --all -- --check`, `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`를 실행합니다.
- Windows package workflow에서 PowerShell 포터블 생성, ZIP 파일 구조, EXE 바이트 동일성, 설치기·ZIP·EXE SHA-256 manifest를 확인합니다.
- `scripts/verify-pages-site.sh`가 설치형·포터블형 직접 다운로드 URL, 무설치 안내, `README-PORTABLE.txt`, SmartScreen, reduced-motion 계약을 확인합니다.
- Windows 10/11 x64 실기기 HVC는 아직 미검증입니다. 자동 CI·정적 검증·패키지 검증은 실제 IME·트레이·자동 실행·삭제 동작의 통과 증거가 아닙니다.

## Release assets and hashes

공개 전 Windows package workflow가 아래 파일의 실제 SHA-256을 생성합니다. 이 문서에 해시를 복사할 때는 workflow artifact의 `SHA256SUMS.txt`와 반드시 대조합니다.

- [v0.1.3 GitHub Release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.3)
- [설치형 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Setup-0.1.3-x64.exe)
- [포터블 ZIP 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Portable-0.1.3-x64.zip)
- [SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.3/SHA256SUMS.txt)
- GitHub Actions Windows package: 공개 후 실제 run 링크와 artifact ID를 기록합니다.

## Known limitations

- Authenticode 코드 서명이 없어 SmartScreen 경고가 표시될 수 있습니다. 출처와 SHA-256을 확인한 뒤 사용하세요.
- 관리자 권한 앱에는 Windows UIPI 정책에 따라 전환 입력이 전달되지 않을 수 있습니다.
- Windows ARM64·32비트, 임의 단축키, 자동 업데이트, 음성·녹음·재생 기능은 지원하지 않습니다.
- macOS 개발 환경에서는 PowerShell 압축 생성·Windows 트레이·한국어 IME·자동 실행 레지스트리를 완전히 검증할 수 없습니다.

자세한 실기기 확인 항목은 [Windows HVC 기록지](https://github.com/WBmaker2/shift-space-lang-change/blob/main/docs/HVC-WINDOWS.md)를 참고하세요.
