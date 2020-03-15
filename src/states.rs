use crate::device::Device;
use crate::lcd::{Cursor, CursorBlink, DisplayAddress};
use core::fmt::Write;

pub struct GameState {}

const BLOCK: char = 0xff as char;
const NUMBER_START: u8 = 48u8;

impl GameState {
    pub fn new(mut device: Device) -> Self {
        device.lcd.reset();
        device.lcd.set_cursor_visibility(Cursor::Invisible);
        device.lcd.set_cursor_blink(CursorBlink::Off);

        device.lcd.write_char((NUMBER_START + 3) as char);
        device.lcd.write_char((NUMBER_START + 8) as char);
        device.lcd.write_char((NUMBER_START + 4) as char);
        device.lcd.write_char('.');
        device.lcd.write_char((NUMBER_START + 4) as char);
        device.lcd.write_char((NUMBER_START + 0) as char);
        device.lcd.write_char((NUMBER_START + 0) as char);
        device.lcd.write_str("km").unwrap();
        device
            .lcd
            .set_cursor_pos(DisplayAddress::from_row_col(0, 16).bits());
        device.lcd.write_str("100%").unwrap();

        device
            .lcd
            .set_cursor_pos(DisplayAddress::from_row_col(1, 0).bits());

        for _ in 0..11 {
            device.lcd.write_char(BLOCK);
        }

        device
            .lcd
            .set_cursor_pos(DisplayAddress::from_row_col(2, 2).bits());
        device.lcd.write_str("Set Oblitiblaster").unwrap();
        device
            .lcd
            .set_cursor_pos(DisplayAddress::from_row_col(3, 8).bits());
        device.lcd.write_str("to 3").unwrap();

        GameState {}
    }

    pub fn tick(self) -> GameState {
        self
    }
}
