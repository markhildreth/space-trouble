mod strings;
mod time_span;

use crate::common::*;
use crate::device::LCD;
use strings::get_action_text;
use time_span::{SpanStatus, TimeSpan};

const BLANK_LINE: &str = "                    ";
const BLANK: char = ' ';

// The real LCD needs block to be 0xff. But in testing,
// this doesn't actually show up as anything. So we'll
// replace it with an asterisk for testing purposes.
#[cfg(not(test))]
const BLOCK: char = 0xff as char;
#[cfg(test)]
const BLOCK: char = '*';

fn calc_blocks(remaining: Duration, total: Duration) -> u8 {
    let numerator = (20 * remaining.as_millis()) - 1;
    let denominator = total.as_millis();
    (numerator / denominator) as u8 + 1
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
        if self.current_blocks == Some(blocks) {
            return;
        }

        self.lcd.set_cursor_pos(1, 0);
        for _ in 0..blocks {
            self.lcd.write_char(BLOCK).unwrap();
        }

        for _ in blocks..20 {
            self.lcd.write_char(BLANK).unwrap();
        }

        self.current_blocks = Some(blocks);
    }
}

impl<T: LCD> Handles<InitGameEvent> for DisplayActor<T> {
    fn handle(&mut self, _: InitGameEvent, _: &mut Context) {
        self.lcd.set_cursor_pos(1, 5);
        self.lcd.write_str("Initializing...").unwrap();
    }
}

impl<T: LCD> Handles<GameStartedEvent> for DisplayActor<T> {
    fn handle(&mut self, _: GameStartedEvent, _: &mut Context) {
        self.lcd.set_cursor_pos(1, 0);
        self.lcd.write_str(BLANK_LINE).unwrap();
        self.lcd.set_cursor_pos(0, 0);
        self.lcd.write_str("0 km").unwrap();
        self.lcd.set_cursor_pos(0, 10);
        self.lcd.write_str("Hull: 100%").unwrap();
    }
}

impl<T: LCD> Handles<TickEvent> for DisplayActor<T> {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        let status_fn = |span: &TimeSpan| span.status(ctx.now());
        match self.directive_time_span.as_ref().map(status_fn) {
            None => (),
            Some(SpanStatus::Ongoing { remaining, total }) => {
                let blocks = calc_blocks(remaining, total);
                self.update_blocks(blocks);
            }
            Some(SpanStatus::Completed) => {
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

        self.directive_time_span = Some(TimeSpan::new(
            ctx.now() + ev.directive.time_limit,
            ev.directive.time_limit,
        ));
        self.update_blocks(20);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::TestLCD;

    fn ms(x: u32) -> Instant {
        Instant::from_millis(x)
    }

    #[test]
    fn starts_clean() {
        let (lcd, screen) = TestLCD::new().split();
        let mut actor = DisplayActor::new(lcd);
        let mut ctx = Context::new(EventsQueue::new(), ms(0));
        actor.handle(GameStartedEvent {}, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }

    #[test]
    fn can_display_health() {
        let (lcd, screen) = TestLCD::new().split();
        let mut actor = DisplayActor::new(lcd);
        let events = EventsQueue::new();
        let mut ctx = Context::new(events, ms(0));

        actor.handle(GameStartedEvent {}, &mut ctx);
        actor.handle(HullHealthUpdatedEvent { health: 98 }, &mut ctx);
        screen.assert([
            "0 km      Hull:  98%",
            "                    ",
            "                    ",
            "                    ",
        ]);
        actor.handle(HullHealthUpdatedEvent { health: 2 }, &mut ctx);
        screen.assert([
            "0 km      Hull:   2%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }

    #[test]
    fn can_display_ship_distance() {
        let (lcd, screen) = TestLCD::new().split();
        let mut actor = DisplayActor::new(lcd);
        let mut ctx = Context::new(EventsQueue::new(), ms(0));

        actor.handle(GameStartedEvent {}, &mut ctx);
        actor.handle(ShipDistanceUpdatedEvent { distance: 9 }, &mut ctx);
        screen.assert([
            "9 km      Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        actor.handle(ShipDistanceUpdatedEvent { distance: 19 }, &mut ctx);
        screen.assert([
            "19 km     Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        actor.handle(ShipDistanceUpdatedEvent { distance: 199 }, &mut ctx);
        screen.assert([
            "199 km    Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        actor.handle(ShipDistanceUpdatedEvent { distance: 1999 }, &mut ctx);
        screen.assert([
            "1999 km   Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);

        actor.handle(ShipDistanceUpdatedEvent { distance: 19999 }, &mut ctx);
        screen.assert([
            "19999 km  Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }

    #[test]
    fn can_display_directive_and_countdown() {
        let (lcd, screen) = TestLCD::new().split();
        let mut actor = DisplayActor::new(lcd);
        let mut ctx = Context::new(EventsQueue::new(), ms(0));

        actor.handle(GameStartedEvent {}, &mut ctx);

        let ev = NewDirectiveEvent {
            directive: Directive {
                action: Action::Eigenthrottle(ToggleSwitchValue::Enabled),
                time_limit: Duration::from_secs(10),
            },
        };
        actor.handle(ev, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "********************",
            "      Enable        ",
            "   Eigenthrottle    ",
        ]);

        ctx.update_now(ms(1));
        actor.handle(TickEvent {}, &mut ctx);
        // No change
        screen.assert([
            "0 km      Hull: 100%",
            "********************",
            "      Enable        ",
            "   Eigenthrottle    ",
        ]);

        ctx.update_now(ms(500));
        actor.handle(TickEvent {}, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "******************* ",
            "      Enable        ",
            "   Eigenthrottle    ",
        ]);

        // Almost half way
        ctx.update_now(ms(4999));
        actor.handle(TickEvent {}, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "***********         ",
            "      Enable        ",
            "   Eigenthrottle    ",
        ]);

        // Half way
        ctx.update_now(ms(5000));
        actor.handle(TickEvent {}, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "**********          ",
            "      Enable        ",
            "   Eigenthrottle    ",
        ]);

        // Almost gone
        ctx.update_now(ms(9999));
        actor.handle(TickEvent {}, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "*                   ",
            "      Enable        ",
            "   Eigenthrottle    ",
        ]);

        // Gone
        ctx.update_now(ms(10_000));
        actor.handle(TickEvent {}, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }

    #[test]
    fn clears_screen_upon_directed_finished() {
        let (lcd, screen) = TestLCD::new().split();
        let mut actor = DisplayActor::new(lcd);
        let mut ctx = Context::new(EventsQueue::new(), ms(0));

        actor.handle(GameStartedEvent {}, &mut ctx);

        let ev = NewDirectiveEvent {
            directive: Directive {
                action: Action::Eigenthrottle(ToggleSwitchValue::Enabled),
                time_limit: Duration::from_secs(10),
            },
        };
        actor.handle(ev, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "********************",
            "      Enable        ",
            "   Eigenthrottle    ",
        ]);

        actor.handle(DirectiveCompletedEvent {}, &mut ctx);
        screen.assert([
            "0 km      Hull: 100%",
            "                    ",
            "                    ",
            "                    ",
        ]);
    }
}
