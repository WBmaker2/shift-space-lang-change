pub mod app;
pub mod error;
pub mod hotkeys;
pub mod ime;
pub mod registry;
pub mod settings;
pub mod single_instance;
pub mod startup;
mod timer;
pub mod ui;

pub use error::Win32Error;
pub use hotkeys::{WinHotkeyBackend, keys_are_released};
pub use ime::WinImeSender;
pub use settings::RegistrySettingsStore;
pub use single_instance::{
    AcquireResult, SingleInstanceGuard, request_existing_exit, show_existing_window,
};
pub use startup::WinStartupController;
