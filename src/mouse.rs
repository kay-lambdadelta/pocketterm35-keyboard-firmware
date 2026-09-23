use usbd_hid::descriptor::MouseReport;

use crate::keymap::{COLUMNS, row0_column};

const MOVE_STEP: i8 = 3;
const SCROLL_DIVIDER: u8 = 2;

#[derive(Default)]
pub struct MouseState {
    scroll_tick: u8,
}

impl MouseState {
    pub fn build_report(&mut self, row0: &[bool; COLUMNS]) -> MouseReport {
        let up = row0[row0_column::UP];
        let left = row0[row0_column::LEFT];
        let down = row0[row0_column::DOWN];
        let right = row0[row0_column::RIGHT];
        let scroll_down = row0[row0_column::X];
        let scroll_up = row0[row0_column::Y];
        let click_right = row0[row0_column::B];
        let click_left = row0[row0_column::A];

        let mut dx = 0;
        let mut dy = 0;
        let mut wheel = 0;

        if left {
            dx -= MOVE_STEP;
        }
        if right {
            dx += MOVE_STEP;
        }
        if up {
            dy -= MOVE_STEP;
        }
        if down {
            dy += MOVE_STEP;
        }

        if scroll_up || scroll_down {
            self.scroll_tick += 1;

            if self.scroll_tick >= SCROLL_DIVIDER {
                self.scroll_tick = 0;

                if scroll_up {
                    wheel += 1;
                }

                if scroll_down {
                    wheel -= 1;
                }
            }
        } else {
            self.scroll_tick = 0;
        }

        let mut buttons = 0u8;
        if click_left {
            buttons |= 1 << 0;
        }
        if click_right {
            buttons |= 1 << 1;
        }

        MouseReport {
            buttons,
            x: dx,
            y: dy,
            wheel,
            pan: 0,
        }
    }
}
