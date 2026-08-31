use crate::config::Hotkey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotkeySpec {
    pub id: i32,
    pub modifiers: u32,
    pub virtual_key: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyStroke {
    pub virtual_key: u16,
    pub key_up: bool,
}

pub fn hotkey_spec(hotkey: Hotkey) -> HotkeySpec {
    match hotkey {
        Hotkey::ShiftSpace => HotkeySpec {
            id: 0x5101,
            modifiers: 0x0004 | 0x4000,
            virtual_key: 0x20,
        },
        Hotkey::CtrlSpace => HotkeySpec {
            id: 0x5102,
            modifiers: 0x0002 | 0x4000,
            virtual_key: 0x20,
        },
    }
}

pub fn hangul_strokes() -> [KeyStroke; 2] {
    [
        KeyStroke {
            virtual_key: 0x15,
            key_up: false,
        },
        KeyStroke {
            virtual_key: 0x15,
            key_up: true,
        },
    ]
}
