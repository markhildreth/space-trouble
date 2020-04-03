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

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    AwaitingInitialControlValues {
        rcvd_eign: bool,
        rcvd_gel: bool,
        rcvd_newt: bool,
    },
    Ready,
}

pub struct DirectivesActor {
    rng: SmallRng,
    state: State,
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
            state: State::AwaitingInitialControlValues {
                rcvd_eign: false,
                rcvd_gel: false,
                rcvd_newt: false,
            },
            ship_state: ShipState::default(),
            directive: CurrentDirective::WaitingForDirective {
                wait_until: Instant::from_millis(0) + DIRECTIVE_WAIT,
            },
        }
    }
}

impl Handles<TickEvent> for DirectivesActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        if let State::AwaitingInitialControlValues { .. } = self.state {
            return;
        }

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

const READY_TO_START_STATE: State = State::AwaitingInitialControlValues {
    rcvd_eign: true,
    rcvd_gel: true,
    rcvd_newt: true,
};

impl Handles<ReportInitialControlStateEvent> for DirectivesActor {
    fn handle(&mut self, ev: ReportInitialControlStateEvent, ctx: &mut Context) {
        self.ship_state.perform(ev.action);
        if let State::AwaitingInitialControlValues {
            rcvd_eign,
            rcvd_gel,
            rcvd_newt,
        } = self.state
        {
            self.state = match ev.action {
                Action::Eigenthrottle(_) => State::AwaitingInitialControlValues {
                    rcvd_eign: true,
                    rcvd_gel,
                    rcvd_newt,
                },
                Action::GelatinousDarkbucket(_) => State::AwaitingInitialControlValues {
                    rcvd_eign,
                    rcvd_gel: true,
                    rcvd_newt,
                },
                Action::NewtonianFibermist(_) => State::AwaitingInitialControlValues {
                    rcvd_eign,
                    rcvd_gel,
                    rcvd_newt: true,
                },
                _ => self.state,
            };

            if self.state == READY_TO_START_STATE {
                self.state = State::Ready;
                ctx.send(ControlInitFinishedEvent {});
            }
        }
    }
}

impl Handles<ActionPerformedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ActionPerformedEvent, ctx: &mut Context) {
        self.ship_state.perform(ev.action);

        if let State::AwaitingInitialControlValues { .. } = self.state {
            return;
        }
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
