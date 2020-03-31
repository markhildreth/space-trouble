mod controls;
mod ship_distance;
mod ship_state;

use crate::actors::Context;
use crate::common::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use ship_distance::{ShipDistance, ShipDistanceResult};
use ship_state::ShipState;

const DIRECTIVE_WAIT: Duration = Duration::from_millis(500);
const DIRECTIVE_TIME_LIMIT: Duration = Duration::from_secs(7);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GenerateFailReason {
    NoActionsAvailable,
}

pub struct GameLogicActor {
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

impl GameLogicActor {
    pub fn new() -> GameLogicActor {
        GameLogicActor {
            rng: SmallRng::seed_from_u64(0x12345678),
            ship_state: ShipState::default(),
            ship_distance: ShipDistance::new(),
            hull_health: 100,
            directive: CurrentDirective::WaitingForDirective {
                wait_until: Instant::from_millis(0) + DIRECTIVE_WAIT,
            },
        }
    }

    fn generate_directive(&mut self, now: Instant, queue: &mut EventsQueue) {
        if let Ok(action) = self.ship_state.generate_action(&mut self.rng) {
            let directive = Directive {
                action,
                time_limit: DIRECTIVE_TIME_LIMIT,
            };
            queue
                .enqueue(NewDirectiveEvent { directive }.into())
                .unwrap();
            self.directive = CurrentDirective::OutstandingDirective {
                action,
                expires_at: now + directive.time_limit,
            };
        }
    }

    fn fail_directive(&mut self, now: Instant, action: Action, queue: &mut EventsQueue) {
        self.ship_state.clear(action);
        self.directive = CurrentDirective::WaitingForDirective {
            wait_until: now + DIRECTIVE_WAIT,
        };
        self.update_hull_health(-4, queue);
    }

    fn update_hull_health(&mut self, update: i16, queue: &mut EventsQueue) {
        self.hull_health = (self.hull_health as i16 + update) as u8;
        queue
            .enqueue(
                HullHealthUpdatedEvent {
                    health: self.hull_health,
                }
                .into(),
            )
            .unwrap();
    }
}

impl Handler for GameLogicActor {
    type Context = Context;
}

impl Handles<TickEvent> for GameLogicActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        match self.directive {
            CurrentDirective::WaitingForDirective { wait_until } => {
                if ctx.now >= wait_until {
                    self.generate_directive(ctx.now, &mut ctx.queue);
                }
            }
            CurrentDirective::OutstandingDirective { expires_at, action } => {
                if ctx.now >= expires_at {
                    self.fail_directive(ctx.now, action, &mut ctx.queue);
                }
            }
        }

        if let ShipDistanceResult::DistanceUpdated(distance) = self.ship_distance.update(ctx.now) {
            let ev = ShipDistanceUpdatedEvent { distance };
            ctx.queue.enqueue(ev.into()).unwrap();
        }
    }
}

impl Handles<ActionPerformedEvent> for GameLogicActor {
    fn handle(&mut self, ev: ActionPerformedEvent, ctx: &mut Context) {
        self.ship_state.perform(ev.action);

        let mut valid = false;

        if let CurrentDirective::OutstandingDirective { action, .. } = self.directive {
            if action == ev.action {
                valid = true;
                ctx.queue
                    .enqueue(DirectiveCompletedEvent {}.into())
                    .unwrap();
                self.directive = CurrentDirective::WaitingForDirective {
                    wait_until: ctx.now + DIRECTIVE_WAIT,
                }
            }
        }

        if !valid {
            self.update_hull_health(-2, &mut ctx.queue);
        }
    }
}
