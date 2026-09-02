#!/usr/bin/env bash

set -euo pipefail

site_dir="${1:-site}"
installer_url="https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Setup-0.1.3-x64.exe"
portable_url="https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Portable-0.1.3-x64.zip"
release_url="https://github.com/WBmaker2/shift-space-lang-change/releases/latest"

required_files=(
  "$site_dir/index.html"
  "$site_dir/styles.css"
  "$site_dir/favicon.svg"
  "$site_dir/assets/hero.png"
)

for file in "${required_files[@]}"; do
  if [[ ! -s "$file" ]]; then
    echo "Missing or empty Pages file: $file" >&2
    exit 1
  fi
done

required_copy=(
  '<html lang="ko">'
  '한/영 전환 도우미'
  'Windows 10/11 x64'
  'Shift + Space'
  'Ctrl + Space'
  '설치형'
  '포터블형'
  '무설치'
  '압축 풀기'
  'README-PORTABLE.txt'
  'HKCU'
  'SmartScreen'
  'v0.1.3'
  '업데이트 내역'
  'v0.1.3의 실제 Windows 10/11 기기 HVC는 아직 미검증'
)

for text in "${required_copy[@]}"; do
  if ! grep -Fq "$text" "$site_dir/index.html"; then
    echo "Required page content is missing: $text" >&2
    exit 1
  fi
done

if ! grep -Fq "$installer_url" "$site_dir/index.html"; then
  echo "The public installer URL is missing." >&2
  exit 1
fi

if ! grep -Fq "$portable_url" "$site_dir/index.html"; then
  echo "The public portable URL is missing." >&2
  exit 1
fi

if ! grep -Fq "$release_url" "$site_dir/index.html"; then
  echo "The public release URL is missing." >&2
  exit 1
fi

if grep -Eq '(href|src)="/' "$site_dir/index.html"; then
  echo "Root-relative assets break on the GitHub Pages project subpath." >&2
  exit 1
fi

if ! grep -Fq 'gi-pulse' "$site_dir/index.html" || ! grep -Fq 'prefers-reduced-motion' "$site_dir/styles.css"; then
  echo "Download emphasis must include gi-pulse and reduced-motion support." >&2
  exit 1
fi

if grep -Fq '0.1.2' "$site_dir/index.html"; then
  echo "The public page still contains stale v0.1.2 copy." >&2
  exit 1
fi

large_files="$(find "$site_dir" -type f -size +10M -print)"
if [[ -n "$large_files" ]]; then
  echo "Pages contains files larger than 10 MB:" >&2
  echo "$large_files" >&2
  exit 1
fi

echo "GitHub Pages site verification passed."
