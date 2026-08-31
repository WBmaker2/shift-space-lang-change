#!/usr/bin/env bash

set -euo pipefail

site_dir="${1:-site}"
installer_url="https://github.com/WBmaker2/shift-space-lang-change/releases/latest/download/ShiftSpaceLangChange-Setup-0.1.0-x64.exe"
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
  'SmartScreen'
  '업데이트 내역'
  '실제 Windows 10/11 기기 HVC는 진행 중'
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

if ! grep -Fq "$release_url" "$site_dir/index.html"; then
  echo "The public release URL is missing." >&2
  exit 1
fi

if grep -Eq '(href|src)="/' "$site_dir/index.html"; then
  echo "Root-relative assets break on the GitHub Pages project subpath." >&2
  exit 1
fi

large_files="$(find "$site_dir" -type f -size +10M -print)"
if [[ -n "$large_files" ]]; then
  echo "Pages contains files larger than 10 MB:" >&2
  echo "$large_files" >&2
  exit 1
fi

echo "GitHub Pages site verification passed."
