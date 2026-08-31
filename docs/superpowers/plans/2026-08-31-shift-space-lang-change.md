# 한/영 전환 도우미 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Windows 10/11 x64에서 `Shift + Space`와 `Ctrl + Space`를 전역 한/영 전환 키로 제공하는 저메모리 사용자 단위 트레이 프로그램과 NSIS 설치 파일을 만든다.

**Architecture:** 플랫폼 독립 Rust 도메인 계층이 설정 불변 조건, 단축키 등록 트랜잭션, 키 해제 대기 상태 머신을 담당한다. Windows 계층은 `windows` 크레이트로 Win32 단축키·입력·레지스트리·창·트레이·mutex API를 얇게 감싸며, 앱 조정 계층이 두 계층을 연결한다.

**Tech Stack:** Rust 1.97.1, Edition 2024, `windows` 0.62.2, Win32 API, NSIS, GitHub Actions `windows-2025`, `actions/checkout@v6`, `actions/upload-artifact@v7`

**Spec:** `docs/superpowers/specs/2026-08-31-shift-space-lang-change-design.md`

## Global Constraints

- 지원 대상은 한국어 Microsoft IME가 설치된 Windows 10/11 x64이다.
- WebView, Electron, .NET 런타임, 백그라운드 서비스, 네트워크 통신 및 분석 SDK를 사용하지 않는다.
- 프로그램은 관리자 권한을 요구하지 않으며 현재 사용자 범위에 설치한다.
- `Shift + Space`와 `Ctrl + Space`는 기본 활성화하고 최소 하나는 항상 활성화한다.
- 설정과 자동 실행 변경은 체크박스를 누르는 즉시 적용하고 실패하면 실제 상태로 롤백한다.
- 모든 Rust 소스 파일은 500줄 미만으로 유지한다.
- 코드 서명, 자동 업데이트, Windows ARM64·32비트 빌드는 범위에서 제외한다.
- VoiceOver, 음성 출력, 녹음 및 재생 기능을 구현하거나 검증하지 않는다.
- 구현 작업은 실패하는 테스트를 먼저 실행하는 RED → GREEN → REFACTOR 순서를 따른다.

## File Map

| 경로 | 책임 |
| --- | --- |
| `Cargo.toml` | 패키지, Windows 기능, 저용량 release 프로필 |
| `Cargo.lock` | 의존성 고정 |
| `rust-toolchain.toml` | Rust 1.97.1, rustfmt, clippy 고정 |
| `src/lib.rs` | 플랫폼 독립 공개 모듈 및 Windows 조건부 모듈 노출 |
| `src/main.rs` | GUI 하위 시스템 진입점과 CLI 실행 모드 전달 |
| `src/config.rs` | `AppSettings`, `Hotkey`, 최소 한 개 활성 규칙 |
| `src/toggle_state.rs` | 키 해제 대기·중복 억제·5초 타임아웃 상태 머신 |
| `src/hotkeys.rs` | `HotkeyBackend`와 등록 변경 트랜잭션 |
| `src/controller.rs` | 설정 저장, hotkey, IME, 자동 실행의 원자적 조정 |
| `src/persistence.rs` | 설정 codec, 저장소·자동 실행 trait, 시작 명령 생성 |
| `src/ports.rs` | `SettingsStore`, `StartupController`, `ImeSender` 공용 port trait |
| `src/launch.rs` | foreground·background·quit 실행 모드 parser |
| `src/windows_mapping.rs` | Win32 hotkey 숫자와 한/영 key stroke의 순수 매핑 |
| `src/ui_model.rs` | 플랫폼 독립 typed UI event와 command ID 매핑 |
| `src/platform/mod.rs` | Windows 플랫폼 모듈 조건부 노출 |
| `src/platform/windows/mod.rs` | Windows 구현 재노출 |
| `src/platform/windows/error.rs` | Win32 오류 코드와 사용자용 메시지 |
| `src/platform/windows/registry.rs` | HKCU DWORD·문자열 읽기·쓰기·삭제 원시 함수 |
| `src/platform/windows/settings.rs` | 앱 설정 레지스트리 저장소 |
| `src/platform/windows/startup.rs` | 현재 사용자 `Run` 자동 실행 제어 |
| `src/platform/windows/hotkeys.rs` | `RegisterHotKey` 기반 backend |
| `src/platform/windows/ime.rs` | `SendInput` 기반 `VK_HANGUL` sender |
| `src/platform/windows/single_instance.rs` | named mutex, 기존 창 표시, 종료 요청 |
| `src/platform/windows/ui/mod.rs` | UI 모듈 공개 인터페이스 |
| `src/platform/windows/ui/window.rs` | 설정 창과 네이티브 컨트롤 생성·상태 반영 |
| `src/platform/windows/ui/tray.rs` | 트레이 아이콘과 메뉴 생명주기 |
| `src/platform/windows/app.rs` | Win32 메시지 루프와 controller 조정 |
| `tests/config_model.rs` | 설정 불변 조건 테스트 |
| `tests/toggle_state.rs` | 입력 전환 상태 머신 테스트 |
| `tests/hotkey_manager.rs` | 등록 성공·충돌·롤백 테스트 |
| `tests/controller.rs` | 즉시 적용과 저장 실패 롤백 테스트 |
| `tests/persistence.rs` | 손상 설정 복구와 자동 실행 command 테스트 |
| `tests/launch.rs` | CLI 실행 모드 테스트 |
| `tests/windows_mapping.rs` | hotkey·VK_HANGUL 숫자 매핑 테스트 |
| `tests/ui_model.rs` | 체크박스·트레이 command 변환 테스트 |
| `tests/windows_registry.rs` | Windows 전용 HKCU 테스트 하위 키 통합 테스트 |
| `tests/windows_single_instance.rs` | Windows 전용 named mutex 통합 테스트 |
| `installer/ShiftSpaceLangChange.nsi` | 사용자 단위 설치·실행·제거 |
| `scripts/verify-package.ps1` | 릴리스 EXE와 installer 산출물 검증 |
| `.github/workflows/ci.yml` | 플랫폼 독립 테스트와 Windows 빌드 게이트 |
| `.github/workflows/windows-package.yml` | Windows release·NSIS artifact 생성 |
| `docs/HVC-WINDOWS.md` | Windows 설치·IME·트레이 수동 확인 기록지 |
| `README.md` | 설치, 사용, 제한, 로컬·CI 빌드 안내 |

