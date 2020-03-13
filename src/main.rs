#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use feather_m0::entry;

mod device;
mod game;
mod game_clock;
mod lcd;
mod transport;

use device::Device;
use lcd::{Cursor, CursorBlink, DisplayAddress};

#[entry]
fn main() -> ! {
    let mut device = Device::new();
    device.lcd.reset();
    device.lcd.set_cursor_visibility(Cursor::Invisible);
    device.lcd.set_cursor_blink(CursorBlink::Off);

    let NUMBER_START = 48u8;
    device.lcd.write_char((NUMBER_START + 3) as char);
    device.lcd.write_char((NUMBER_START + 8) as char);
    device.lcd.write_char((NUMBER_START + 4) as char);
    device.lcd.write_char('.');
    device.lcd.write_char((NUMBER_START + 4) as char);
    device.lcd.write_char((NUMBER_START + 0) as char);
    device.lcd.write_char((NUMBER_START + 0) as char);
    device.lcd.write_str("km");
    device
        .lcd
        .set_cursor_pos(DisplayAddress::from_row_col(0, 16).bits());
    device.lcd.write_str("100%");

    device
        .lcd
        .set_cursor_pos(DisplayAddress::from_row_col(1, 0).bits());
    for x in 0..11 {
        device.lcd.write_char(0xff as char);
    }

    /*
    device
        .lcd
        .set_cursor_pos(DisplayAddress::from_row_col(2, 2).bits());
    device.lcd.write_str("Set Oblitiblaster");
    device
        .lcd
        .set_cursor_pos(DisplayAddress::from_row_col(3, 8).bits());
    device.lcd.write_str("to 3");
    */
    device
        .lcd
        .set_cursor_pos(DisplayAddress::from_row_col(2, 1).bits());
    // device.lcd.write_str("Translation Error!");
    device
        .lcd
        .set_cursor_pos(DisplayAddress::from_row_col(3, 5).bits());
    device.lcd.write_str("Set ");
    device.lcd.write_char(0b11001110 as char);
    device.lcd.write_str(" to 3");

    loop {}
}
