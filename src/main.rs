#[cfg(not(windows))]
fn main() {
    println!("이 프로그램은 Windows 10/11 전용입니다.");
}

#[cfg(windows)]
fn main() {}
