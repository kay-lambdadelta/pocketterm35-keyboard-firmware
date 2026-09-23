use usbd_hid::descriptor::generator_prelude::*;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = 0x05) = {
        (usage_page = GENERIC_DESKTOP, usage = 0x39, logical_min = 0, logical_max = 8) = {
            #[item_settings(data, variable, absolute)] hat = input;
        };
        (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8, logical_min = 0, logical_max = 1) = {
            #[packed_bits = 8] #[item_settings(data, variable, absolute)] buttons = input;
        };
    }
)]
pub struct GamepadReport {
    pub hat: u8,
    pub buttons: u8,
}

impl Default for GamepadReport {
    fn default() -> Self {
        Self {
            hat: Hat::Centered as u8,
            buttons: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum GamepadButton {
    A = 0,
    B = 1,
    X = 2,
    Y = 3,
    Tl = 4,
    Tr = 5,
    Select = 6,
    Start = 7,
}

impl GamepadButton {
    #[inline]
    pub const fn mask(self) -> u8 {
        1 << (self as u8)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Hat {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
    Centered = 8,
}
