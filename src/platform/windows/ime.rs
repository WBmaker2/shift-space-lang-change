use std::mem::size_of;

use crate::ports::ImeSender;
use crate::windows_mapping::hangul_strokes;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY,
};

use super::error::Win32Error;

pub struct WinImeSender;

impl ImeSender for WinImeSender {
    type Error = Win32Error;

    fn send_toggle(&mut self) -> Result<(), Self::Error> {
        let inputs = hangul_strokes().map(|stroke| INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(stroke.virtual_key),
                    wScan: 0,
                    dwFlags: if stroke.key_up {
                        KEYEVENTF_KEYUP
                    } else {
                        Default::default()
                    },
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        });

        // Safety: inputs is a live, initialized array of two keyboard INPUT values; its pointer
        // and the advertised element size remain valid for the synchronous SendInput call.
        let sent = unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };
        if sent == inputs.len() as u32 {
            Ok(())
        } else {
            // Safety: GetLastError reads the calling thread's Win32 error slot and takes no
            // pointers; SendInput has just returned on this same thread.
            let code = unsafe { GetLastError().0 };
            Err(Win32Error::new(code))
        }
    }
}
