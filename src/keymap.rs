use usbd_hid::descriptor::{KeyboardUsage, MediaKey, SystemControlKey};

pub const ROWS: usize = 7;
pub const COLUMNS: usize = 10;

pub mod modifier {
    pub const LEFT_CTRL: u8 = 0x01;
    pub const LEFT_SHIFT: u8 = 0x02;
    pub const LEFT_ALT: u8 = 0x04;
    pub const LEFT_GUI: u8 = 0x08;
    pub const RIGHT_ALT: u8 = 0x40;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    Key(KeyboardUsage),
    Modifier(u8),
    Fn,
    Media(MediaKey),
    System(SystemControlKey),
    OneShot { modifier: u8, usage: KeyboardUsage },
    BacklightUp,
    BacklightDown,
    AdUp,
    AdDown,
}

pub const KEY_MAP: [[Option<KeyAction>; COLUMNS]; ROWS] = [
    [None, None, None, None, None, None, None, None, None, None],
    [
        Some(KeyAction::Key(KeyboardUsage::Keyboard1Exclamation)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard2At)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard3Hash)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard4Dollar)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard5Percent)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard6Caret)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard7Ampersand)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard8Asterisk)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard9OpenParens)),
        Some(KeyAction::Key(KeyboardUsage::Keyboard0CloseParens)),
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardQq)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardWw)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardEe)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardRr)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardTt)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardYy)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardUu)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardIi)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardOo)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardPp)),
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardAa)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSs)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardDd)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardFf)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardGg)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardHh)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardJj)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardKk)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardLl)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardBackspace)),
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardZz)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardXx)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardCc)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardVv)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardBb)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardNn)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardMm)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSlashQuestion)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardEnter)),
        None,
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardTab)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardCapsLock)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardDashUnderscore)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardEqualPlus)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSemiColon)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSingleDoubleQuote)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardCommaLess)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardPeriodGreater)),
        Some(KeyAction::Modifier(modifier::LEFT_SHIFT)),
        None,
    ],
    [
        Some(KeyAction::Fn),
        Some(KeyAction::Modifier(modifier::LEFT_CTRL)),
        Some(KeyAction::Modifier(modifier::LEFT_ALT)),
        None,
        Some(KeyAction::Key(KeyboardUsage::KeyboardSpacebar)),
        None,
        Some(KeyAction::Modifier(modifier::RIGHT_ALT)),
        Some(KeyAction::Modifier(modifier::LEFT_GUI)),
        Some(KeyAction::Fn),
        None,
    ],
];

pub const FN_MAP: [[Option<KeyAction>; COLUMNS]; ROWS] = [
    [None, None, None, None, None, None, None, None, None, None],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardF1)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF2)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF3)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF4)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF5)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF6)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF7)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF8)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF9)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF10)),
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardEscape)),
        Some(KeyAction::Media(MediaKey::Mute)),
        Some(KeyAction::AdDown),
        Some(KeyAction::AdUp),
        Some(KeyAction::Media(MediaKey::PrevTrack)),
        Some(KeyAction::Media(MediaKey::PlayPause)),
        Some(KeyAction::Media(MediaKey::NextTrack)),
        Some(KeyAction::OneShot {
            modifier: modifier::LEFT_GUI,
            usage: KeyboardUsage::KeyboardLl,
        }),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF11)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardF12)),
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardBacktickTilde)),
        Some(KeyAction::OneShot {
            modifier: modifier::LEFT_SHIFT,
            usage: KeyboardUsage::KeyboardBacktickTilde,
        }),
        Some(KeyAction::Key(KeyboardUsage::KeyboardBackslashBar)),
        Some(KeyAction::OneShot {
            modifier: modifier::LEFT_SHIFT,
            usage: KeyboardUsage::KeyboardBackslashBar,
        }),
        Some(KeyAction::OneShot {
            modifier: modifier::LEFT_SHIFT,
            usage: KeyboardUsage::KeyboardOpenBracketBrace,
        }),
        Some(KeyAction::OneShot {
            modifier: modifier::LEFT_SHIFT,
            usage: KeyboardUsage::KeyboardCloseBracketBrace,
        }),
        Some(KeyAction::Key(KeyboardUsage::KeyboardOpenBracketBrace)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardCloseBracketBrace)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardLl)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardDelete)),
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardInsert)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardHome)),
        Some(KeyAction::System(SystemControlKey::PowerDown)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardEnd)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardPageUp)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardPageDown)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardScrollLock)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSlashQuestion)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardEnter)),
        None,
    ],
    [
        Some(KeyAction::Key(KeyboardUsage::KeyboardTab)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardCapsLock)),
        Some(KeyAction::BacklightDown),
        Some(KeyAction::BacklightUp),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSemiColon)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSingleDoubleQuote)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardCommaLess)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardPeriodGreater)),
        Some(KeyAction::Modifier(modifier::LEFT_SHIFT)),
        None,
    ],
    [
        Some(KeyAction::Fn),
        Some(KeyAction::Modifier(modifier::LEFT_CTRL)),
        Some(KeyAction::Modifier(modifier::LEFT_ALT)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardPrintScreen)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardSpacebar)),
        Some(KeyAction::Key(KeyboardUsage::KeyboardPause)),
        Some(KeyAction::Modifier(modifier::RIGHT_ALT)),
        Some(KeyAction::Modifier(modifier::LEFT_GUI)),
        Some(KeyAction::Fn),
        None,
    ],
];

pub const KEYBOARD_ROW0_MAP: [Option<KeyboardUsage>; COLUMNS] = [
    Some(KeyboardUsage::KeyboardUpArrow),
    Some(KeyboardUsage::KeyboardLeftArrow),
    Some(KeyboardUsage::KeyboardDownArrow),
    Some(KeyboardUsage::KeyboardRightArrow),
    Some(KeyboardUsage::KeyboardPageUp),
    Some(KeyboardUsage::KeyboardPageDown),
    Some(KeyboardUsage::KeyboardSpacebar),
    Some(KeyboardUsage::KeyboardTab),
    Some(KeyboardUsage::KeyboardEscape),
    Some(KeyboardUsage::KeyboardEnter),
];

pub mod row0_column {
    pub const UP: usize = 0;
    pub const LEFT: usize = 1;
    pub const DOWN: usize = 2;
    pub const RIGHT: usize = 3;
    pub const TL: usize = 4;
    pub const TR: usize = 5;
    pub const X: usize = 6;
    pub const Y: usize = 7;
    pub const B: usize = 8;
    pub const A: usize = 9;
}

pub mod row6_column {
    pub const FN_LEFT: usize = 0;
    pub const SELECT: usize = 3;
    pub const START: usize = 5;
    pub const FN_RIGHT: usize = 8;
}

#[inline]
pub fn is_fn_active(row6: &[bool; COLUMNS]) -> bool {
    row6[row6_column::FN_LEFT] || row6[row6_column::FN_RIGHT]
}
