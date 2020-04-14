mod play_display;
mod strings;
mod time_span;

use crate::common::*;
use crate::device::LCD;
use play_display::PlayDisplay;
use strings::get_action_text;
use time_span::{SpanStatus, TimeSpan};

fn calc_blocks(remaining: Duration, total: Duration) -> u8 {
    let numerator = (20 * remaining.as_millis()) - 1;
    let denominator = total.as_millis();
    (numerator / denominator) as u8 + 1
}

struct PlayingState<T: LCD> {
    display: PlayDisplay<T>,
    directive_time_span: Option<TimeSpan>,
}

impl<T: LCD> PlayingState<T> {
    fn new(mut lcd: T) -> PlayingState<T> {
        lcd.clear();
        PlayingState {
            display: PlayDisplay::new(lcd),
            directive_time_span: None,
        }
    }

    fn tick(&mut self, now: Instant) {
        match self
            .directive_time_span
            .as_ref()
            .map(|span| span.status(now))
        {
            None => (),
            Some(SpanStatus::Ongoing { remaining, total }) => {
                let blocks = calc_blocks(remaining, total);
                self.display.update_countdown(blocks);
            }
            Some(SpanStatus::Completed) => {
                self.clear_directive();
            }
        }
    }

    fn update_ship_distance(&mut self, distance: u32) {
        self.display.update_ship_distance(distance);
    }

    fn update_ship_hull_health(&mut self, hull_health: u8) {
        self.display.update_ship_hull_health(hull_health);
    }

    fn new_directive(&mut self, directive: Directive, now: Instant) {
        self.directive_time_span = Some(TimeSpan::new(now, directive.time_limit));

        let (text1, text2) = get_action_text(directive.action);
        self.display.display_directive(text1, text2);
    }

    fn clear_directive(&mut self) {
        self.display.clear_directive();
        self.directive_time_span = None;
    }

    fn unwrap(self) -> T {
        self.display.unwrap()
    }
}

enum State<T: LCD> {
    Transition,
    WaitingForInput { lcd: T },
    Initializing { lcd: T },
    Playing(PlayingState<T>),
    GameOver { lcd: T },
}

impl<T: LCD> State<T> {
    fn new(mut lcd: T) -> State<T> {
        lcd.clear();
        lcd.set_cursor_pos(1, 0);
        lcd.write_str("  Press any button  ").unwrap();
        lcd.set_cursor_pos(2, 0);
        lcd.write_str("      to begin      ").unwrap();
        State::WaitingForInput { lcd }
    }

    fn replace<F>(&mut self, f: F)
    where
        F: Fn(T) -> State<T>,
    {
        let mut temp = State::Transition;
        core::mem::swap(self, &mut temp);
        temp = match temp {
            State::Transition => unreachable!(),
            State::WaitingForInput { lcd } => f(lcd),
            State::Initializing { lcd } => f(lcd),
            State::Playing(s) => f(s.unwrap()),
            State::GameOver { lcd } => f(lcd),
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

impl<T: LCD> Handles<InitializeGameEvent> for DisplayActor<T> {
    fn handle(&mut self, _: InitializeGameEvent, _: &mut Context) {
        self.state.replace(|mut lcd| {
            lcd.clear();
            lcd.write_str("Initializing...").unwrap();
            State::Initializing { lcd }
        });
    }
}

impl<T: LCD> Handles<GameStartedEvent> for DisplayActor<T> {
    fn handle(&mut self, _: GameStartedEvent, _: &mut Context) {
        self.state
            .replace(|lcd| State::Playing(PlayingState::new(lcd)));
    }
}

impl<T: LCD> Handles<TickEvent> for DisplayActor<T> {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        if let State::Playing(s) = &mut self.state {
            s.tick(ctx.now());
        }
    }
}

impl<T: LCD> Handles<ShipDistanceUpdatedEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: ShipDistanceUpdatedEvent, _: &mut Context) {
        if let State::Playing(s) = &mut self.state {
            s.update_ship_distance(ev.distance);
        }
    }
}

impl<T: LCD> Handles<HullHealthUpdatedEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: HullHealthUpdatedEvent, _: &mut Context) {
        if let State::Playing(s) = &mut self.state {
            s.update_ship_hull_health(ev.health);
        }
    }
}

impl<T: LCD> Handles<NewDirectiveEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: NewDirectiveEvent, ctx: &mut Context) {
        if let State::Playing(s) = &mut self.state {
            s.new_directive(ev.directive, ctx.now());
        }
    }
}

impl<T: LCD> Handles<DirectiveCompletedEvent> for DisplayActor<T> {
    fn handle(&mut self, _: DirectiveCompletedEvent, _: &mut Context) {
        if let State::Playing(s) = &mut self.state {
            s.clear_directive();
        }
    }
}

impl<T: LCD> Handles<GameEndedEvent> for DisplayActor<T> {
    fn handle(&mut self, ev: GameEndedEvent, _: &mut Context) {
        self.state.replace(|mut lcd| {
            lcd.clear();
            lcd.set_cursor_pos(1, 0);
            lcd.write_str("     Game Over      ").unwrap();
            lcd.set_cursor_pos(2, 0);
            lcd.write_fmt(format_args!("Distance: {: >8}km", ev.distance_traveled))
                .unwrap();
            State::GameOver { lcd }
        });
    }
}
