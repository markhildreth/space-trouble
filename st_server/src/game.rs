use crate::ship_distance::{ShipDistance, ShipDistanceResult};
use crate::ShipState;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use st_common::time::*;
use st_common::{Action, Directive, GameMessage, GameMessageProducer};

const DIRECTIVE_WAIT: Duration = Duration::from_millis(500);
const DIRECTIVE_TIME_LIMIT: Duration = Duration::from_secs(7);

pub struct Game<'a> {
    rng: SmallRng,
    producer: GameMessageProducer<'a>,
    ship_state: ShipState,
    ship_distance: ShipDistance,
    hull_health: u8,
    directive: CurrentDirective,
}

enum CurrentDirective {
    WaitingForDirective { wait_until: Instant },
    OutstandingDirective { expires_at: Instant, action: Action },
}

impl<'a> Game<'a> {
    pub fn new(producer: GameMessageProducer<'a>) -> Game {
        Game {
            rng: SmallRng::seed_from_u64(0x12345678),
            producer,
            ship_state: ShipState::default(),
            ship_distance: ShipDistance::new(),
            hull_health: 100,
            directive: CurrentDirective::WaitingForDirective {
                // TODO: This is a bit funky.
                wait_until: Instant::from_millis(0) + DIRECTIVE_WAIT,
            },
        }
    }

    pub fn update(&mut self, now: Instant) {
        match self.directive {
            CurrentDirective::WaitingForDirective { wait_until } => {
                if now >= wait_until {
                    self.generate_directive(now);
                }
            }
            CurrentDirective::OutstandingDirective { expires_at, action } => {
                if now >= expires_at {
                    self.fail_directive(now, action);
                }
            }
        }

        if let ShipDistanceResult::DistanceUpdated(new_distance) = self.ship_distance.update(now) {
            let message = GameMessage::ShipDistanceUpdated(new_distance);
            self.producer.enqueue(message).unwrap();
        }
    }

    pub fn perform(&mut self, now: Instant, performed_action: Action) {
        self.ship_state.perform(performed_action);

        let mut valid = false;

        if let CurrentDirective::OutstandingDirective { action, .. } = self.directive {
            if action == performed_action {
                valid = true;
                self.producer
                    .enqueue(GameMessage::DirectiveCompleted)
                    .unwrap();
                self.directive = CurrentDirective::WaitingForDirective {
                    wait_until: now + DIRECTIVE_WAIT,
                }
            }
        }

        if !valid {
            self.update_hull_health(-2);
        }
    }

    fn generate_directive(&mut self, now: Instant) {
        if let Ok(action) = self.ship_state.generate_action(&mut self.rng) {
            let directive = Directive {
                action,
                time_limit: DIRECTIVE_TIME_LIMIT,
            };
            self.producer
                .enqueue(GameMessage::NewDirective(directive))
                .unwrap();
            self.directive = CurrentDirective::OutstandingDirective {
                action,
                expires_at: now + directive.time_limit,
            };
        }
    }

    fn fail_directive(&mut self, now: Instant, action: Action) {
        self.ship_state.clear(action);
        self.directive = CurrentDirective::WaitingForDirective {
            wait_until: now + DIRECTIVE_WAIT,
        };
        self.update_hull_health(-4);
    }

    fn update_hull_health(&mut self, update: i16) {
        self.hull_health = (self.hull_health as i16 + update) as u8;
        self.producer
            .enqueue(GameMessage::HullHealthUpdated(self.hull_health))
            .unwrap();
    }
}
