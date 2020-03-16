use crate::lcd::{Cursor, CursorBlink, DisplayAddress, LCD};
use core::fmt;
use core::fmt::Write;

struct Dirtiable<T: Copy> {
    current: T,
    dirty: bool,
}

impl<T: Copy> Dirtiable<T> {
    pub fn new(start: T) -> Dirtiable<T> {
        Dirtiable {
            current: start,
            dirty: true,
        }
    }

    pub fn update(&mut self, new: T) {
        self.current = new;
        self.dirty = true;
    }

    pub fn clean(&mut self, mut f: impl FnMut(T)) {
        if self.dirty {
            f(self.current);
            self.dirty = false;
        }
    }
}

impl<T: Copy> core::convert::From<T> for Dirtiable<T> {
    fn from(value: T) -> Dirtiable<T> {
        Dirtiable::new(value)
    }
}

pub struct GameScreen<'a> {
    distance: Dirtiable<u16>,
    hull_health: Dirtiable<u16>,
    timer: Dirtiable<Option<u8>>,
    command_text_1: Dirtiable<Option<&'a str>>,
    command_text_2: Dirtiable<Option<&'a str>>,
}

const BLOCK: char = 0xff as char;

impl<'a> GameScreen<'a> {
    pub fn new() -> GameScreen<'a> {
        GameScreen {
            distance: 3104u16.into(),
            hull_health: 100.into(),
            timer: Some(8).into(),
            command_text_1: Some("  Set Oblitiblaster").into(),
            command_text_2: Some("        to 3").into(),
        }
    }

    pub fn update_timer(&mut self, n: Option<u8>) {
        self.timer.update(n);
    }

    pub fn init(&mut self, lcd: &mut LCD) {
        lcd.reset();
        lcd.set_cursor_visibility(Cursor::Invisible);
        lcd.set_cursor_blink(CursorBlink::Off);
    }

    pub fn update(&mut self, lcd: &mut LCD) {
        self.distance.clean(|new| {
            fmt::write(lcd, format_args!("{} km", new)).unwrap();
        });

        self.hull_health.clean(|new| {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(0, 16).bits());
            fmt::write(lcd, format_args!("{: >3}%", new)).unwrap();
        });

        self.timer.clean(|new| {
            if let Some(blocks) = new {
                lcd.set_cursor_pos(DisplayAddress::from_row_col(1, 0).bits());
                for _ in 0..blocks {
                    lcd.write_char(BLOCK);
                }
                for _ in blocks..=20 {
                    lcd.write_char(' ');
                }
            }
        });

        self.command_text_1.clean(|new| {
            if let Some(command_text_1) = new {
                lcd.set_cursor_pos(DisplayAddress::from_row_col(2, 0).bits());
                lcd.write_str(command_text_1).unwrap();
            }
        });

        self.command_text_2.clean(|new| {
            if let Some(command_text_2) = new {
                lcd.set_cursor_pos(DisplayAddress::from_row_col(3, 0).bits());
                lcd.write_str(command_text_2).unwrap();
            }
        });
    }
}
