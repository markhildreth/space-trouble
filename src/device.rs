use crate::lcd::LCD;
use crate::panels::Panel;
use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals, TC3};
use hal::prelude::*;
use hal::timer::TimerCounter;

const LCD_I2C_ADDRESS: u8 = 0x27;

pub struct DeviceComponents {
    pub panel: Panel,
    pub lcd: LCD,
    pub timer: TimerCounter<TC3>,
}

pub fn initialize_device() -> DeviceComponents {
    let core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let lcd = {
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
        let lcd = st_device::lcd::LCD::new_i2c(i2c, LCD_I2C_ADDRESS, lcd_delay);
        LCD::new(lcd)
    };

    let timer = {
        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
        timer.start(1.khz());
        timer
    };

    let panel = { Panel::default() };

    DeviceComponents { panel, lcd, timer }
}
