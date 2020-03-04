#![no_std]
#![no_main]

#[macro_use]
extern crate bitflags;
extern crate panic_halt;

use feather_m0 as hal;
use hal::prelude::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::clock::GenericClockController;
use hal::delay::Delay;

mod lcd;

use crate::lcd::{I2CLCD, Backlight, DisplayControls};

const LCD_I2C_ADDRESS: u8 = 0x27;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut i2c = hal::i2c_master(
        &mut clocks,
        270.khz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );

    let mut lcd = I2CLCD::new(LCD_I2C_ADDRESS);
    lcd.initialize(&mut i2c, &mut delay);
    lcd.clear(&mut i2c, &mut delay);
    lcd.backlight(&mut i2c, &mut delay, Backlight::ON);
    lcd.display_control(&mut i2c, &mut delay, DisplayControls::DISPLAY | DisplayControls::CURSOR);


    /*
    // Set ddram
    for x in 0..80u8 {
        write_nibble(&mut i2c, &mut delay, 0b0011, true, true);
        write_nibble(&mut i2c, &mut delay, 0b0011, true, true);
    }
    */

    loop {
        red_led.set_low().unwrap();
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
    }
}
