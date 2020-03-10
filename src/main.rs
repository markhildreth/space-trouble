#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::TimerCounter;

mod lcd;
use crate::lcd::{Cursor, CursorBlink, Display, DisplayAddress, DisplayMode, LCD};

const LCD_I2C_ADDRESS: u8 = 0x27;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    red_led.set_high().unwrap();

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
    let mut lcd = LCD::new_i2c(i2c, LCD_I2C_ADDRESS, lcd_delay);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(1.hz());

    lcd.set_display_mode(DisplayMode {
        display: Display::On,
        cursor_visibility: Cursor::Invisible,
        cursor_blink: CursorBlink::Off,
    });
    lcd.reset();
    lcd.clear();
    lcd.set_cursor_pos(DisplayAddress::from_row_col(0, 10).bits());
    lcd.write_str("Hull: 100%").unwrap();
    let mut count = 0;
    loop {
        if let Ok(_) = timer.wait() {
            count += 1;
            lcd.set_cursor_pos(DisplayAddress::from_row_col(0, 0).bits());
            lcd.write_char(count.into());
        }
    }
}
