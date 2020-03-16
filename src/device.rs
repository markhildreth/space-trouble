use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::{OpenDrain, Output, Pa17};
use hal::pac::{CorePeripherals, Peripherals, TC3};
use hal::prelude::*;
use hal::timer::TimerCounter;

use crate::lcd::LCD;

const LCD_I2C_ADDRESS: u8 = 0x27;

pub struct Device {
    pub led_pin: Pa17<Output<OpenDrain>>,
    pub lcd: LCD,
}

impl Device {
    pub fn new() -> (Device, TimerCounter<TC3>) {
        let core = CorePeripherals::take().unwrap();
        let mut peripherals = Peripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut pins = hal::Pins::new(peripherals.PORT);

        let led_pin = pins.d13.into_open_drain_output(&mut pins.port);

        let i2c = hal::i2c_master(
            &mut clocks,
            270.khz(),
            peripherals.SERCOM3,
            &mut peripherals.PM,
            pins.sda,
            pins.scl,
            &mut pins.port,
        );

        let lcd_delay = Delay::new(core.SYST, &mut clocks);
        let lcd = LCD::new_i2c(i2c, LCD_I2C_ADDRESS, lcd_delay);

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
        timer.start(1.khz());

        let device = Device { led_pin, lcd };
        (device, timer)
    }
}
