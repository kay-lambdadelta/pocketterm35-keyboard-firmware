use crate::{
    hid::{GamepadButton, GamepadReport, Hat},
    keymap::{COLUMNS, is_fn_active, row0_column, row6_column},
};

pub fn build_report(row0: &[bool; COLUMNS], row6: &[bool; COLUMNS]) -> GamepadReport {
    let up = row0[row0_column::UP];
    let left = row0[row0_column::LEFT];
    let down = row0[row0_column::DOWN];
    let right = row0[row0_column::RIGHT];

    let hat = match (up, right, down, left) {
        (true, true, _, _) => Hat::UpRight,
        (_, true, true, _) => Hat::DownRight,
        (_, _, true, true) => Hat::DownLeft,
        (true, _, _, true) => Hat::UpLeft,
        (true, false, false, false) => Hat::Up,
        (false, false, true, false) => Hat::Down,
        (false, false, false, true) => Hat::Left,
        (false, true, false, false) => Hat::Right,
        _ => Hat::Centered,
    };

    let mut buttons = 0;
    if row0[row0_column::TL] {
        buttons |= GamepadButton::Tl.mask();
    }

    if row0[row0_column::TR] {
        buttons |= GamepadButton::Tr.mask();
    }

    if row0[row0_column::X] {
        buttons |= GamepadButton::X.mask();
    }

    if row0[row0_column::Y] {
        buttons |= GamepadButton::Y.mask();
    }

    if row0[row0_column::B] {
        buttons |= GamepadButton::B.mask();
    }

    if row0[row0_column::A] {
        buttons |= GamepadButton::A.mask();
    }

    if !is_fn_active(row6) {
        if row6[row6_column::SELECT] {
            buttons |= GamepadButton::Select.mask();
        }
        if row6[row6_column::START] {
            buttons |= GamepadButton::Start.mask();
        }
    }

    GamepadReport {
        hat: hat as u8,
        buttons,
    }
}