---

### Task 1: Rust 프로젝트와 설정 도메인

**Files:**
- Create: `.gitignore`
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `src/lib.rs`
- Create: `src/main.rs`
- Create: `src/config.rs`
- Create: `tests/config_model.rs`
- Create: `Cargo.lock`

**Interfaces:**
- Consumes: 없음
- Produces: `Hotkey`, `AppSettings`, `SettingsError`, `AppSettings::new`, `AppSettings::with_hotkey`, `AppSettings::enabled_hotkeys`

- [ ] **Step 1: 테스트를 실행할 최소 프로젝트 설정과 실패 테스트 작성**

`Cargo.toml`의 패키지는 다음 값으로 시작한다.

```toml
[package]
name = "shift-space-lang-change"
version = "0.1.0"
edition = "2024"
rust-version = "1.97"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.62.2", features = [
  "Win32_Foundation",
  "Win32_Graphics_Gdi",
  "Win32_System_LibraryLoader",
  "Win32_System_Registry",
  "Win32_System_Threading",
  "Win32_UI_Input_KeyboardAndMouse",
  "Win32_UI_Shell",
  "Win32_UI_WindowsAndMessaging",
] }

[profile.release]
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

`rust-toolchain.toml`은 `1.97.1`, `minimal`, `rustfmt`, `clippy`를 고정한다. `.gitignore`에는 `/target`, `*.user`, `*.log`만 넣는다. `tests/config_model.rs`에는 다음 세 동작을 먼저 작성한다.

```rust
use shift_space_lang_change::config::{AppSettings, Hotkey, SettingsError};

#[test]
fn defaults_enable_both_hotkeys() {
    let settings = AppSettings::default();
    assert_eq!(settings.enabled_hotkeys(), vec![Hotkey::ShiftSpace, Hotkey::CtrlSpace]);
}

#[test]
fn either_single_hotkey_is_valid() {
    assert!(AppSettings::new(true, false).is_ok());
    assert!(AppSettings::new(false, true).is_ok());
}

#[test]
fn disabling_the_last_hotkey_is_rejected() {
    let settings = AppSettings::new(true, false).unwrap();
    assert_eq!(
        settings.with_hotkey(Hotkey::ShiftSpace, false),
        Err(SettingsError::NoHotkeyEnabled)
    );
}
```

- [ ] **Step 2: RED 확인**

Run: `cargo test --test config_model`

Expected: FAIL. `shift_space_lang_change` 라이브러리 또는 `config` 모듈을 찾을 수 없어야 한다.

- [ ] **Step 3: 설정 도메인의 최소 구현 작성**

`src/config.rs`에 다음 공개 타입과 메서드를 구현한다.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hotkey { ShiftSpace, CtrlSpace }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppSettings {
    pub shift_space_enabled: bool,
    pub ctrl_space_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsError { NoHotkeyEnabled }

impl Default for AppSettings {
    fn default() -> Self {
        Self { shift_space_enabled: true, ctrl_space_enabled: true }
    }
}

impl AppSettings {
    pub fn new(shift_space_enabled: bool, ctrl_space_enabled: bool) -> Result<Self, SettingsError>;
    pub fn with_hotkey(self, hotkey: Hotkey, enabled: bool) -> Result<Self, SettingsError>;
    pub fn enabled_hotkeys(self) -> Vec<Hotkey>;
    pub fn is_enabled(self, hotkey: Hotkey) -> bool;
}
```

`src/lib.rs`는 `pub mod config;`를 노출한다. `src/main.rs`는 Windows가 아닌 호스트에서 `이 프로그램은 Windows 10/11 전용입니다.`만 출력하고, Windows 진입점은 뒤 작업에서 연결할 수 있도록 조건부 `main`을 둔다.

- [ ] **Step 4: GREEN과 정적 검사 확인**

Run: `cargo test --test config_model && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings`

Expected: 3 tests PASS, rustfmt와 clippy exit 0.

- [ ] **Step 5: lockfile 생성과 커밋**

Run: `cargo generate-lockfile`

```bash
git add .gitignore Cargo.toml Cargo.lock rust-toolchain.toml src/lib.rs src/main.rs src/config.rs tests/config_model.rs
git commit -m "feat: add settings domain model"
```

### Task 2: 키 해제 대기 상태 머신

**Files:**
- Create: `src/toggle_state.rs`
- Modify: `src/lib.rs`
- Create: `tests/toggle_state.rs`

**Interfaces:**
- Consumes: `std::time::Duration`
- Produces: `ToggleState::request`, `ToggleState::poll`, `PollResult`, `TOGGLE_TIMEOUT`

- [ ] **Step 1: 중복 억제·키 해제·타임아웃 실패 테스트 작성**

```rust
use std::time::Duration;
use shift_space_lang_change::toggle_state::{PollResult, ToggleState};

#[test]
fn one_request_sends_once_after_keys_are_released() {
    let mut state = ToggleState::default();
    assert!(state.request(Duration::ZERO));
    assert!(!state.request(Duration::from_millis(10)));
    assert_eq!(state.poll(Duration::from_millis(20), false), PollResult::Waiting);
    assert_eq!(state.poll(Duration::from_millis(30), true), PollResult::SendToggle);
    assert_eq!(state.poll(Duration::from_millis(40), true), PollResult::Idle);
}

#[test]
fn request_times_out_after_five_seconds() {
    let mut state = ToggleState::default();
    state.request(Duration::ZERO);
    assert_eq!(state.poll(Duration::from_secs(5), false), PollResult::TimedOut);
    assert_eq!(state.poll(Duration::from_secs(6), true), PollResult::Idle);
}
```

- [ ] **Step 2: RED 확인**

Run: `cargo test --test toggle_state`

Expected: FAIL. `toggle_state` 모듈이 없어야 한다.

