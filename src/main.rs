#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(not(windows))]
fn main() {
    println!("이 프로그램은 Windows 10/11 전용입니다.");
}

#[cfg(windows)]
fn main() {
    use shift_space_lang_change::launch::parse_launch_mode;
    use shift_space_lang_change::platform::windows::app::{run, show_error_message};

    let mode = parse_launch_mode(std::env::args().skip(1));
    match run(mode) {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            show_error_message("한/영 전환 도우미", &error.to_string());
            std::process::exit(1);
        }
    }
}
