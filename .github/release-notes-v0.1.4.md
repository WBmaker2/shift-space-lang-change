# 한/영 전환 도우미 v0.1.4

2026-09-02 v0.1.3 포터블 패키지 검증 실패를 수정한 패치 릴리스입니다. 기존 설치형을 유지하고, 설치형(추천)과 포터블형을 GitHub Release와 [공개 홍보 페이지](https://wbmaker2.github.io/shift-space-lang-change/)에서 선택할 수 있습니다.

## Fixed

- 압축 엔트리의 Deflate 스트림을 partial read 청크 경계에 의존하지 않고 SHA-256으로 끝까지 읽어 원본 release EXE와 비교하도록 수정했습니다.
- 실패한 v0.1.3 태그는 공개 자산 없이 실패 기록으로 보존합니다.

## Added

- `ShiftSpaceLangChange-Portable-0.1.4-x64.zip`을 추가했습니다.
- 포터블 ZIP에는 다음 두 파일만 들어 있습니다.

  ```text
  ShiftSpaceLangChange-Portable-0.1.4-x64/
  ├── ShiftSpaceLangChange.exe
  └── README-PORTABLE.txt
  ```

- Windows package workflow가 설치기·포터블 ZIP·원본 EXE·`SHA256SUMS.txt`를 생성하고 ZIP 구조, 실행 파일 동일성, SHA-256을 검증합니다.
- 홍보 페이지에 설치형(추천)·포터블형(무설치) 선택 카드와 `prefers-reduced-motion` 대응을 제공합니다.

## Portable scope

포터블은 설치기 없이 압축을 풀어 실행하는 방식입니다. 설치 폴더, 시작 메뉴 바로가기, 제거 프로그램은 만들지 않으며 별도 런타임이나 관리자 권한도 필요하지 않습니다. Windows 10/11 x64와 한국어 Microsoft IME가 필요합니다.

완전히 무흔적인 모드는 아닙니다. 설정은 `HKCU\Software\ShiftSpaceLangChange`에 저장되고, 사용자가 `Windows 시작 시 자동 실행`을 켜면 현재 EXE 경로가 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`에 등록됩니다. 폴더 이동 후 자동 실행을 다시 설정하고, 삭제 전 자동 실행을 끄고 트레이 메뉴에서 종료해야 합니다.

## Verification status

- `cargo fmt --all -- --check`, `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`와 Windows 대상 check/clippy를 통과했습니다.
- Windows package run `Windows package`가 포터블 생성·구조·SHA-256 검증을 수행합니다.
- `scripts/verify-pages-site.sh`가 설치형·포터블형 URL, 무설치·압축 풀기·README-PORTABLE.txt·SmartScreen·reduced-motion 계약을 확인합니다.
- Windows 10/11 x64 실기기 HVC는 아직 미검증입니다. 자동 CI·패키지 검증은 실제 IME·트레이·자동 실행·삭제 통과 증거가 아닙니다.

## Release assets and hashes

- [v0.1.4 GitHub Release](https://github.com/WBmaker2/shift-space-lang-change/releases/tag/v0.1.4)
- [설치형 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Setup-0.1.4-x64.exe)
- [포터블 ZIP 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Portable-0.1.4-x64.zip)
- [원본 EXE 직접 다운로드](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.4/shift-space-lang-change.exe)
- [SHA256SUMS.txt](https://github.com/WBmaker2/shift-space-lang-change/releases/download/v0.1.4/SHA256SUMS.txt)

검증 기준 커밋은 `84e9232de7346fced47647c568e1f896764c3f65`이며, `v0.1.4` annotated tag가 해당 main merge commit에 연결되어 있습니다. [태그 CI run 33638803179](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638803179), [Windows package run 33638803323](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638803323), [Pages 배포 run 33638754916](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638754916)이 성공했습니다. [artifact 9849985527](https://github.com/WBmaker2/shift-space-lang-change/actions/runs/33638803323/artifacts/9849985527)는 285,229바이트이며, 원본 EXE 159,232바이트/`45b8f4daff1b21c5651707ef0d89b985baf654b842f0ececc8a81b812a12fa56`, 설치기 125,233바이트/`8bdf8dcf7392b5d72a5aa0e395b577e9c7096c06db05339116550f69ea993780`, 포터블 ZIP 87,439바이트/`2a297c539aa9468547d5115f1a9cae6855a38954ebb936ed08d38e4cfbf85a12`, `SHA256SUMS.txt` 314바이트/`36d45d6f2f23d0330415a00fab152140ea0f7d1c104818fc65f4157ded4bdcd6`입니다. `SHA256SUMS.txt`와 모두 대조했고, latest 포터블을 재다운로드해 ZIP 해시가 일치함을 확인했습니다.

## Known limitations

- Authenticode 코드 서명이 없어 SmartScreen 경고가 표시될 수 있습니다.
- 관리자 권한 앱에는 Windows UIPI 정책에 따라 전환 입력이 전달되지 않을 수 있습니다.
- Windows ARM64·32비트, 임의 단축키, 자동 업데이트, 음성·녹음·재생 기능은 지원하지 않습니다.
- macOS에서는 PowerShell 압축 생성과 Windows 실기기 HVC를 완전히 검증할 수 없습니다.
