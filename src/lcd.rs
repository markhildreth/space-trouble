use st_device::lcd::{DisplayAddress, LCD as LCDImpl};

pub struct LCD {
    lcd: LCDImpl,
}

impl LCD {
    pub fn new(lcd: LCDImpl) -> LCD {
        LCD { lcd }
    }
}

impl st_client::LCD for LCD {
    fn set_cursor_pos(&mut self, row: u8, col: u8) {
        self.lcd
            .set_cursor_pos(DisplayAddress::from_row_col(row, col).bits())
    }
}

impl core::fmt::Write for LCD {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.lcd.write_str(s)
    }
}
