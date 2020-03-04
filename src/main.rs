#![no_std]
#![no_main]

extern crate panic_halt;
use feather_m0 as hal;
use hal::prelude::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use atsamd_hal::samd21::sercom::{I2CMaster3};

type I2CMaster = I2CMaster3<atsamd_hal::samd21::sercom::Sercom3Pad0<atsamd_hal::common::gpio::Pa22<atsamd_hal::common::gpio::PfC>>, atsamd_hal::samd21::sercom::Sercom3Pad1<atsamd_hal::common::gpio::Pa23<atsamd_hal::common::gpio::PfC>>>;

const LCD_I2C_ADDRESS: u8 = 0x27;

fn write_nibble(i2c: &mut I2CMaster, delay: &mut Delay, val: u8, backlight: bool, command: bool) {
    i2c.write(LCD_I2C_ADDRESS, &[(val << 4) | 0b100 | (backlight as u8 * 0b1000) | (command as u8 * 0b1)]);
    delay.delay_us(1u8);
    i2c.write(LCD_I2C_ADDRESS, &[(val << 4) & !0b100 | (backlight as u8 * 0b1000) | (command as u8 * 0b1)]);
    delay.delay_us(50u8);
}

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

    // Manually reset the SPLC780D circuit to a 4-bit interface
    i2c.write(LCD_I2C_ADDRESS, &[0u8]);
    delay.delay_ms(50u8);
    write_nibble(&mut i2c, &mut delay, 0x03, false, false);
    delay.delay_ms(10u8);
    write_nibble(&mut i2c, &mut delay, 0x03, false, false);
    delay.delay_ms(1u8);
    write_nibble(&mut i2c, &mut delay, 0x03, false, false);

    // Set 4-bit mode (function set)
    write_nibble(&mut i2c, &mut delay, 0b0010, false, false);

    // Full Function Set
    write_nibble(&mut i2c, &mut delay, 0b0010, false, false);
    write_nibble(&mut i2c, &mut delay, 0b1000, false, false);

    // Display off
    write_nibble(&mut i2c, &mut delay, 0, false, false);
    write_nibble(&mut i2c, &mut delay, 0b1000, false, false);

    // Display Clear
    write_nibble(&mut i2c, &mut delay, 0, false, false);
    write_nibble(&mut i2c, &mut delay, 0b0001, false, false);

    // Entry mode set
    write_nibble(&mut i2c, &mut delay, 0, false, false);
    write_nibble(&mut i2c, &mut delay, 0b0111, false, false);
    delay.delay_ms(1u8);

    /* The LCD is ready for use. */
    // Return cursor home
    write_nibble(&mut i2c, &mut delay, 0, true, false);
    write_nibble(&mut i2c, &mut delay, 0b0011, true, false);
    delay.delay_ms(2u8);

    // Turn on display, cursor, blink
    write_nibble(&mut i2c, &mut delay, 0, true, false);
    write_nibble(&mut i2c, &mut delay, 0b1111, true, false);
    delay.delay_ms(1u8);

    // Set ddram
    for x in 0..80u8 {
        write_nibble(&mut i2c, &mut delay, 0b0011, true, true);
        write_nibble(&mut i2c, &mut delay, 0b0011, true, true);

    }

    loop {
        red_led.set_low().unwrap();
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
    }
}
