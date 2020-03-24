use atsamd_hal::gpio::{Pa22, Pa23, PfC};
use atsamd_hal::sercom::{I2CMaster3, Sercom3Pad0, Sercom3Pad1};
use feather_m0::delay::Delay;
use hd44780_driver::bus::I2CBus;
use hd44780_driver::HD44780;
pub use hd44780_driver::{Cursor, CursorBlink, Direction, Display, DisplayMode};

const DISPLAY_ADDRESS_ROWS: [u8; 4] = [0, 0x40, 0x14, 0x54];

pub type LCD = HD44780<Delay, I2CBus<I2CMaster3<Sercom3Pad0<Pa22<PfC>>, Sercom3Pad1<Pa23<PfC>>>>>;

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
