use embedded_hal::pwm::SetDutyCycle;

const PWM_STEP: u16 = 6553;
const INITIAL_DUTY: u16 = 32767;

pub struct Backlight<Bl, Ad> {
    bl_channel: Bl,
    ad_channel: Ad,
    bl_value: u16,
    ad_value: u16,
}

impl<Bl: SetDutyCycle, Ad: SetDutyCycle> Backlight<Bl, Ad> {
    pub fn new(mut bl_channel: Bl, mut ad_channel: Ad) -> Self {
        let _ = bl_channel.set_duty_cycle(INITIAL_DUTY);
        let _ = ad_channel.set_duty_cycle(INITIAL_DUTY);

        Self {
            bl_channel,
            ad_channel,
            bl_value: INITIAL_DUTY,
            ad_value: INITIAL_DUTY,
        }
    }

    pub fn bl_up(&mut self) {
        self.bl_value = self.bl_value.saturating_sub(PWM_STEP);
        let _ = self.bl_channel.set_duty_cycle(self.bl_value);
    }

    pub fn bl_down(&mut self) {
        self.bl_value = self.bl_value.saturating_add(PWM_STEP);
        let _ = self.bl_channel.set_duty_cycle(self.bl_value);
    }

    pub fn ad_up(&mut self) {
        self.ad_value = self.ad_value.saturating_add(PWM_STEP);
        let _ = self.ad_channel.set_duty_cycle(self.ad_value);
    }

    pub fn ad_down(&mut self) {
        self.ad_value = self.ad_value.saturating_sub(PWM_STEP);
        let _ = self.ad_channel.set_duty_cycle(self.ad_value);
    }
}
