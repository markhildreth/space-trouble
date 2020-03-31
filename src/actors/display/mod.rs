mod strings;
mod time_span;

use super::Context;
use crate::common::*;
use crate::lcd::LCD;
use core::fmt::Write;
use strings::get_action_text;
use time_span::{SpanStatus, TimeSpan};

const BLANK_LINE: &str = "                    ";
const BLOCK: char = 0xff as char;
const BLANK: char = ' ';

fn calc_blocks(remaining: Duration, total: Duration) -> u8 {
    return (20 * remaining.as_millis() / total.as_millis()) as u8;
}

pub struct DisplayActor {
    directive_time_span: Option<TimeSpan>,
    current_blocks: Option<u8>,
}

impl DisplayActor {
    pub fn new() -> DisplayActor {
        DisplayActor {
            directive_time_span: None,
            current_blocks: None,
        }
    }

    fn update_blocks(&self, lcd: &mut LCD, blocks: u8) {
        lcd.set_cursor_pos(1, 0);
        for _ in 0..blocks {
            lcd.write_char(BLOCK).unwrap();
        }

        for _ in blocks..20 {
            lcd.write_char(BLANK).unwrap();
        }
    }
}

impl Handler for DisplayActor {
    type Context = Context;
}

impl Handles<TickEvent> for DisplayActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        if let Some(time_span) = &self.directive_time_span {
            match time_span.status(ctx.now) {
                SpanStatus::Ongoing { remaining, total } => {
                    let blocks = calc_blocks(remaining, total);
                    if self.current_blocks.unwrap() != blocks {
                        self.current_blocks = Some(blocks);
                        self.update_blocks(&mut ctx.lcd, blocks);
                    }
                }

                SpanStatus::Completed => {
                    self.directive_time_span = None;
                    self.current_blocks = None;
                    ctx.lcd.set_cursor_pos(1, 0);
                    ctx.lcd.write_str(BLANK_LINE).unwrap();
                    ctx.lcd.set_cursor_pos(2, 0);
                    ctx.lcd.write_str(BLANK_LINE).unwrap();
                    ctx.lcd.set_cursor_pos(3, 0);
                    ctx.lcd.write_str(BLANK_LINE).unwrap();
                }
            }
        }
    }
}

impl Handles<ShipDistanceUpdatedEvent> for DisplayActor {
    fn handle(&mut self, ev: ShipDistanceUpdatedEvent, ctx: &mut Context) {
        ctx.lcd.set_cursor_pos(0, 0);
        ctx.lcd
            .write_fmt(format_args!("{} km", ev.distance))
            .unwrap();
    }
}

impl Handles<HullHealthUpdatedEvent> for DisplayActor {
    fn handle(&mut self, ev: HullHealthUpdatedEvent, ctx: &mut Context) {
        ctx.lcd.set_cursor_pos(0, 16);
        ctx.lcd
            .write_fmt(format_args!("{: >3}%", ev.health))
            .unwrap();
    }
}

impl Handles<NewDirectiveEvent> for DisplayActor {
    fn handle(&mut self, ev: NewDirectiveEvent, ctx: &mut Context) {
        let (command_text_1, command_text_2) = get_action_text(ev.directive.action);

        ctx.lcd.set_cursor_pos(2, 0);
        ctx.lcd.write_str(command_text_1).unwrap();
        ctx.lcd.set_cursor_pos(3, 0);
        ctx.lcd.write_str(command_text_2).unwrap();

        self.directive_time_span = Some(TimeSpan::new(ctx.now, ev.directive.time_limit));
        self.current_blocks = Some(20);
    }
}

impl Handles<DirectiveCompletedEvent> for DisplayActor {
    fn handle(&mut self, _: DirectiveCompletedEvent, ctx: &mut Context) {
        ctx.lcd.set_cursor_pos(1, 0);
        ctx.lcd.write_str(BLANK_LINE).unwrap();
        ctx.lcd.set_cursor_pos(2, 0);
        ctx.lcd.write_str(BLANK_LINE).unwrap();
        ctx.lcd.set_cursor_pos(3, 0);
        ctx.lcd.write_str(BLANK_LINE).unwrap();
        self.directive_time_span = None;
        self.current_blocks = None;
    }
}
