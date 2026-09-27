use embedded_hal::pwm::SetDutyCycle;

const PWM_STEP: u16 = 6553;
const INITIAL_DUTY: u16 = 32767;

pub struct Backlight<Bl> {
    channel: Bl,
    value: u16,
}

impl<Bl: SetDutyCycle> Backlight<Bl> {
    pub fn new(mut channel: Bl) -> Self {
        let _ = channel.set_duty_cycle(INITIAL_DUTY);

        Self {
            channel,
            value: INITIAL_DUTY,
        }
    }

    pub fn brightness_up(&mut self) {
        self.value = self.value.saturating_sub(PWM_STEP);
        let _ = self.channel.set_duty_cycle(self.value);
    }

    pub fn brightness_down(&mut self) {
        self.value = self.value.saturating_add(PWM_STEP);
        let _ = self.channel.set_duty_cycle(self.value);
    }
}
