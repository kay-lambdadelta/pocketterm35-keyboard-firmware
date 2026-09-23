use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage, MediaKey, SystemControlKey};

use crate::{
    DeviceMode,
    keymap::{COLUMNS, FN_MAP, KEY_MAP, KEYBOARD_ROW0_MAP, KeyAction, ROWS, is_fn_active},
    matrix::KeyMatrix,
};

pub enum KeyEvent {
    Media(MediaKey),
    System(SystemControlKey),
    OneShot { modifier: u8, usage: KeyboardUsage },
    BacklightUp,
    BacklightDown,
    AdUp,
    AdDown,
}

#[allow(clippy::needless_range_loop)]
pub fn build_report(
    device_mode: DeviceMode,
    matrix: &KeyMatrix,
    mut on_event: impl FnMut(KeyEvent),
) -> KeyboardReport {
    let fn_active = is_fn_active(matrix.row(6));
    let map = if fn_active { &FN_MAP } else { &KEY_MAP };

    let mut modifier = 0;
    let mut keys = [0; 6];
    let mut key_count = 0;

    for row_index in 1..ROWS {
        for column_index in 0..COLUMNS {
            if !matrix.is_pressed(row_index, column_index) {
                continue;
            }
            let edge = matrix.is_edge(row_index, column_index);

            match map[row_index][column_index] {
                None | Some(KeyAction::Fn) => {}
                Some(KeyAction::Modifier(bit)) => modifier |= bit,
                Some(KeyAction::Key(usage)) => {
                    if key_count < keys.len() {
                        keys[key_count] = usage as u8;

                        key_count += 1;
                    }
                }
                Some(KeyAction::Media(media_key)) => {
                    if edge {
                        on_event(KeyEvent::Media(media_key));
                    }
                }

                Some(KeyAction::System(system_key)) => {
                    if edge {
                        on_event(KeyEvent::System(system_key));
                    }
                }
                Some(KeyAction::OneShot { modifier, usage }) => {
                    if edge {
                        on_event(KeyEvent::OneShot { modifier, usage });
                    }
                }
                Some(KeyAction::BacklightUp) => {
                    if edge {
                        on_event(KeyEvent::BacklightUp);
                    }
                }
                Some(KeyAction::BacklightDown) => {
                    if edge {
                        on_event(KeyEvent::BacklightDown);
                    }
                }
                Some(KeyAction::AdUp) => {
                    if edge {
                        on_event(KeyEvent::AdUp);
                    }
                }
                Some(KeyAction::AdDown) => {
                    if edge {
                        on_event(KeyEvent::AdDown);
                    }
                }
            }
        }
    }

    if device_mode == DeviceMode::Keyboard {
        for column_index in 0..COLUMNS {
            if matrix.row(0)[column_index]
                && let Some(usage) = KEYBOARD_ROW0_MAP[column_index]
                && key_count < keys.len()
            {
                keys[key_count] = usage as u8;

                key_count += 1;
            }
        }
    }

    KeyboardReport {
        modifier,
        reserved: 0,
        leds: 0,
        keycodes: keys,
    }
}
