use feather_m0::delay::Delay;
use feather_m0::gpio::{Pa22, Pa23, PfC};
use feather_m0::sercom::{I2CMaster3, Sercom3Pad0, Sercom3Pad1};
use hd44780_driver::bus::I2CBus;
use hd44780_driver::HD44780;

use hd44780_driver::{Cursor, CursorBlink};

const DISPLAY_ADDRESS_ROWS: [u8; 4] = [0, 0x40, 0x14, 0x54];

pub type LCDImpl =
    HD44780<Delay, I2CBus<I2CMaster3<Sercom3Pad0<Pa22<PfC>>, Sercom3Pad1<Pa23<PfC>>>>>;
pub struct LCD(LCDImpl);

impl LCD {
    pub fn new(mut lcd: LCDImpl) -> LCD {
        lcd.reset();
        lcd.set_cursor_visibility(Cursor::Invisible);
        lcd.set_cursor_blink(CursorBlink::Off);
        LCD(lcd)
    }

    pub fn set_cursor_pos(&mut self, row: u8, col: u8) {
        self.0
            .set_cursor_pos(DisplayAddress::from_row_col(row, col).bits())
    }
}

impl core::fmt::Write for LCD {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.0.write_str(s)
    }
}

pub struct DisplayAddress {
    address: u8,
}

impl DisplayAddress {
    pub fn from_row_col(row: u8, col: u8) -> DisplayAddress {
        DisplayAddress {
            address: DISPLAY_ADDRESS_ROWS[row as usize] + col,
        }
    }

    pub fn bits(&self) -> u8 {
        self.address
    }
}
