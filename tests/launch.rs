use shift_space_lang_change::launch::{LaunchMode, parse_launch_mode};

#[test]
fn launch_modes_are_explicit() {
    assert_eq!(
        parse_launch_mode(Vec::<String>::new()),
        LaunchMode::Foreground
    );
    assert_eq!(parse_launch_mode(["--background"]), LaunchMode::Background);
    assert_eq!(
        parse_launch_mode(["--quit-existing"]),
        LaunchMode::QuitExisting
    );
}
