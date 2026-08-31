# 한/영 전환 도우미 v0.1.0 배포 진행 기록

## 승인된 순서

오류 수정 → Windows CI/package → Windows 실기기 HVC → `main` 병합 → `v0.1.0` 릴리스 순서로 진행한다.

## 오류 수정 게이트

- 최종 수정 HEAD: `389938464991de587195b6a79c0abda950c03431`
- `ERROR_HOTKEY_ALREADY_REGISTERED`만 충돌로 처리하고 다른 Win32 오류는 Fatal로 보존한다.
- 실제 등록 집합 기준 reconciliation, rollback cleanup, fallback fatal 테스트를 추가했다.
- 독립 리뷰 최종 결과: APPROVED
- push 전 독립 검증: 테스트 36개, fmt, native/Windows clippy, Windows x64 check, diff check, 파일당 500줄 제한 통과

## GitHub와 CI

- 비공개 저장소: `https://github.com/WBmaker2/shift-space-lang-change`
- `main`과 `feat/windows-hotkey-app` 브랜치를 push했다.
- 기능 브랜치 CI run `33411398369`: Ubuntu core와 Windows job 모두 통과했다.

## Package workflow 실행 조정

- `workflow_dispatch` 첫 시도는 package workflow 파일이 아직 기본 브랜치 `main`에 없어서 GitHub API 404로 중단되었다. 실행 자체가 생성되지 않았으며 저장소 내용은 바뀌지 않았다.
- HVC 전 `main` 조기 병합을 피하기 위해 기능 브랜치 HEAD에 release-candidate 태그 `v0.1.0-rc.1`을 사용한다.
- 태그의 `v*` push trigger로 Windows package workflow를 실행하고 `0.1.0-rc.1` 설치 파일을 HVC 대상으로 사용한다.
- 최종 `v0.1.0` 태그와 GitHub Release는 HVC 통과 뒤에만 만든다.

## RC.1 package 실패와 수정 계획

- package run `33411729703`은 Rust 테스트, clippy, release EXE 빌드까지 통과했지만 NSIS 단계에서 실패했다.
- 원인은 Chocolatey로 NSIS를 설치한 뒤 현재 PowerShell 세션의 `PATH`가 갱신되지 않아 `makensis` 명령을 찾지 못한 것이다.
- workflow에서 Chocolatey 기본 NSIS 설치 위치를 포함한 후보 경로를 탐색하고 `makensis.exe` 존재 여부를 검증한 뒤 절대 경로로 실행한다.
- 수정은 별도 구현·리뷰·검증을 거쳐 기능 브랜치에 push하고, 기존 RC.1 태그는 보존한 채 `v0.1.0-rc.2` 태그로 package workflow를 다시 실행한다.

## RC.1 package 수정 결과

- 수정 커밋: `e69e5c9843eac2e06c16e5086e225a39a37a32ae`
- `makensis.exe` 후보를 고정된 신뢰 경로에서 탐색하고, 파일 존재와 이름을 확인한 뒤 `GITHUB_ENV`로 절대 경로를 다음 단계에 전달한다.
- 별도 Luna 리뷰 결과: APPROVED. PowerShell 호출 연산자, 공백 포함 경로, 환경 전달, YAML 구조, 패키지 출력 회귀를 점검했다.
- RC.2 package workflow의 실제 Windows runner 성공 여부를 다음 게이트로 확인한다.

## 아직 미완료

- Windows package artifact 생성·다운로드·SHA-256 기록
- Windows 10/11 x64 한국어 Microsoft IME 실기기 HVC
- HVC 통과 후 `main` 병합, 최종 CI, `v0.1.0` tag/release
