use super::ship_state::{GenerateFailReason, ShipState};
use super::States;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::common::*;

const DIRECTIVE_WAIT: Duration = Duration::from_millis(500);
const DIRECTIVE_TIME_LIMIT: Duration = Duration::from_secs(7);

enum CurrentDirective {
    WaitingForDirective { wait_until: Instant },
    OutstandingDirective { expires_at: Instant, action: Action },
}

pub(super) struct PlayingState {
    rng: SmallRng,
    ship_state: ShipState,
    directive: CurrentDirective,
}

impl PlayingState {
    pub(super) fn new(rng_seed: u64, ship_state: ShipState, now: Instant) -> PlayingState {
        PlayingState {
            rng: SmallRng::seed_from_u64(rng_seed),
            ship_state,
            directive: CurrentDirective::WaitingForDirective {
                wait_until: now + DIRECTIVE_WAIT,
            },
        }
    }

    pub(super) fn handle_tick(mut self, _ev: TickEvent, ctx: &mut Context) -> States {
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

        States::Playing(self)
    }

    pub(super) fn handle_action_performed(
        mut self,
        ev: ActionPerformedEvent,
        ctx: &mut Context,
    ) -> States {
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

        States::Playing(self)
    }

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
