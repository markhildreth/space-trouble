use crate::ship_distance::{ShipDistance, ShipDistanceResult};
use crate::ShipState;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use st_common::time::*;
use st_common::{Action, Directive, Event, EventQueue};

const DIRECTIVE_WAIT: Duration = Duration::from_millis(500);
const DIRECTIVE_TIME_LIMIT: Duration = Duration::from_secs(7);

pub struct Game {
    rng: SmallRng,
    ship_state: ShipState,
    ship_distance: ShipDistance,
    hull_health: u8,
    directive: CurrentDirective,
}

enum CurrentDirective {
    WaitingForDirective { wait_until: Instant },
    OutstandingDirective { expires_at: Instant, action: Action },
}

impl Game {
    pub fn new() -> Game {
        Game {
            rng: SmallRng::seed_from_u64(0x12345678),
            ship_state: ShipState::default(),
            ship_distance: ShipDistance::new(),
            hull_health: 100,
            directive: CurrentDirective::WaitingForDirective {
                // TODO: This is a bit funky.
                wait_until: Instant::from_millis(0) + DIRECTIVE_WAIT,
            },
        }
    }

    pub fn update(&mut self, now: Instant, queue: &mut EventQueue) {
        match self.directive {
            CurrentDirective::WaitingForDirective { wait_until } => {
                if now >= wait_until {
                    self.generate_directive(now, queue);
                }
            }
            CurrentDirective::OutstandingDirective { expires_at, action } => {
                if now >= expires_at {
                    self.fail_directive(now, action, queue);
                }
            }
        }

        if let ShipDistanceResult::DistanceUpdated(new_distance) = self.ship_distance.update(now) {
            let message = Event::ShipDistanceUpdated(new_distance);
            queue.enqueue(message).unwrap();
        }
    }

    pub fn perform(&mut self, now: Instant, performed_action: Action, queue: &mut EventQueue) {
        self.ship_state.perform(performed_action);

        let mut valid = false;

        if let CurrentDirective::OutstandingDirective { action, .. } = self.directive {
            if action == performed_action {
                valid = true;
                queue.enqueue(Event::DirectiveCompleted).unwrap();
                self.directive = CurrentDirective::WaitingForDirective {
                    wait_until: now + DIRECTIVE_WAIT,
                }
            }
        }

        if !valid {
            self.update_hull_health(-2, queue);
        }
    }

    fn generate_directive(&mut self, now: Instant, queue: &mut EventQueue) {
        if let Ok(action) = self.ship_state.generate_action(&mut self.rng) {
            let directive = Directive {
                action,
                time_limit: DIRECTIVE_TIME_LIMIT,
            };
            queue.enqueue(Event::NewDirective(directive)).unwrap();
            self.directive = CurrentDirective::OutstandingDirective {
                action,
                expires_at: now + directive.time_limit,
            };
        }
    }

    fn fail_directive(&mut self, now: Instant, action: Action, queue: &mut EventQueue) {
        self.ship_state.clear(action);
        self.directive = CurrentDirective::WaitingForDirective {
            wait_until: now + DIRECTIVE_WAIT,
        };
        self.update_hull_health(-4, queue);
    }

    fn update_hull_health(&mut self, update: i16, queue: &mut EventQueue) {
        self.hull_health = (self.hull_health as i16 + update) as u8;
        queue
            .enqueue(Event::HullHealthUpdated(self.hull_health))
            .unwrap();
    }
}