- [ ] **Step 3: 최소 상태 머신 구현**

```rust
pub const TOGGLE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollResult { Idle, Waiting, SendToggle, TimedOut }

#[derive(Debug, Default)]
pub struct ToggleState { requested_at: Option<Duration> }

impl ToggleState {
    pub fn request(&mut self, now: Duration) -> bool;
    pub fn poll(&mut self, now: Duration, keys_released: bool) -> PollResult;
    pub fn is_pending(&self) -> bool;
}
```

`poll`은 대기 중이 아니면 `Idle`, 경과 시간이 5초 이상이면 상태를 지우고 `TimedOut`, 키가 모두 놓였으면 상태를 지우고 `SendToggle`, 나머지는 `Waiting`을 반환한다.

- [ ] **Step 4: GREEN과 전체 회귀 확인**

Run: `cargo test --all-targets && cargo clippy --all-targets -- -D warnings`

Expected: 모든 테스트 PASS, clippy exit 0.

- [ ] **Step 5: 커밋**

```bash
git add src/lib.rs src/toggle_state.rs tests/toggle_state.rs
git commit -m "feat: add hotkey release state machine"
```

### Task 3: 단축키 등록 트랜잭션

**Files:**
- Create: `src/hotkeys.rs`
- Modify: `src/lib.rs`
- Create: `tests/hotkey_manager.rs`

**Interfaces:**
- Consumes: `AppSettings`, `Hotkey`
- Produces: `HotkeyBackend`, `HotkeyManager::new`, `HotkeyManager::apply`, `HotkeyManager::active_settings`, `ApplyError`

- [ ] **Step 1: fake backend로 등록 성공과 충돌 롤백 테스트 작성**

테스트 fake는 `BTreeSet<Hotkey>`와 `fail_register: Option<Hotkey>`를 가진다. 다음 핵심 테스트를 작성한다.

```rust
#[test]
fn failed_registration_keeps_previous_active_configuration() {
    let backend = FakeBackend::with_failure(Hotkey::CtrlSpace);
    let initial = AppSettings::new(true, false).unwrap();
    let mut manager = HotkeyManager::new(backend, initial).unwrap();

    let desired = AppSettings::default();
    let error = manager.apply(desired).unwrap_err();

    assert!(matches!(error, ApplyError::Register { hotkey: Hotkey::CtrlSpace, .. }));
    assert_eq!(manager.active_settings(), initial);
    assert_eq!(manager.backend().registered(), [Hotkey::ShiftSpace].into());
}

#[test]
fn successful_change_updates_active_configuration() {
    let backend = FakeBackend::default();
    let mut manager = HotkeyManager::new(backend, AppSettings::default()).unwrap();
    let desired = AppSettings::new(false, true).unwrap();
    manager.apply(desired).unwrap();
    assert_eq!(manager.active_settings(), desired);
}
```

등록 추가가 여러 개 중간에 실패하면 이번 호출에서 추가한 등록을 해제하는 테스트와, 해제 실패 시 제거했던 항목을 재등록하고 이전 설정을 유지하는 테스트도 각각 작성한다.

- [ ] **Step 2: RED 확인**

Run: `cargo test --test hotkey_manager`

Expected: FAIL. `hotkeys` 모듈과 타입을 찾을 수 없어야 한다.

- [ ] **Step 3: backend 인터페이스와 트랜잭션 구현**

```rust
pub trait HotkeyBackend {
    type Error: std::error::Error + Send + Sync + 'static;
    fn register(&mut self, hotkey: Hotkey) -> Result<(), Self::Error>;
    fn unregister(&mut self, hotkey: Hotkey) -> Result<(), Self::Error>;
}

pub struct HotkeyManager<B: HotkeyBackend> {
    backend: B,
    active: AppSettings,
}

pub enum ApplyError<E> {
    Register { hotkey: Hotkey, source: E },
    Unregister { hotkey: Hotkey, source: E },
    Rollback { operation: &'static str, hotkey: Hotkey, source: E },
}
```

`apply`은 먼저 추가할 단축키를 등록하고 그 다음 제거할 단축키를 해제한다. 어느 단계에서 실패해도 이번 호출에서 변경한 항목을 역순으로 복구하고 `active`를 바꾸지 않는다. 모든 Windows 호출이 성공한 뒤에만 `active = desired`를 실행한다.

- [ ] **Step 4: GREEN과 회귀 확인**

Run: `cargo test --all-targets && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings`

Expected: 모든 테스트 PASS.

- [ ] **Step 5: 커밋**

```bash
git add src/lib.rs src/hotkeys.rs tests/hotkey_manager.rs
git commit -m "feat: add transactional hotkey manager"
```

### Task 4: Windows 설정 저장소와 자동 실행

**Files:**
- Create: `src/persistence.rs`
- Create: `src/ports.rs`
- Create: `src/platform/mod.rs`
- Create: `src/platform/windows/mod.rs`
- Create: `src/platform/windows/error.rs`
- Create: `src/platform/windows/registry.rs`
- Create: `src/platform/windows/settings.rs`
- Create: `src/platform/windows/startup.rs`
- Modify: `src/lib.rs`
- Create: `tests/persistence.rs`
- Create: `tests/windows_registry.rs`

**Interfaces:**
- Consumes: `AppSettings`
- Produces: `SettingsStore`, `RegistrySettingsStore`, `StartupController`, `WinStartupController`, `startup_command`, `Win32Error`

- [ ] **Step 1: 플랫폼 독립 설정 복구와 시작 명령 테스트 작성**

`tests/persistence.rs`에서 Win32 호출과 분리된 codec와 시작 명령을 먼저 테스트한다.

```rust
#[test]
fn missing_or_invalid_values_recover_to_defaults() {
    assert_eq!(decode_settings(None, None).settings, AppSettings::default());
    assert!(decode_settings(None, None).needs_rewrite);
    assert_eq!(decode_settings(Some(7), Some(0)).settings, AppSettings::default());
    assert_eq!(decode_settings(Some(0), Some(0)).settings, AppSettings::default());
}

#[test]
fn startup_command_quotes_paths_with_spaces() {
    let path = Path::new(r"C:\Users\Kim User\App\shift-space-lang-change.exe");
    assert_eq!(startup_command(path), r#""C:\Users\Kim User\App\shift-space-lang-change.exe" --background"#);
}
```

