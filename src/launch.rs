/// The process startup mode selected by its optional command-line argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode {
    Foreground,
    Background,
    QuitExisting,
}

/// The registered class name used by the app's one settings window.
pub const WINDOW_CLASS: &str = "ShiftSpaceLangChange.MainWindow";

/// Private app message used by a second process to ask the primary process to exit.
pub const WM_APP_REQUEST_EXIT_ID: u32 = 0x8002;

/// Parse the app-specific arguments, ignoring the executable path and unknown options.
///
/// The caller should pass only arguments after `argv[0]`. The first recognized app mode wins;
/// no argument (or an unknown argument) keeps the normal foreground behavior.
pub fn parse_launch_mode<I, S>(args: I) -> LaunchMode
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    for argument in args {
        match argument.as_ref() {
            "--background" => return LaunchMode::Background,
            "--quit-existing" => return LaunchMode::QuitExisting,
            _ => {}
        }
    }

    LaunchMode::Foreground
}
