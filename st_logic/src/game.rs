use crate::ship_distance::{ShipDistance, ShipDistanceResult};
use crate::{GameMessageProducer, ShipState};
use rand::Rng;
use st_data::{Action, Directive, GameMessage};

const DIRECTIVE_WAIT: u32 = 500;
const DIRECTIVE_EXPIRATION: u32 = 7_000;

pub struct Game<'a> {
    producer: GameMessageProducer<'a>,
    ship_state: ShipState,
    ship_distance: ShipDistance,
    hull_health: u8,
    directive: CurrentDirective,
}

enum CurrentDirective {
    WaitingForDirective { wait_until: u32 },
    OutstandingDirective { expires_at: u32, action: Action },
}

impl<'a> Game<'a> {
    pub fn new(producer: GameMessageProducer<'a>) -> Game {
        Game {
            producer,
            ship_state: ShipState::default(),
            ship_distance: ShipDistance::new(),
            hull_health: 100,
            directive: CurrentDirective::WaitingForDirective {
                wait_until: DIRECTIVE_WAIT,
            },
        }
    }

    pub fn update(&mut self, ms: u32, rng: &mut impl Rng) {
        match self.directive {
            CurrentDirective::WaitingForDirective { wait_until } => {
                if ms >= wait_until {
                    self.generate_directive(ms, rng);
                }
            }
            CurrentDirective::OutstandingDirective { expires_at, action } => {
                if ms >= expires_at {
                    self.fail_directive(ms, action);
                }
            }
        }

        if let ShipDistanceResult::DistanceUpdated(new_distance) = self.ship_distance.update(ms) {
            let message = GameMessage::ShipDistanceUpdated(new_distance);
            self.producer.enqueue(message).unwrap();
        }
    }

    pub fn perform(&mut self, ms: u32, performed_action: Action) {
        self.ship_state.perform(performed_action);

        let mut valid = false;

        if let CurrentDirective::OutstandingDirective { action, .. } = self.directive {
            if action == performed_action {
                valid = true;
                self.producer
                    .enqueue(GameMessage::DirectiveCompleted)
                    .unwrap();
                self.directive = CurrentDirective::WaitingForDirective {
                    wait_until: ms + DIRECTIVE_WAIT,
                }
            }
        }

        if !valid {
            self.update_hull_health(-2);
        }
    }

    fn generate_directive(&mut self, ms: u32, rng: &mut impl Rng) {
        if let Ok(action) = self.ship_state.generate_action(rng) {
            let directive = Directive {
                action,
                expiration: DIRECTIVE_EXPIRATION,
            };
            self.producer
                .enqueue(GameMessage::NewDirective(directive))
                .unwrap();
            self.directive = CurrentDirective::OutstandingDirective {
                action,
                expires_at: ms + directive.expiration,
            };
        }
    }

    fn fail_directive(&mut self, ms: u32, action: Action) {
        self.ship_state.clear(action);
        self.directive = CurrentDirective::WaitingForDirective {
            wait_until: ms + DIRECTIVE_WAIT,
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