`src/persistence.rs`의 공개 codec 타입은 다음과 같다.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedSettings {
    pub settings: AppSettings,
    pub needs_rewrite: bool,
}

pub fn decode_settings(shift: Option<u32>, control: Option<u32>) -> DecodedSettings;
pub fn startup_command(executable: &Path) -> String;
```

`tests/windows_registry.rs`는 `#![cfg(windows)]`로 제한하고, `HKCU\Software\ShiftSpaceLangChange\Tests\<process-id>`에 두 DWORD를 저장·재로드한 뒤 테스트 키를 삭제하는 통합 테스트를 작성한다. 실제 `Run` 키는 테스트에서 변경하지 않는다.

- [ ] **Step 2: RED 확인**

Run: `cargo test --test persistence`

Expected: FAIL. `persistence` 모듈이 없어야 한다.

- [ ] **Step 3: registry와 저장소 최소 구현**

```rust
pub trait SettingsStore {
    type Error: std::error::Error + Send + Sync + 'static;
    fn load(&self) -> Result<AppSettings, Self::Error>;
    fn save(&self, settings: AppSettings) -> Result<(), Self::Error>;
}

pub trait StartupController {
    type Error: std::error::Error + Send + Sync + 'static;
    fn is_enabled(&self) -> Result<bool, Self::Error>;
    fn set_enabled(&self, enabled: bool) -> Result<(), Self::Error>;
}
```

두 저장소 trait와 다음 `ImeSender` trait는 `src/ports.rs`에 둔다. `DecodedSettings`, `decode_settings`, `startup_command`는 `src/persistence.rs`에 둔다.

```rust
pub trait ImeSender {
    type Error: std::error::Error + Send + Sync + 'static;
    fn send_toggle(&mut self) -> Result<(), Self::Error>;
}
```

`registry.rs`는 `RegCreateKeyExW`, `RegGetValueW`, `RegSetValueExW`, `RegDeleteValueW`, `RegDeleteTreeW`, `RegCloseKey`를 사용하며 RAII key handle로 닫는다. `RegistrySettingsStore::new()`는 제품 경로를 사용하고, Windows 통합 테스트용 `RegistrySettingsStore::with_subkey(String)`은 전달받은 HKCU 하위 경로만 사용한다. `RegistrySettingsStore::load`는 codec가 `needs_rewrite`를 반환하면 기본값을 저장한 뒤 반환한다. `WinStartupController`는 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`의 `ShiftSpaceLangChange` 문자열 하나만 관리한다.

- [ ] **Step 4: macOS 검사와 Windows target type-check**

Run: `rustup target add x86_64-pc-windows-msvc`

Run: `cargo test --all-targets && cargo check --target x86_64-pc-windows-msvc --all-targets`

Expected: macOS 테스트 PASS, Windows target check exit 0. Windows 통합 테스트 실행은 Task 9의 Windows CI에서 확인한다.

- [ ] **Step 5: 커밋**

```bash
git add src/lib.rs src/persistence.rs src/ports.rs src/platform tests/persistence.rs tests/windows_registry.rs
git commit -m "feat: persist settings and startup state"
```

### Task 5: Windows 전역 단축키와 IME 입력 adapter

**Files:**
- Create: `src/windows_mapping.rs`
- Modify: `src/lib.rs`
- Create: `src/platform/windows/hotkeys.rs`
- Create: `src/platform/windows/ime.rs`
- Modify: `src/platform/windows/mod.rs`
- Create: `tests/windows_mapping.rs`

**Interfaces:**
- Consumes: `HotkeyBackend`, `Hotkey`
- Produces: `HotkeySpec`, `KeyStroke`, `WinHotkeyBackend::new(HWND)`, `WinImeSender`, `keys_are_released`

- [ ] **Step 1: Win32 값 매핑과 한/영 입력 배열 실패 테스트 작성**

`tests/windows_mapping.rs`에 다음 테스트를 작성한다. 숫자 상수는 Microsoft Win32 정의와 일치해야 한다.

```rust
#[test]
fn hotkey_specs_match_the_approved_combinations() {
    let shift = hotkey_spec(Hotkey::ShiftSpace);
    assert_eq!(shift.id, 0x5101);
    assert_eq!(shift.modifiers, 0x0004 | 0x4000);
    assert_eq!(shift.virtual_key, 0x20);

    let control = hotkey_spec(Hotkey::CtrlSpace);
    assert_eq!(control.id, 0x5102);
    assert_eq!(control.modifiers, 0x0002 | 0x4000);
}

#[test]
fn hangul_sequence_contains_one_down_and_one_up_event() {
    let strokes = hangul_strokes();
    assert_eq!(strokes, [
        KeyStroke { virtual_key: 0x15, key_up: false },
        KeyStroke { virtual_key: 0x15, key_up: true },
    ]);
}
```

- [ ] **Step 2: RED 확인**

Run: `cargo test --test windows_mapping`

Expected: FAIL. `windows_mapping` 모듈이 없어야 한다.

- [ ] **Step 3: Win32 adapter 구현**

`src/windows_mapping.rs`에 `HotkeySpec`, `KeyStroke`, `hotkey_spec`, `hangul_strokes`를 구현한다. `WinHotkeyBackend`는 이 순수 매핑을 사용해 `RegisterHotKey(hwnd, id, modifiers, virtual_key)`를 호출하고, `UnregisterHotKey(hwnd, id)`와 함께 0 반환을 `Win32Error`로 바꾼다. `keys_are_released`는 `VK_SHIFT`, `VK_CONTROL`, `VK_SPACE`의 `GetAsyncKeyState` 상위 비트를 모두 검사한다.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotkeySpec { pub id: i32, pub modifiers: u32, pub virtual_key: u32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyStroke { pub virtual_key: u16, pub key_up: bool }

pub fn hotkey_spec(hotkey: Hotkey) -> HotkeySpec;
pub fn hangul_strokes() -> [KeyStroke; 2];
```

