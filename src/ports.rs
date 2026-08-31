use crate::config::AppSettings;

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

pub trait ImeSender {
    type Error: std::error::Error + Send + Sync + 'static;

    fn send_toggle(&mut self) -> Result<(), Self::Error>;
}
