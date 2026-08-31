use std::mem::size_of;

use crate::ports::ImeSender;
use crate::windows_mapping::{SendInputDecision, hangul_strokes, send_input_decision};
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
        match send_input_decision(sent) {
            SendInputDecision::Complete => Ok(()),
            SendInputDecision::RecoverKeyUp => {
                let key_up = &inputs[1..2];
                // Safety: key_up is a valid one-element slice into inputs, and both its pointer
                // and element size remain valid for this synchronous recovery call.
                let recovered = unsafe { SendInput(key_up, size_of::<INPUT>() as i32) };
                if recovered == 1 {
                    // The initial count of one means the key-down was accepted; a successful
                    // recovery completes the pair, so this transaction is reported as success.
                    Ok(())
                } else {
                    // Safety: this immediately follows the failed recovery SendInput call on the
                    // same thread, so it captures that call's Win32 error slot.
                    Err(Win32Error::new(unsafe { GetLastError().0 }))
                }
            }
            SendInputDecision::Failed => {
                // Safety: this immediately follows the failed initial SendInput call on the same
                // thread and reads only the thread-local Win32 error slot.
                Err(Win32Error::new(unsafe { GetLastError().0 }))
            }
        }
    }
}
