use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::{Input, OpenDrain, Output, Pa15, Pa17, PullDown};
use hal::pac::{CorePeripherals, Peripherals, TC3};
use hal::prelude::*;
use hal::timer::TimerCounter;

use crate::lcd::LCD;

const LCD_I2C_ADDRESS: u8 = 0x27;

pub struct Device {
    pub led_pin: Pa17<Output<OpenDrain>>,
    pub button_pin: Pa15<Input<PullDown>>,
    pub lcd: LCD,
    pub timer: TimerCounter<TC3>,
    ms: u32,
}

impl Device {
    pub fn new() -> Device {
        let core = CorePeripherals::take().unwrap();
        let mut peripherals = Peripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut pins = hal::Pins::new(peripherals.PORT);

        let mut led_pin = pins.d13.into_open_drain_output(&mut pins.port);
        led_pin.set_low().unwrap();
        let button_pin = pins.d5.into_pull_down_input(&mut pins.port);

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

        Device {
            led_pin,
            button_pin,
            lcd,
            timer,
            ms: 0,
        }
    }

    pub fn ms(&self) -> u32 {
        return self.ms;
    }

    pub fn update(&mut self) {
        if let Ok(_) = self.timer.wait() {
            self.ms += 1;
        }
    }
}