```rust
pub struct WinImeSender;

impl ImeSender for WinImeSender {
    type Error = Win32Error;
    fn send_toggle(&mut self) -> Result<(), Self::Error>;
}
```

`send_toggle`은 `VK_HANGUL` key-down과 `KEYEVENTF_KEYUP` 두 `INPUT`을 한 번의 `SendInput` 호출로 전달하고 반환 개수가 2가 아니면 오류로 처리한다. 테스트에서는 실제 입력을 주입하지 않는다.

- [ ] **Step 4: Windows compile gate 확인**

Run: `cargo check --target x86_64-pc-windows-msvc --all-targets && cargo test --all-targets`

Expected: Windows type-check와 macOS 테스트 모두 exit 0.

- [ ] **Step 5: 커밋**

```bash
git add src/lib.rs src/windows_mapping.rs src/platform/windows/hotkeys.rs src/platform/windows/ime.rs src/platform/windows/mod.rs tests/windows_mapping.rs
git commit -m "feat: add Windows hotkey and IME adapters"
```

### Task 6: 실행 모드와 단일 인스턴스

**Files:**
- Create: `src/launch.rs`
- Modify: `src/lib.rs`
- Create: `src/platform/windows/single_instance.rs`
- Modify: `src/platform/windows/mod.rs`
- Modify: `src/main.rs`
- Create: `tests/launch.rs`
- Create: `tests/windows_single_instance.rs`

**Interfaces:**
- Consumes: Win32 named mutex와 window messaging
- Produces: `WINDOW_CLASS`, `WM_APP_REQUEST_EXIT_ID`, `LaunchMode`, `parse_launch_mode`, `SingleInstanceGuard::acquire`, `show_existing_window`, `request_existing_exit`

- [ ] **Step 1: CLI 모드와 mutex 실패 테스트 작성**

`tests/launch.rs`에 다음 테스트를 작성한다.

```rust
#[test]
fn launch_modes_are_explicit() {
    assert_eq!(parse_launch_mode(Vec::<String>::new()), LaunchMode::Foreground);
    assert_eq!(parse_launch_mode(["--background"]), LaunchMode::Background);
    assert_eq!(parse_launch_mode(["--quit-existing"]), LaunchMode::QuitExisting);
}
```

Windows 통합 테스트는 고유한 `Local\ShiftSpaceLangChange.Test.<pid>` 이름으로 첫 guard가 `Primary`, 두 번째 획득이 `AlreadyRunning`을 반환하는지 확인한다.

- [ ] **Step 2: RED 확인**

Run: `cargo test launch_modes_are_explicit`

Expected: FAIL. `launch` 모듈이 없어야 한다.

- [ ] **Step 3: parser와 Win32 single-instance 구현**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode { Foreground, Background, QuitExisting }

pub const WINDOW_CLASS: &str = "ShiftSpaceLangChange.MainWindow";
pub const WM_APP_REQUEST_EXIT_ID: u32 = 0x8002;

pub enum AcquireResult {
    Primary(SingleInstanceGuard),
    AlreadyRunning,
}

impl SingleInstanceGuard {
    pub fn acquire() -> Result<AcquireResult, Win32Error>;
    pub fn acquire_named(name: &str) -> Result<AcquireResult, Win32Error>;
}
```

`LaunchMode`, `WINDOW_CLASS`, `WM_APP_REQUEST_EXIT_ID`는 `src/launch.rs`에 둔다. `CreateMutexW` 이름은 `Local\ShiftSpaceLangChange.SingleInstance`로 고정하고 `ERROR_ALREADY_EXISTS`를 분기한다. guard의 `Drop`은 `CloseHandle`을 한 번 호출한다. 기존 창 표시는 `FindWindowW`, `ShowWindow(SW_RESTORE)`, `SetForegroundWindow` 순서로 처리한다. 종료 요청은 `WM_APP_REQUEST_EXIT_ID`를 `PostMessageW`로 전달한다.

- [ ] **Step 4: Windows check와 macOS 회귀 확인**

Run: `cargo check --target x86_64-pc-windows-msvc --all-targets && cargo test --all-targets`

Expected: exit 0.

- [ ] **Step 5: 커밋**

```bash
git add src/lib.rs src/launch.rs src/main.rs src/platform/windows/mod.rs src/platform/windows/single_instance.rs tests/launch.rs tests/windows_single_instance.rs
git commit -m "feat: prevent duplicate app instances"
```

### Task 7: 네이티브 설정 창과 시스템 트레이

**Files:**
- Create: `src/ui_model.rs`
- Modify: `src/lib.rs`
- Create: `src/platform/windows/ui/mod.rs`
- Create: `src/platform/windows/ui/window.rs`
- Create: `src/platform/windows/ui/tray.rs`
- Modify: `src/platform/windows/mod.rs`
- Create: `tests/ui_model.rs`

**Interfaces:**
- Consumes: `AppSettings`, 자동 실행 bool, Win32 `WNDPROC`
- Produces: `UiHandles`, `UiEvent`, `create_settings_window`, `read_ui_event`, `render_state`, `TrayIcon`

- [ ] **Step 1: 명령 ID를 의미 있는 UI event로 바꾸는 실패 테스트 작성**

`tests/ui_model.rs`에 다음 테스트를 작성한다.

```rust
#[test]
fn checkbox_commands_map_to_typed_events() {
    assert_eq!(map_command(IDC_SHIFT_SPACE, true), Some(UiEvent::SetHotkey(Hotkey::ShiftSpace, true)));
    assert_eq!(map_command(IDC_CTRL_SPACE, false), Some(UiEvent::SetHotkey(Hotkey::CtrlSpace, false)));
    assert_eq!(map_command(IDC_STARTUP, true), Some(UiEvent::SetStartup(true)));
    assert_eq!(map_command(IDC_HIDE, false), Some(UiEvent::Hide));
}
```

트레이 메뉴 ID는 `설정 열기` → `UiEvent::Show`, `종료` → `UiEvent::Exit`로 매핑하는 테스트를 추가한다.

- [ ] **Step 2: RED 확인**

Run: `cargo test --test ui_model`

Expected: FAIL. `ui_model` 모듈이 없어야 한다.

- [ ] **Step 3: 설정 창 생성과 상태 렌더링 구현**

`src/ui_model.rs`는 플랫폼 독립 ID와 typed event를 한 곳에 정의한다.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEvent {
    SetHotkey(Hotkey, bool),
    SetStartup(bool),
    Hide,
    Show,
    Exit,
}

pub const IDC_SHIFT_SPACE: i32 = 1001;
pub const IDC_CTRL_SPACE: i32 = 1002;
pub const IDC_STARTUP: i32 = 1003;
pub const IDC_HIDE: i32 = 1004;
pub const IDC_STATUS: i32 = 1005;
pub const IDM_SHOW: usize = 2001;
pub const IDM_EXIT: usize = 2002;

pub fn map_command(id: i32, checked: bool) -> Option<UiEvent>;
pub fn map_tray_command(id: usize) -> Option<UiEvent>;
```

