# 한/영 전환 도우미 GitHub Pages 배포 계획

- 작성일: 2026-09-01
- 대상 브랜치: `codex/github-pages-site`
- 공개 경로: `https://wbmaker2.github.io/shift-space-lang-change/`

## 목표

기존 Sites 홍보 페이지와 동일한 핵심 정보·시각 방향을 유지하는 정적 홍보 페이지를 저장소에 추가하고 GitHub Pages로 공개한다. 모든 다운로드 버튼은 공개된 v0.1.0 설치기의 안정적인 `releases/latest/download` 주소를 사용한다.

## 구현 범위

1. `site/`에 서버 기능이 필요 없는 정적 HTML/CSS 홍보 페이지를 구성한다.
2. 제품 설명, 핵심 기능, 설치 방법, 사용 방법, 지원 범위, SmartScreen 안내, 업데이트 내역을 포함한다.
3. 기존에 생성한 제품 소개 이미지를 복사해 본문 및 Open Graph 공유 이미지로 사용한다.
4. GitHub Pages 하위 경로에서도 자산이 정상 로드되도록 상대 경로를 사용한다.
5. `.github/workflows/deploy-pages.yml`에서 `site/`를 Pages artifact로 업로드하고 배포한다.
6. 정적 파일·내부 링크·다운로드 주소를 검사하는 검증 스크립트를 추가한다.

## 배포 검증

- 정적 검증 스크립트 통과
- 공식 설치기 다운로드 응답과 SHA-256 릴리스 정보 일치
- GitHub Actions Pages workflow 성공
- 공개 주소의 제목, 본문, 이미지, 설치기 링크 응답 확인
- 375px 너비에서 가로 넘침 없이 핵심 다운로드 버튼 확인
- VoiceOver 검증은 범위에서 제외
