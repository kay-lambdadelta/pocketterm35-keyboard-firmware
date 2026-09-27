use alloc::string::ToString;
use core::panic::PanicInfo;

use morse_codec::{
    DEFAULT_CHARACTER_SET,
    encoder::{Encoder, SDM},
};
use rp2040_pac::SIO;

const UNIT: u32 = 10000000;
const LED: u32 = 1 << 22;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut message = info.to_string();
    message.retain(|character| {
        character.is_ascii() && DEFAULT_CHARACTER_SET.contains(&(character as u8))
    });

    let mut encoder = Encoder::<256>::new().with_message(&message, false).build();

    encoder.encode_message_all();

    unsafe {
        let sio = SIO::steal();

        sio.gpio_oe_set().write(|w| w.gpio_oe_set().bits(LED));

        cortex_m::asm::delay(2000000);

        loop {
            for sdm in encoder
                .get_encoded_message_as_sdm_arrays()
                .flatten()
                .flatten()
            {
                match sdm {
                    SDM::High(multiplier) => {
                        sio.gpio_out_set().write(|w| w.gpio_out_set().bits(LED));

                        cortex_m::asm::delay(UNIT * multiplier as u32);
                    }

                    SDM::Low(multiplier) => {
                        sio.gpio_out_clr().write(|w| w.gpio_out_clr().bits(LED));

                        cortex_m::asm::delay(UNIT * multiplier as u32);
                    }
                    SDM::Empty => {}
                }
            }

            sio.gpio_out_clr().write(|w| w.gpio_out_clr().bits(LED));

            cortex_m::asm::delay(4000000);
        }
    }
}
