use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals, TC3};
use hal::prelude::*;
use hal::timer::TimerCounter;

use crate::lcd::LCD;

const LCD_I2C_ADDRESS: u8 = 0x27;

pub struct Device {
    clocks: GenericClockController,
    timer: TimerCounter<TC3>,
    clock_counter: u32,
}

impl Device {
    pub fn new() -> Device {
        let mut peripherals = Peripherals::take().unwrap();
        let core = CorePeripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );

        let mut pins = hal::Pins::new(peripherals.PORT);

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
        timer.start(1.khz());

        Device {
            clocks,
            clock_counter: 0,
            timer,
        }
    }

    pub fn led(&self) -> hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>> {
        let peripherals = Peripherals::take().unwrap();
        let mut pins = hal::Pins::new(peripherals.PORT);
        pins.d13.into_open_drain_output(&mut pins.port)
    }

    pub fn tick(&mut self) {
        if let Ok(_) = self.timer.wait() {
            self.clock_counter += 1;
        }
    }

    pub fn millis(&self) -> u32 {
        self.clock_counter
    }

    pub fn lcd(&self) -> LCD {
        let core = CorePeripherals::take().unwrap();
        let peripherals = Peripherals::take().unwrap();
        let pins = hal::Pins::new(peripherals.PORT);

        let i2c = hal::i2c_master(
            &mut self.clocks,
            270.khz(),
            peripherals.SERCOM3,
            &mut peripherals.PM,
            pins.sda,
            pins.scl,
            &mut pins.port,
        );

        let lcd_delay = Delay::new(core.SYST, self.clocks.gclk0());
        let lcd = LCD::new_i2c(i2c, LCD_I2C_ADDRESS, lcd_delay);
    }
}
