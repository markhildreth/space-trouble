use crate::LCD;
use core::fmt;

type StaticStrRef = &'static str;

const BLANK_LINE: &str = "                    ";

enum DirtyState<T> {
    NotDirty,
    Dirty(T),
}

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

    pub fn clean(&mut self) -> DirtyState<T> {
        if self.dirty {
            self.dirty = false;
            DirtyState::Dirty(self.current)
        } else {
            DirtyState::NotDirty
        }
    }
}

impl<T: Copy + PartialEq + Eq> core::convert::From<T> for Dirtiable<T> {
    fn from(value: T) -> Dirtiable<T> {
        Dirtiable::new(value)
    }
}

pub struct GameScreen<T: LCD> {
    lcd: T,
    distance: Dirtiable<u32>,
    hull_health: Dirtiable<u8>,
    timer: Dirtiable<u8>,
    command_text_1: Dirtiable<Option<StaticStrRef>>,
    command_text_2: Dirtiable<Option<StaticStrRef>>,
}

const BLOCK: char = 0xff as char;

impl<T: LCD> GameScreen<T> {
    pub fn new(lcd: T) -> GameScreen<T> {
        GameScreen {
            lcd,
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

    pub fn update(&mut self) {
        if let DirtyState::Dirty(new) = self.distance.clean() {
            self.lcd.set_cursor_pos(0, 0);
            self.lcd.write_fmt(format_args!("{} km", new));
        }

        if let DirtyState::Dirty(new) = self.hull_health.clean() {
            self.lcd.set_cursor_pos(0, 16);
            self.lcd.write_fmt(format_args!("{: >3}%", new));
        }

        if let DirtyState::Dirty(new) = self.timer.clean() {
            self.lcd.set_cursor_pos(1, 0);
            for _ in 0..new {
                self.lcd.write_char(BLOCK);
            }
            for _ in new..20 {
                self.lcd.write_char(' ');
            }
        }

        if let DirtyState::Dirty(new) = self.command_text_1.clean() {
            self.lcd.set_cursor_pos(2, 0);
            if let Some(command_text_1) = new {
                self.lcd.write_str(command_text_1).unwrap();
            } else {
                self.lcd.write_str(BLANK_LINE).unwrap();
            }
        }

        if let DirtyState::Dirty(new) = self.command_text_2.clean() {
            self.lcd.set_cursor_pos(3, 0);
            if let Some(command_text_2) = new {
                self.lcd.write_str(command_text_2).unwrap();
            } else {
                self.lcd.write_str(BLANK_LINE).unwrap();
            }
        }
    }
}
