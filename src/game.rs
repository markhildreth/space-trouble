use crate::messages::{Action, Directive, Interface, Messages, Value};
use crate::queue::OutgoingProducer;

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

pub struct Game<'a> {
    queue: OutgoingProducer<'a>,
    hull_health: u8,
    ship_distance: u32,
    next_ship_distance_update: u32,
    directive_status: DirectiveStatus,
}

impl<'a> Game<'a> {
    pub fn new(queue: OutgoingProducer<'a>) -> Game {
        Game {
            queue,
            hull_health: 100,
            ship_distance: 0,
            next_ship_distance_update: 0,
            directive_status: DirectiveStatus::AwaitingDirective {
                wait_until: 0 + TIME_BETWEEN_DIRECTIVES,
            },
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
                    self.directive_status = DirectiveStatus::AwaitingDirective {
                        wait_until: ms + TIME_BETWEEN_DIRECTIVES,
                    };
                    self.hull_health -= 4;
                    self.queue
                        .enqueue(Messages::UpdateHullHealth(self.hull_health))
                        .unwrap();
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

    fn generate_directive(&self) -> Directive {
        Directive {
            action: Action {
                interface: Interface::Eigenthrottle,
                value: Value::Enable,
            },
            time_ms: 10_000,
        }
    }
}
