use crate::{Action, Directive, GameMessage, GameMessageProducer, ShipState};
use rand::Rng;

const DIRECTIVE_WAIT: u32 = 2_000;

pub struct Game<'a> {
    producer: GameMessageProducer<'a>,
    ship_state: ShipState,
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
            hull_health: 100,
            ship_state: ShipState::default(),
            directive: CurrentDirective::WaitingForDirective { wait_until: 2_000 },
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
    }

    pub fn perform(&mut self, ms: u32, action: Action) {
        self.ship_state.perform(action);
        if let CurrentDirective::OutstandingDirective {
            action: outstanding_action,
            ..
        } = self.directive
        {
            if outstanding_action == action {
                self.directive = CurrentDirective::WaitingForDirective {
                    wait_until: ms + DIRECTIVE_WAIT,
                }
            }
        }
    }

    fn generate_directive(&mut self, ms: u32, rng: &mut impl Rng) {
        if let Ok(action) = self.ship_state.generate_action(rng) {
            let directive = Directive {
                action,
                expiration: 10_000,
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
