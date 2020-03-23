use crate::lcd::{Cursor, CursorBlink, DisplayAddress, LCD};
use core::fmt;
use core::fmt::Write;

type StaticStrRef = &'static str;

const BLANK_LINE: &str = "                    ";

struct Dirtiable<T: Copy + PartialEq + Eq> {
    current: T,
    dirty: bool,
}

impl<T: Copy + PartialEq + Eq> Dirtiable<T> {
    pub fn new(start: T) -> Dirtiable<T> {
        Dirtiable {
            current: start,
            dirty: true,
        }
    }

    pub fn update(&mut self, new: T) {
        if self.current != new {
            self.current = new;
            self.dirty = true;
        }
    }

    pub fn clean(&mut self, mut f: impl FnMut(T)) {
        if self.dirty {
            f(self.current);
            self.dirty = false;
        }
    }
}

impl<T: Copy + PartialEq + Eq> core::convert::From<T> for Dirtiable<T> {
    fn from(value: T) -> Dirtiable<T> {
        Dirtiable::new(value)
    }
}

pub struct GameScreen {
    distance: Dirtiable<u32>,
    hull_health: Dirtiable<u8>,
    timer: Dirtiable<u8>,
    command_text_1: Dirtiable<Option<StaticStrRef>>,
    command_text_2: Dirtiable<Option<StaticStrRef>>,
}

const BLOCK: char = 0xff as char;

impl GameScreen {
    pub fn new() -> GameScreen {
        GameScreen {
            distance: 0.into(),
            hull_health: 100.into(),
            timer: 0.into(),
            command_text_1: None.into(),
            command_text_2: None.into(),
        }
    }

    pub fn update_distance(&mut self, distance: u32) {
        self.distance.update(distance);
    }

    pub fn update_hull_health(&mut self, hull_health: u8) {
        self.hull_health.update(hull_health);
    }

    pub fn update_command_text(
        &mut self,
        text_1: Option<StaticStrRef>,
        text_2: Option<StaticStrRef>,
    ) {
        self.command_text_1.update(text_1);
        self.command_text_2.update(text_2);
    }

    pub fn update_timer(&mut self, n: u8) {
        self.timer.update(n);
    }

    pub fn init(&mut self, lcd: &mut LCD) {
        lcd.reset();
        lcd.set_cursor_visibility(Cursor::Invisible);
        lcd.set_cursor_blink(CursorBlink::Off);
    }

    pub fn update(&mut self, lcd: &mut LCD) {
        self.distance.clean(|new| {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(0, 0).bits());
            fmt::write(lcd, format_args!("{} km", new)).unwrap();
        });

        self.hull_health.clean(|new| {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(0, 16).bits());
            fmt::write(lcd, format_args!("{: >3}%", new)).unwrap();
        });

        self.timer.clean(|new| {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(1, 0).bits());
            for _ in 0..new {
                lcd.write_char(BLOCK);
            }
            for _ in new..20 {
                lcd.write_char(' ');
            }
        });

        self.command_text_1.clean(|new| {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(2, 0).bits());
            if let Some(command_text_1) = new {
                lcd.write_str(command_text_1).unwrap();
            } else {
                lcd.write_str(BLANK_LINE).unwrap();
            }
        });

        self.command_text_2.clean(|new| {
            lcd.set_cursor_pos(DisplayAddress::from_row_col(3, 0).bits());
            if let Some(command_text_2) = new {
                lcd.write_str(command_text_2).unwrap();
            } else {
                lcd.write_str(BLANK_LINE).unwrap();
            }
        });
    }
}
