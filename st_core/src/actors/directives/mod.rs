mod controls;
mod ship_state;

use crate::common::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use ship_state::ShipState;

const DIRECTIVE_WAIT: Duration = Duration::from_millis(500);
const DIRECTIVE_TIME_LIMIT: Duration = Duration::from_secs(7);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GenerateFailReason {
    NoActionsAvailable,
}

enum CurrentDirective {
    WaitingForDirective { wait_until: Instant },
    OutstandingDirective { expires_at: Instant, action: Action },
}

pub struct DirectivesActor {
    rng: SmallRng,
    ship_state: ShipState,
    directive: CurrentDirective,
}

impl DirectivesActor {
    fn generate_directive(&mut self, now: Instant) -> Result<Directive, GenerateFailReason> {
        if let Ok(action) = self.ship_state.generate_action(&mut self.rng) {
            let directive = Directive {
                action,
                time_limit: DIRECTIVE_TIME_LIMIT,
            };
            self.directive = CurrentDirective::OutstandingDirective {
                action,
                expires_at: now + directive.time_limit,
            };
            return Ok(directive);
        }
        Err(GenerateFailReason::NoActionsAvailable)
    }
}

impl Default for DirectivesActor {
    fn default() -> DirectivesActor {
        DirectivesActor {
            rng: SmallRng::seed_from_u64(0x1234_5678),
            ship_state: ShipState::default(),
            directive: CurrentDirective::WaitingForDirective {
                wait_until: Instant::from_millis(0) + DIRECTIVE_WAIT,
            },
        }
    }
}

impl Handles<TickEvent> for DirectivesActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        match self.directive {
            CurrentDirective::WaitingForDirective { wait_until } => {
                if ctx.now() >= wait_until {
                    if let Ok(directive) = self.generate_directive(ctx.now()) {
                        ctx.send(NewDirectiveEvent { directive });
                    }
                }
            }
            CurrentDirective::OutstandingDirective { expires_at, action } => {
                if ctx.now() >= expires_at {
                    self.ship_state.clear(action);
                    self.directive = CurrentDirective::WaitingForDirective {
                        wait_until: ctx.now() + DIRECTIVE_WAIT,
                    };
                    ctx.send(UpdateHullHealthEvent { delta: -4 });
                }
            }
        }
    }
}

impl Handles<ActionPerformedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ActionPerformedEvent, ctx: &mut Context) {
        self.ship_state.perform(ev.action);

        let mut valid = false;

        if let CurrentDirective::OutstandingDirective { action, .. } = self.directive {
            if action == ev.action {
                valid = true;
                ctx.send(DirectiveCompletedEvent {});
                self.directive = CurrentDirective::WaitingForDirective {
                    wait_until: ctx.now() + DIRECTIVE_WAIT,
                }
            }
        }

        if !valid {
            ctx.send(UpdateHullHealthEvent { delta: -2 })
        }
    }
}