`window.rs`는 `launch::WINDOW_CLASS`와 `launch::WM_APP_REQUEST_EXIT_ID`를 재사용하고 `WM_APP_TRAY = 0x8001`을 정의한다. `create_settings_window`는 420×300 논클라이언트 기준 창에 상태 라벨, 연결된 텍스트를 가진 단축키 체크박스 2개, 자동 실행 체크박스, `트레이로 숨기기` 버튼을 표준 Win32 컨트롤로 만든다. 세 체크박스와 버튼에는 `WS_TABSTOP`을 지정해 Tab 이동과 Space 활성화를 Windows 기본 동작으로 제공한다. `render_state`는 `BM_SETCHECK`과 `SetWindowTextW`로 실제 controller 상태를 반영한다. `WM_CLOSE`와 Escape는 `ShowWindow(SW_HIDE)`로 처리하도록 typed event를 반환한다.

- [ ] **Step 4: 트레이 아이콘 RAII 구현**

`TrayIcon::install(hwnd)`은 `NOTIFYICONDATAW`와 `NIM_ADD`, `NIM_SETVERSION`을 사용하고 Windows 기본 `IDI_APPLICATION` 아이콘을 로드한다. 우클릭 시 `설정 열기`, 비활성 상태 요약 행, `종료` 메뉴를 `TrackPopupMenu`로 표시한다. `Drop`은 `NIM_DELETE`를 호출한다. 두 번 클릭은 `UiEvent::Show`로 변환한다.

- [ ] **Step 5: Windows check와 매핑 테스트 확인**

Run: `cargo check --target x86_64-pc-windows-msvc --all-targets && cargo test --all-targets`

Expected: exit 0. 각 Rust 파일이 500줄 미만인지 `find src -name '*.rs' -print0 | xargs -0 wc -l`로 확인한다.

- [ ] **Step 6: 커밋**

```bash
git add src/lib.rs src/ui_model.rs src/platform/windows/mod.rs src/platform/windows/ui tests/ui_model.rs
git commit -m "feat: add native settings window and tray"
```

### Task 8: 앱 controller와 Windows 메시지 루프

**Files:**
- Create: `src/controller.rs`
- Modify: `src/lib.rs`
- Create: `tests/controller.rs`
- Create: `src/platform/windows/app.rs`
- Modify: `src/platform/windows/mod.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: `HotkeyManager<B>`, `SettingsStore`, `StartupController`, `ImeSender`, `ToggleState`, UI typed events
- Produces: `AppController`, `ControllerEvent`, `platform::windows::app::run(LaunchMode)`

- [ ] **Step 1: 즉시 적용과 실패 롤백 controller 테스트 작성**

fake backend, store, startup, IME sender를 사용해 다음을 검증한다.

```rust
#[test]
fn successful_hotkey_change_is_registered_then_saved() {
    let mut controller = fixture(AppSettings::default());
    let actual = controller.set_hotkey(Hotkey::ShiftSpace, false).unwrap();
    assert_eq!(actual, AppSettings::new(false, true).unwrap());
    assert_eq!(controller.saved_settings(), actual);
}

#[test]
fn save_failure_restores_previous_hotkeys_and_ui_state() {
    let mut controller = fixture_with_save_failure(AppSettings::default());
    assert!(controller.set_hotkey(Hotkey::ShiftSpace, false).is_err());
    assert_eq!(controller.settings(), AppSettings::default());
    assert_eq!(controller.registered_hotkeys(), AppSettings::default());
}

#[test]
fn released_keys_send_exactly_one_ime_toggle() {
    let mut controller = fixture(AppSettings::default());
    controller.on_hotkey(Duration::ZERO);
    assert_eq!(controller.poll_toggle(Duration::from_millis(20), false).unwrap(), ControllerEvent::Waiting);
    assert_eq!(controller.poll_toggle(Duration::from_millis(30), true).unwrap(), ControllerEvent::ToggleSent);
    assert_eq!(controller.ime_send_count(), 1);
}
```

자동 실행 저장 실패 시 `ControllerError::Startup`을 반환하고 `startup_enabled()`가 다시 읽은 실제 상태를 유지하는 테스트와 5초 타임아웃이 `ControllerEvent::ToggleTimedOut`을 반환하는 테스트도 작성한다.

- [ ] **Step 2: RED 확인**

Run: `cargo test --test controller`

Expected: FAIL. `controller` 모듈이 없어야 한다.

- [ ] **Step 3: 플랫폼 독립 controller 구현**

```rust
pub struct AppController<B, S, A, I>
where
    B: HotkeyBackend,
    S: SettingsStore,
    A: StartupController,
    I: ImeSender;

