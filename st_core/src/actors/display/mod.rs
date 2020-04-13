mod play_lcd;
mod strings;
mod time_span;

use crate::common::*;
use crate::device::LCD;
use play_lcd::PlayLCD;
use strings::get_action_text;
use time_span::{SpanStatus, TimeSpan};

fn calc_blocks(remaining: Duration, total: Duration) -> u8 {
    let numerator = (20 * remaining.as_millis()) - 1;
    let denominator = total.as_millis();
    (numerator / denominator) as u8 + 1
}

enum State<T: LCD> {
    Transition,
    WaitingForGame {
        lcd: T,
    },
    Playing {
        lcd: PlayLCD<T>,
        directive_time_span: Option<TimeSpan>,
    },
}

impl<T: LCD> State<T> {
    fn new(lcd: T) -> State<T> {
        State::WaitingForGame { lcd }
    }

    fn replace<F>(&mut self, f: F)
    where
        F: Fn(T) -> State<T>,
    {
        let mut temp = State::Transition;
        core::mem::swap(self, &mut temp);
        temp = match temp {
            State::Transition => unreachable!(),
            State::WaitingForGame { lcd } => f(lcd),
            State::Playing { lcd, .. } => f(lcd.unwrap()),
        };
        core::mem::swap(self, &mut temp);
    }
}

pub struct DisplayActor<T>
where
    T: LCD,
{
    state: State<T>,
}

impl<T> DisplayActor<T>
where
    T: LCD,
{
    pub fn new(lcd: T) -> DisplayActor<T> {
        DisplayActor {
            state: State::new(lcd),
        }
    }
}

impl<T: LCD> Handles<InitGameEvent> for DisplayActor<T> {
    fn handle(&mut self, _: InitGameEvent, _: &mut Context) {
        self.state.replace(|lcd| State::Playing {
            lcd: PlayLCD::new(lcd),
            directive_time_span: None,
        });
    }
}

impl<T: LCD> Handles<GameStartedEvent> for DisplayActor<T> {
    fn handle(&mut self, _: GameStartedEvent, _: &mut Context) {}
}

impl<T: LCD> Handles<TickEvent> for DisplayActor<T> {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        if let State::Playing {
            lcd,
            directive_time_span,
        } = &mut self.state
        {
            let status_fn = |span: &TimeSpan| span.status(ctx.now());
            match directive_time_span.as_ref().map(status_fn) {
                None => (),
                Some(SpanStatus::Ongoing { remaining, total }) => {
                    let blocks = calc_blocks(remaining, total);
                    lcd.update_countdown(blocks);
                }
                Some(SpanStatus::Completed) => {
                    lcd.clear_directive();
                    *directive_time_span = None;
                }
            }
        }
    }
}

impl<T: LCD> Handles<ShipDistanceUpdatedEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: ShipDistanceUpdatedEvent, _: &mut Context) {
        if let State::Playing { lcd, .. } = &mut self.state {
            lcd.update_ship_distance(ev.distance);
        }
    }
}

impl<T: LCD> Handles<HullHealthUpdatedEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: HullHealthUpdatedEvent, _: &mut Context) {
        if let State::Playing { lcd, .. } = &mut self.state {
            lcd.update_ship_hull_health(ev.health);
        }
    }
}

impl<T: LCD> Handles<NewDirectiveEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: NewDirectiveEvent, ctx: &mut Context) {
        if let State::Playing {
            lcd,
            directive_time_span,
        } = &mut self.state
        {
            *directive_time_span = Some(TimeSpan::new(
                ctx.now() + ev.directive.time_limit,
                ev.directive.time_limit,
            ));

            let (text1, text2) = get_action_text(ev.directive.action);
            lcd.display_directive(text1, text2);
        }
    }
}

impl<T: LCD> Handles<DirectiveCompletedEvent> for DisplayActor<T> {
    fn handle(&mut self, _: DirectiveCompletedEvent, _: &mut Context) {}
}
