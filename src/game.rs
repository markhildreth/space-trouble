use crate::messages::{Action, ClientMessages, Directive, Messages};
use crate::queue::IncomingProducer;
use crate::ship_state::ShipState;

use rand::rngs::SmallRng;
use rand::SeedableRng;

const TIME_BETWEEN_DIRECTIVES: u32 = 2_000;
const SHIP_DISTANCE_UPDATE: u32 = 2_000;
const SHIP_DISTANCE_PER_UPDATE: u32 = 297;

enum DirectiveStatus {
    AwaitingDirective {
        wait_until: u32,
    },
    HasDirective {
        expiration: u32,
        directive: Directive,
    },
}

impl DirectiveStatus {
    pub fn reset(ms: u32) -> DirectiveStatus {
        DirectiveStatus::AwaitingDirective {
            wait_until: ms + TIME_BETWEEN_DIRECTIVES,
        }
    }

    pub fn requires(&self, action: Action) -> bool {
        match self {
            DirectiveStatus::AwaitingDirective { .. } => false,
            DirectiveStatus::HasDirective { directive, .. } => directive.action == action,
        }
    }
}

pub struct Game<'a> {
    rng: SmallRng,
    queue: IncomingProducer<'a>,
    hull_health: u8,
    ship_distance: u32,
    next_ship_distance_update: u32,
    directive_status: DirectiveStatus,
    ship_state: ShipState,
}

impl<'a> Game<'a> {
    pub fn new(queue: IncomingProducer<'a>) -> Game {
        let rng = SmallRng::seed_from_u64(0x123456);
        Game {
            rng,
            queue,
            hull_health: 100,
            ship_distance: 0,
            next_ship_distance_update: 0,
            directive_status: DirectiveStatus::reset(0),
            ship_state: ShipState::default(),
        }
    }

    pub fn handle(&mut self, ms: u32, msg: ClientMessages) {
        match msg {
            ClientMessages::ActionPerformed(action) => {
                self.ship_state.update(action);
                if self.directive_status.requires(action) {
                    self.directive_status = DirectiveStatus::reset(ms);
                    self.queue.enqueue(Messages::DirectiveComplete).unwrap();
                } else {
                    self.update_hull_health(-2);
                }
            }
        }
    }

    pub fn update(&mut self, ms: u32) {
        match self.directive_status {
            DirectiveStatus::AwaitingDirective { wait_until } => {
                if ms > wait_until {
                    let directive = self.generate_directive();
                    self.directive_status = DirectiveStatus::HasDirective {
                        directive,
                        expiration: ms + directive.time_ms,
                    };
                    self.queue
                        .enqueue(Messages::NewDirective(directive))
                        .unwrap();
                }
            }
            DirectiveStatus::HasDirective { expiration, .. } => {
                if ms > expiration {
                    self.directive_status = DirectiveStatus::reset(ms);
                    self.update_hull_health(-4);
                }
            }
        }

        if ms > self.next_ship_distance_update {
            self.ship_distance += SHIP_DISTANCE_PER_UPDATE;
            self.queue
                .enqueue(Messages::UpdateDistance(self.ship_distance))
                .unwrap();
            self.next_ship_distance_update += SHIP_DISTANCE_UPDATE;
        }
    }

    fn generate_directive(&mut self) -> Directive {
        let action = self.ship_state.generate_action_needed(&mut self.rng);
        Directive {
            action,
            time_ms: 10_000,
        }
    }

    fn update_hull_health(&mut self, change: i8) {
        self.hull_health = (self.hull_health as i16 + change as i16) as u8;
        self.queue
            .enqueue(Messages::UpdateHullHealth(self.hull_health))
            .unwrap();
    }
}