impl<B, S, A, I> AppController<B, S, A, I>
where
    B: HotkeyBackend,
    S: SettingsStore,
    A: StartupController,
    I: ImeSender,
{
    pub fn new(
        hotkeys: HotkeyManager<B>,
        store: S,
        startup: A,
        ime: I,
        startup_enabled: bool,
    ) -> Self;
    pub fn set_hotkey(&mut self, hotkey: Hotkey, enabled: bool) -> Result<AppSettings, ControllerError>;
    pub fn set_startup(&mut self, enabled: bool) -> Result<bool, ControllerError>;
    pub fn on_hotkey(&mut self, now: Duration) -> bool;
    pub fn poll_toggle(&mut self, now: Duration, keys_released: bool) -> Result<ControllerEvent, ControllerError>;
    pub fn settings(&self) -> AppSettings;
    pub fn startup_enabled(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerEvent { Idle, Waiting, ToggleSent, ToggleTimedOut }

#[derive(Debug, PartialEq, Eq)]
pub enum ControllerError {
    InvalidSettings(SettingsError),
    HotkeyApply(String),
    SettingsSave(String),
    SettingsRollback { save: String, rollback: String },
    Startup(String),
    Ime(String),
}
```

`set_hotkey` 순서는 도메인 검증 → hotkey manager 적용 → settings 저장이다. 저장 실패 시 이전 settings를 hotkey manager에 다시 적용한다. `set_startup`은 변경 전 실제 상태를 읽고 쓰기 실패 시 다시 읽은 실제 상태를 반환한다.

- [ ] **Step 4: Windows runtime와 메시지 루프 구현**

`run`은 다음 순서를 지킨다.

1. `LaunchMode::QuitExisting`이면 기존 창에 `WM_APP_REQUEST_EXIT`를 보내고 종료한다.
2. mutex를 획득하고 중복이면 기존 설정 창을 표시한 뒤 종료한다.
3. 설정 창과 컨트롤을 생성한다.
4. 레지스트리 설정을 읽고 `HotkeyManager<WinHotkeyBackend>`를 초기화한다. 기본 두 단축키 중 하나가 선점되면 새 backend로 나머지 한 단축키만 등록해 유효 상태를 유지하고 설정을 그 실제 상태로 저장하며 충돌 경고를 표시한다. 두 단축키가 모두 선점되면 한국어 오류 message box를 표시하고 exit code 1로 종료한다.
5. `WinStartupController::is_enabled()`로 실제 Run 상태를 읽고 `WinImeSender`, `TrayIcon`, `AppController`와 함께 구성한다.
6. foreground는 창을 표시하고 background는 숨긴다.
7. `WM_COMMAND`, `WM_HOTKEY`, 15ms `WM_TIMER`, tray message, `WM_CLOSE`, `WM_APP_REQUEST_EXIT`, `WM_DESTROY`를 처리한다.
8. controller 오류마다 `render_state`로 체크를 실제 값으로 되돌리고 한국어 상태 문구와 트레이 알림을 표시한다.
9. 종료 시 timer, hotkey 등록, tray icon, 창, mutex가 RAII 또는 명시적 종료 순서로 정리된다.

`src/main.rs` 첫 줄에 `#![cfg_attr(windows, windows_subsystem = "windows")]`를 두어 release 실행 시 콘솔 창을 만들지 않는다. Windows 진입점은 `parse_launch_mode(std::env::args().skip(1))`를 `run`에 넘기고, 최상위 오류는 네이티브 message box로 표시한 뒤 exit code 1을 반환한다.

- [ ] **Step 5: 전체 로컬 테스트와 Windows type-check**

Run: `cargo test --all-targets && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo check --target x86_64-pc-windows-msvc --all-targets`

Expected: 모두 exit 0.

- [ ] **Step 6: 커밋**

```bash
git add src/lib.rs src/main.rs src/controller.rs src/platform/windows/app.rs src/platform/windows/mod.rs tests/controller.rs
git commit -m "feat: wire the Windows tray application"
```

### Task 9: NSIS 설치, CI, HVC 문서와 최종 검증

**Files:**
- Create: `installer/ShiftSpaceLangChange.nsi`
- Create: `scripts/verify-package.ps1`
- Create: `.github/workflows/ci.yml`
- Create: `.github/workflows/windows-package.yml`
- Create: `docs/HVC-WINDOWS.md`
- Create: `README.md`

**Interfaces:**
- Consumes: `target/release/shift-space-lang-change.exe`, `--background`, `--quit-existing`
- Produces: `dist/ShiftSpaceLangChange-Setup-<version>-x64.exe`, CI artifact `shift-space-lang-change-windows-x64`

- [ ] **Step 1: package 검증 스크립트를 먼저 작성하고 RED 확인**

`scripts/verify-package.ps1`은 버전 인수를 받아 다음 경로를 검사한다.

```powershell
param([Parameter(Mandatory=$true)][string]$Version)
$ErrorActionPreference = 'Stop'
$binary = 'target\release\shift-space-lang-change.exe'
$installer = "dist\ShiftSpaceLangChange-Setup-$Version-x64.exe"
if (-not (Test-Path $binary)) { throw "Missing release binary: $binary" }
if (-not (Test-Path $installer)) { throw "Missing installer: $installer" }
if ((Get-Item $binary).Length -le 0) { throw 'Release binary is empty' }
if ((Get-Item $installer).Length -le 0) { throw 'Installer is empty' }
Write-Host "Verified $binary and $installer"
```

Run on Windows: `pwsh -File scripts/verify-package.ps1 -Version 0.1.0`

Expected: FAIL with `Missing release binary` or `Missing installer` before the release build exists.

- [ ] **Step 2: 사용자 단위 NSIS installer 구현**

NSIS script는 `RequestExecutionLevel user`, `SetShellVarContext current`, `InstallDir "$LOCALAPPDATA\Programs\ShiftSpaceLangChange"`를 사용한다. 설치 section은 release EXE, 시작 메뉴 바로가기, uninstall 정보, 다음 자동 실행 값을 작성한다.

```nsis
Unicode true
RequestExecutionLevel user
SetShellVarContext current
!ifndef VERSION
  !define VERSION "0.1.0"
!endif
Name "한/영 전환 도우미"
OutFile "..\dist\ShiftSpaceLangChange-Setup-${VERSION}-x64.exe"
InstallDir "$LOCALAPPDATA\Programs\ShiftSpaceLangChange"

Section "Install"
SetOutPath "$INSTDIR"
File "..\target\release\shift-space-lang-change.exe"
WriteUninstaller "$INSTDIR\Uninstall.exe"
CreateDirectory "$SMPROGRAMS\한영 전환 도우미"
CreateShortCut "$SMPROGRAMS\한영 전환 도우미\한영 전환 도우미.lnk" \
  "$INSTDIR\shift-space-lang-change.exe"
WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" \
  "ShiftSpaceLangChange" '"$INSTDIR\shift-space-lang-change.exe" --background'
Exec '"$INSTDIR\shift-space-lang-change.exe"'
SectionEnd
```

uninstall section은 다음 순서를 사용한다. 출력 파일 이름은 `dist\ShiftSpaceLangChange-Setup-${VERSION}-x64.exe`로 고정한다.

```nsis
Section "Uninstall"
ExecWait '"$INSTDIR\shift-space-lang-change.exe" --quit-existing'
DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" \
  "ShiftSpaceLangChange"
DeleteRegKey HKCU "Software\ShiftSpaceLangChange"
DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\ShiftSpaceLangChange"
Delete "$SMPROGRAMS\한영 전환 도우미\한영 전환 도우미.lnk"
RMDir "$SMPROGRAMS\한영 전환 도우미"
Delete "$INSTDIR\shift-space-lang-change.exe"
Delete "$INSTDIR\Uninstall.exe"
RMDir "$INSTDIR"
SectionEnd
```

- [ ] **Step 3: CI workflow 작성**

`.github/workflows/ci.yml`은 다음 두 job을 사용한다.

```yaml
name: CI
on:
  push:
  pull_request:
permissions:
  contents: read
jobs:
  core:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v6
      - run: rustup toolchain install 1.97.1 --profile minimal --component rustfmt,clippy
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test --all-targets
  windows:
    runs-on: windows-2025
    steps:
      - uses: actions/checkout@v6
      - run: rustup toolchain install 1.97.1 --profile minimal --component rustfmt,clippy
      - run: cargo test --all-targets
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo build --release
```

`.github/workflows/windows-package.yml`은 다음 핵심 단계를 그대로 포함한다.

```yaml
name: Windows package
on:
  workflow_dispatch:
  push:
    tags: ['v*']
permissions:
  contents: read
jobs:
  package:
    runs-on: windows-2025
    steps:
      - uses: actions/checkout@v6
      - run: rustup toolchain install 1.97.1 --profile minimal --component rustfmt,clippy
      - run: cargo test --all-targets
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo build --release
      - name: Resolve version
        shell: pwsh
        run: |
          if ('${{ github.ref_type }}' -eq 'tag') {
            $version = '${{ github.ref_name }}'.TrimStart('v')
          } else {
            $version = '0.1.0'
          }
          "VERSION=$version" >> $env:GITHUB_ENV
      - run: choco install nsis --no-progress -y
      - run: New-Item -ItemType Directory -Force dist
        shell: pwsh
      - run: makensis "/DVERSION=$env:VERSION" installer\ShiftSpaceLangChange.nsi
        shell: pwsh
      - run: pwsh -File scripts\verify-package.ps1 -Version $env:VERSION
      - uses: actions/upload-artifact@v7
        with:
          name: shift-space-lang-change-windows-x64
          path: |
            target/release/shift-space-lang-change.exe
            dist/ShiftSpaceLangChange-Setup-${{ env.VERSION }}-x64.exe
```

- [ ] **Step 4: README와 Windows HVC 기록지 작성**

`README.md`에는 지원 환경, 설치, 두 체크박스, 트레이 종료, 자동 실행, 한국어 IME 필요 조건, 관리자 권한 앱에서 UIPI로 전환이 제한될 수 있음, unsigned SmartScreen 경고, 로컬 테스트 및 Windows package workflow 실행법을 적는다.

`docs/HVC-WINDOWS.md`에는 다음 각 항목마다 `결과: 미검증 / 통과 / 실패`, Windows 버전, 앱 버전, 확인일, 증거 링크를 기록할 칸을 둔다.

- 관리자 권한 없는 설치
- 메모장·브라우저·Office의 두 단축키
- 단축키 하나만 활성화
- 마지막 단축키 비활성 거부
- 창 닫기 후 트레이 상주
- 로그인 후 background 자동 실행
- 자동 실행 해제
- 중복 실행 방지
- 단축키 충돌 롤백
- 제거 후 파일·바로가기·Run·설정 삭제
- 작업 관리자 유휴 메모리

- [ ] **Step 5: 전체 검증 실행**

Run locally:

```bash
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo check --target x86_64-pc-windows-msvc --all-targets
find src -name '*.rs' -print0 | xargs -0 wc -l
```

Expected: 모든 명령 exit 0, 모든 Rust 파일 500줄 미만.

Run on Windows CI:

```powershell
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release
makensis /DVERSION=0.1.0 installer\ShiftSpaceLangChange.nsi
pwsh -File scripts/verify-package.ps1 -Version 0.1.0
```

Expected: tests 0 failures, clippy 0 warnings, release EXE와 `dist\ShiftSpaceLangChange-Setup-0.1.0-x64.exe` 존재.

- [ ] **Step 6: 요구사항 대조와 최종 커밋**

설계 문서 2·7·8·9·10·11·12·13·14절을 한 줄씩 대조하고, 자동 검증할 수 없는 항목은 `docs/HVC-WINDOWS.md`에서 `미검증`으로 정확히 남긴다. 로컬 macOS 결과를 Windows 설치·IME HVC 결과로 표현하지 않는다.

```bash
git add installer scripts .github README.md docs/HVC-WINDOWS.md
git commit -m "build: package and verify the Windows app"
```

## Final Review Gate

모든 Task 커밋 후 계획 시작 커밋부터 HEAD까지 최종 code review를 수행한다. Critical·Important 지적은 같은 구현 담당자에게 돌려보내 수정하고, 해당 지적 범위만 재검토한다. 그 다음 전체 테스트·format·clippy·Windows target check를 새로 실행하고 실제 결과만 보고한다. 원격 GitHub 저장소가 연결되어 있지 않으면 artifact 다운로드 링크가 아직 없다는 사실과 Windows HVC 대기 상태를 명시한다.
