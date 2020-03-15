use crate::lcd::{Cursor, CursorBlink, DisplayAddress, LCD};
use core::fmt;
use core::fmt::Write;

pub struct GameScreen<'a> {
    distance: u16,
    hull_health: u16,
    timer: Option<u8>,
    command_text_1: Option<&'a str>,
    command_text_2: Option<&'a str>,
}

const BLOCK: char = 0xff as char;

impl<'a> GameScreen<'a> {
    pub fn new() -> GameScreen<'a> {
        GameScreen {
            distance: 3104,
            hull_health: 95,
            timer: Some(8),
            command_text_1: Some("  Set Oblitiblaster"),
            command_text_2: Some("        to 3"),
        }
    }

    pub fn update(&self, lcd: &mut LCD) {
        lcd.reset();
        lcd.set_cursor_visibility(Cursor::Invisible);
        lcd.set_cursor_blink(CursorBlink::Off);

        fmt::write(lcd, format_args!("{} km", self.distance)).unwrap();

        lcd.set_cursor_pos(DisplayAddress::from_row_col(0, 16).bits());
        fmt::write(lcd, format_args!("{: >3}%", self.hull_health)).unwrap();

        if let Some(blocks) = self.timer {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(1, 0).bits());
            for _ in 0..blocks {
                lcd.write_char(BLOCK);
            }
        }

        if let Some(command_text_1) = self.command_text_1 {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(2, 0).bits());
            lcd.write_str(command_text_1).unwrap();
        }

        if let Some(command_text_2) = self.command_text_2 {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(3, 0).bits());
            lcd.write_str(command_text_2).unwrap();
        }
    }
}
