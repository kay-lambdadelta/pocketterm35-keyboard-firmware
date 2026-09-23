use core::time::Duration;

const HOLD: Duration = Duration::from_secs(4);

/// Index is the number of blinks the led will make
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceMode {
    #[default]
    Keyboard = 1,
    Gamepad = 2,
    Mouse = 3,
}

impl DeviceMode {
    fn next(self) -> Self {
        match self {
            DeviceMode::Keyboard => DeviceMode::Gamepad,
            DeviceMode::Gamepad => DeviceMode::Mouse,
            DeviceMode::Mouse => DeviceMode::Keyboard,
        }
    }
}

#[derive(Default)]
pub struct ModeChangeDetector {
    held_since: Option<Duration>,
    fired: bool,
}

impl ModeChangeDetector {
    pub fn update(
        &mut self,
        combo_held: bool,
        now: Duration,
        current: DeviceMode,
    ) -> Option<DeviceMode> {
        if !combo_held {
            self.held_since = None;
            self.fired = false;

            return None;
        }

        let since = *self.held_since.get_or_insert(now);

        if !self.fired && now.saturating_sub(since) >= HOLD {
            self.fired = true;

            return Some(current.next());
        }
        None
    }
}
