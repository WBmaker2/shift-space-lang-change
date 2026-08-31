use shift_space_lang_change::config::Hotkey;
use shift_space_lang_change::windows_mapping::{hangul_strokes, hotkey_spec};

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
    assert_eq!(
        strokes,
        [
            shift_space_lang_change::windows_mapping::KeyStroke {
                virtual_key: 0x15,
                key_up: false,
            },
            shift_space_lang_change::windows_mapping::KeyStroke {
                virtual_key: 0x15,
                key_up: true,
            },
        ]
    );
}
