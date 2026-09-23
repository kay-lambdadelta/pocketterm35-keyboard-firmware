#![no_std]
#![no_main]

extern crate alloc;

mod backlight;
mod gamepad;
mod hid;
mod keyboard;
mod keymap;
mod matrix;
mod mode;
mod mouse;

use alloc::boxed::Box;
use core::{cell::RefCell, time::Duration};

use cortex_m::delay::Delay;
use critical_section::Mutex;
use embedded_alloc::TlsfHeap;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use hid::GamepadReport;
use keyboard::KeyEvent;
use keymap::row6_column;
use matrix::KeyMatrix;
use mode::{DeviceMode, ModeChangeDetector};
use mouse::MouseState;
use panic_halt as _;
use rp2040_hal::{
    Timer, Watchdog,
    clocks::{Clock, init_clocks_and_plls},
    entry,
    gpio::Pins,
    pwm::Slices,
    sio::Sio,
    usb::UsbBus,
};
use rp2040_pac::{NVIC, interrupt};
use usb_device::{
    LangID,
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{
        KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor, SystemControlReport,
    },
    hid_class::HIDClass,
};

use crate::backlight::Backlight;

#[used]
#[unsafe(link_section = ".boot2")]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const SCAN_INTERVAL: Duration = Duration::from_millis(10);
const ONE_SHOT_PULSE: Duration = Duration::from_millis(15);
const LED_CAPS_LOCK: u8 = 0x02;
const XTAL_FREQ_HZ: u32 = 12000000;
const USB_ID: UsbVidPid = UsbVidPid(0xcafe, 0x4011);

struct UsbContext {
    device: UsbDevice<'static, UsbBus>,
    keyboard: HIDClass<'static, UsbBus>,
    consumer: HIDClass<'static, UsbBus>,
    system: HIDClass<'static, UsbBus>,
    gamepad: HIDClass<'static, UsbBus>,
    mouse: HIDClass<'static, UsbBus>,
}

static USB_CONTEXT: Mutex<RefCell<Option<UsbContext>>> = Mutex::new(RefCell::new(None));

#[global_allocator]
static GLOBAL_HEAP: TlsfHeap = TlsfHeap::empty();

#[interrupt]
fn USBCTRL_IRQ() {
    critical_section::with(|critical_section| {
        if let Some(usb_context) = USB_CONTEXT.borrow_ref_mut(critical_section).as_mut() {
            usb_context.device.poll(&mut [
                &mut usb_context.keyboard,
                &mut usb_context.consumer,
                &mut usb_context.system,
                &mut usb_context.gamepad,
                &mut usb_context.mouse,
            ]);
        }
    });
}

fn blink_led(led: &mut impl OutputPin, times: u8, delay: &mut impl DelayNs, restore_high: bool) {
    for _ in 0..times {
        let _ = led.set_high();
        delay.delay_ms(120);

        let _ = led.set_low();
        delay.delay_ms(120);
    }

    let _ = if restore_high {
        led.set_high()
    } else {
        led.set_low()
    };
}

