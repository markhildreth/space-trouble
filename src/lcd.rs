use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write;
use feather_m0 as hal;
use hal::delay::Delay;
pub use hd44780_driver::{Cursor, CursorBlink, Direction, Display, DisplayMode, HD44780};

const DISPLAY_ADDRESS_ROWS: [u8; 4] = [0, 0x40, 0x14, 0x54];

pub type LCD<A, B> = HD44780<A, B>;

pub struct DisplayAddress {
    address: u8,
}

impl DisplayAddress {
    pub fn raw(address: u8) -> DisplayAddress {
        DisplayAddress { address }
    }

    pub fn from_row_col(row: u8, col: u8) -> DisplayAddress {
        DisplayAddress {
            address: DISPLAY_ADDRESS_ROWS[row as usize] + col,
        }
    }

    pub fn bits(&self) -> u8 {
        self.address
    }
}
