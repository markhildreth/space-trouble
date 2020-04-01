mod strings;
mod time_span;

use crate::common::*;
use core::fmt::Write;
use strings::get_action_text;
use time_span::{SpanStatus, TimeSpan};

pub trait LCD: Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}

const BLANK_LINE: &str = "                    ";
const BLOCK: char = 0xff as char;
const BLANK: char = ' ';

fn calc_blocks(remaining: Duration, total: Duration) -> u8 {
    (20 * remaining.as_millis() / total.as_millis()) as u8
}

pub struct DisplayActor<T>
where
    T: LCD,
{
    lcd: T,
    directive_time_span: Option<TimeSpan>,
    current_blocks: Option<u8>,
}

impl<T> DisplayActor<T>
where
    T: LCD,
{
    pub fn new(lcd: T) -> DisplayActor<T> {
        DisplayActor {
            lcd,
            directive_time_span: None,
            current_blocks: None,
        }
    }

    fn update_blocks(&mut self, blocks: u8) {
        self.lcd.set_cursor_pos(1, 0);
        for _ in 0..blocks {
            self.lcd.write_char(BLOCK).unwrap();
        }

        for _ in blocks..20 {
            self.lcd.write_char(BLANK).unwrap();
        }
    }
}

impl<T: LCD> Handles<TickEvent> for DisplayActor<T> {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        if let Some(time_span) = &self.directive_time_span {
            match time_span.status(ctx.now) {
                SpanStatus::Ongoing { remaining, total } => {
                    let blocks = calc_blocks(remaining, total);
                    if self.current_blocks.unwrap() != blocks {
                        self.current_blocks = Some(blocks);
                        self.update_blocks(blocks);
                    }
                }

                SpanStatus::Completed => {
                    self.directive_time_span = None;
                    self.current_blocks = None;
                    self.lcd.set_cursor_pos(1, 0);
                    self.lcd.write_str(BLANK_LINE).unwrap();
                    self.lcd.set_cursor_pos(2, 0);
                    self.lcd.write_str(BLANK_LINE).unwrap();
                    self.lcd.set_cursor_pos(3, 0);
                    self.lcd.write_str(BLANK_LINE).unwrap();
                }
            }
        }
    }
}

impl<T: LCD> Handles<ShipDistanceUpdatedEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: ShipDistanceUpdatedEvent, _: &mut Context) {
        self.lcd.set_cursor_pos(0, 0);
        self.lcd
            .write_fmt(format_args!("{} km", ev.distance))
            .unwrap();
    }
}

impl<T: LCD> Handles<HullHealthUpdatedEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: HullHealthUpdatedEvent, _: &mut Context) {
        self.lcd.set_cursor_pos(0, 16);
        self.lcd
            .write_fmt(format_args!("{: >3}%", ev.health))
            .unwrap();
    }
}

impl<T: LCD> Handles<NewDirectiveEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: NewDirectiveEvent, ctx: &mut Context) {
        let (command_text_1, command_text_2) = get_action_text(ev.directive.action);

        self.lcd.set_cursor_pos(2, 0);
        self.lcd.write_str(command_text_1).unwrap();
        self.lcd.set_cursor_pos(3, 0);
        self.lcd.write_str(command_text_2).unwrap();

        self.directive_time_span = Some(TimeSpan::new(ctx.now, ev.directive.time_limit));
        self.current_blocks = Some(20);
    }
}

impl<T: LCD> Handles<DirectiveCompletedEvent> for DisplayActor<T> {
    fn handle(&mut self, _: DirectiveCompletedEvent, _: &mut Context) {
        self.lcd.set_cursor_pos(1, 0);
        self.lcd.write_str(BLANK_LINE).unwrap();
        self.lcd.set_cursor_pos(2, 0);
        self.lcd.write_str(BLANK_LINE).unwrap();
        self.lcd.set_cursor_pos(3, 0);
        self.lcd.write_str(BLANK_LINE).unwrap();
        self.directive_time_span = None;
        self.current_blocks = None;
    }
}