#[entry]
fn main() -> ! {
    // 32 kilobytes, arbitrary
    unsafe {
        embedded_alloc::init!(GLOBAL_HEAP, 32 * 1024);
    }

    let mut pac = rp2040_pac::Peripherals::take().unwrap();
    let core = rp2040_pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let row_pins = [
        pins.gpio16.into_pull_down_input().into_dyn_pin(),
        pins.gpio10.into_pull_down_input().into_dyn_pin(),
        pins.gpio11.into_pull_down_input().into_dyn_pin(),
        pins.gpio12.into_pull_down_input().into_dyn_pin(),
        pins.gpio13.into_pull_down_input().into_dyn_pin(),
        pins.gpio14.into_pull_down_input().into_dyn_pin(),
        pins.gpio15.into_pull_down_input().into_dyn_pin(),
    ];

    let col_pins = [
        pins.gpio0.into_push_pull_output().into_dyn_pin(),
        pins.gpio1.into_push_pull_output().into_dyn_pin(),
        pins.gpio2.into_push_pull_output().into_dyn_pin(),
        pins.gpio3.into_push_pull_output().into_dyn_pin(),
        pins.gpio4.into_push_pull_output().into_dyn_pin(),
        pins.gpio5.into_push_pull_output().into_dyn_pin(),
        pins.gpio6.into_push_pull_output().into_dyn_pin(),
        pins.gpio7.into_push_pull_output().into_dyn_pin(),
        pins.gpio8.into_push_pull_output().into_dyn_pin(),
        pins.gpio9.into_push_pull_output().into_dyn_pin(),
    ];

    let mut matrix = KeyMatrix::new(row_pins, col_pins);

    let mut caps_led = pins.gpio22.into_push_pull_output();
    let mut mute_led = pins.gpio19.into_push_pull_output();
    let mut bl_ctrl_led = pins.gpio21.into_push_pull_output();

    let _ = caps_led.set_low();
    let _ = mute_led.set_low();
    let _ = bl_ctrl_led.set_low();

    let pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
    let mut pwm_bl = pwm_slices.pwm2;
    let mut pwm_ad = pwm_slices.pwm1;

    pwm_bl.set_div_int(2);
    pwm_bl.set_top(65535);
    pwm_bl.enable();

    pwm_ad.set_div_int(2);
    pwm_ad.set_top(65535);
    pwm_ad.enable();

    let mut bl_channel = pwm_bl.channel_a;
    bl_channel.output_to(pins.gpio20);

    let mut ad_channel = pwm_ad.channel_a;
    ad_channel.output_to(pins.gpio18);

    let mut backlight = Backlight::new(bl_channel, ad_channel);

    // Leak the USB bus so it can be static
    let usb_bus = Box::leak(Box::new(UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    )))) as _;

    let string_descriptors = [StringDescriptors::new(LangID::EN_US)
        .manufacturer("Waveshare")
        .product("PocketTerm35 Builtin")
        .serial_number("343434")];

    let usb_device = UsbDeviceBuilder::new(usb_bus, USB_ID)
        .strings(&string_descriptors)
        .unwrap()
        .build();

    critical_section::with(|critical_section| {
        USB_CONTEXT.replace(
            critical_section,
            Some(UsbContext {
                device: usb_device,
                keyboard: HIDClass::new(usb_bus, KeyboardReport::desc(), 1),
                consumer: HIDClass::new_ep_in(usb_bus, MediaKeyboardReport::desc(), 10),
                system: HIDClass::new_ep_in(usb_bus, SystemControlReport::desc(), 4),
                gamepad: HIDClass::new_ep_in(usb_bus, GamepadReport::desc(), 4),
                mouse: HIDClass::new_ep_in(usb_bus, MouseReport::desc(), 4),
            }),
        );
    });

    unsafe {
        NVIC::unmask(rp2040_pac::Interrupt::USBCTRL_IRQ);
    }

    let mut device_mode = DeviceMode::default();
    let mut mode_change = ModeChangeDetector::default();
    let mut mouse_state = MouseState::default();
    let mut capslock_led_on = false;
    let mut next_scan = Duration::ZERO;

    loop {
        let instant = timer.get_counter();
        let ticks = instant.ticks();
        let now = Duration::from_micros(ticks);

        if now < next_scan {
            continue;
        }
        next_scan = now + SCAN_INTERVAL;
        matrix.scan(&mut timer);

        let combo_held = matrix.row(6)[row6_column::SELECT] && matrix.row(6)[row6_column::START];

        if let Some(new_mode) = mode_change.update(combo_held, now, device_mode) {
            device_mode = new_mode;

            blink_led(&mut caps_led, device_mode as _, &mut delay, capslock_led_on);
        }

        let keyboard_report = keyboard::build_report(device_mode, &matrix, |event| match event {
            KeyEvent::Media(media_key) => {
                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .consumer
                        .push_input(&MediaKeyboardReport {
                            usage_id: media_key as u16,
                        });
                });

                delay.delay_ms(ONE_SHOT_PULSE.subsec_millis());

                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .consumer
                        .push_input(&MediaKeyboardReport { usage_id: 0 });
                });
            }
            KeyEvent::OneShot { modifier, usage } => {
                let mut keys = [0; 6];
                keys[0] = usage as u8;

                let report = KeyboardReport {
                    modifier,
                    reserved: 0,
                    leds: 0,
                    keycodes: keys,
                };

                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .keyboard
                        .push_input(&report);
                });

                delay.delay_ms(ONE_SHOT_PULSE.subsec_millis());

                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .keyboard
                        .push_input(&KeyboardReport::default());
                });
            }
            KeyEvent::System(system_key) => {
                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .system
                        .push_input(&SystemControlReport {
                            usage_id: system_key as u8,
                        });
                });

                delay.delay_ms(ONE_SHOT_PULSE.subsec_millis());

                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .system
                        .push_input(&SystemControlReport { usage_id: 0 });
                });
            }
            KeyEvent::BacklightUp => backlight.bl_up(),
            KeyEvent::BacklightDown => backlight.bl_down(),
            KeyEvent::AdUp => backlight.ad_up(),
            KeyEvent::AdDown => backlight.ad_down(),
        });

        critical_section::with(|critical_section| {
            let mut usb_context = USB_CONTEXT.borrow_ref_mut(critical_section);
            let usb_context = usb_context.as_mut().unwrap();

            let _ = usb_context.keyboard.push_input(&keyboard_report);

            let mut led = 0;
            if usb_context
                .keyboard
                .pull_raw_output(core::array::from_mut(&mut led))
                .is_ok()
            {
                capslock_led_on = (led & LED_CAPS_LOCK) != 0;
            }
        });

        let _ = if capslock_led_on {
            caps_led.set_high()
        } else {
            caps_led.set_low()
        };

        match device_mode {
            DeviceMode::Gamepad => {
                let report = gamepad::build_report(matrix.row(0), matrix.row(6));

                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .gamepad
                        .push_input(&report);
                });
            }
            DeviceMode::Mouse => {
                let report = mouse_state.build_report(matrix.row(0));

                critical_section::with(|critical_section| {
                    let _ = USB_CONTEXT
                        .borrow_ref_mut(critical_section)
                        .as_mut()
                        .unwrap()
                        .mouse
                        .push_input(&report);
                });
            }
            DeviceMode::Keyboard => {}
        }
    }
}
