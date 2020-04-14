mod control_init_state;
mod playing_state;

mod controls;
mod ship_actions;

use crate::common::*;
use control_init_state::{ControlInitReadyState, ControlInitState};
use playing_state::PlayingState;

enum States {
    Transition,
    Startup,
    ControlInit(ControlInitState),
    Playing(PlayingState),
}

pub struct DirectivesActor {
    state: States,
}

impl DirectivesActor {
    fn replace_state<F>(&mut self, f: F)
    where
        F: Fn(States) -> States,
    {
        let mut old_state = States::Transition;
        core::mem::swap(&mut self.state, &mut old_state);

        let mut new_state = f(old_state);
        core::mem::swap(&mut self.state, &mut new_state);
    }
}

impl Default for DirectivesActor {
    fn default() -> DirectivesActor {
        DirectivesActor {
            state: States::Startup,
        }
    }
}

impl Handles<InitializeGameEvent> for DirectivesActor {
    fn handle(&mut self, _: InitializeGameEvent, _: &mut Context) {
        self.state = States::ControlInit(ControlInitState::default());
    }
}

impl Handles<TickEvent> for DirectivesActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        match &mut self.state {
            States::Playing(s) => s.handle_tick(ctx),
            _ => (),
        }
    }
}

impl Handles<ControlInitReportedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ControlInitReportedEvent, ctx: &mut Context) {
        let ready_state = match &mut self.state {
            States::ControlInit(s) => s.handle_report(ev.action),
            _ => ControlInitReadyState::NotReady,
        };

        if ready_state == ControlInitReadyState::Ready {
            ctx.send(ControlInitFinishedEvent {});
        }
    }
}

impl Handles<GameStartedEvent> for DirectivesActor {
    fn handle(&mut self, ev: GameStartedEvent, ctx: &mut Context) {
        self.replace_state(|old_state| {
            if let States::ControlInit(s) = old_state {
                let ship_state = s.finish();
                States::Playing(PlayingState::new(
                    ev.random_seed as u64,
                    ship_state,
                    ctx.now(),
                ))
            } else {
                old_state
            }
        });
    }
}

impl Handles<ActionPerformedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ActionPerformedEvent, ctx: &mut Context) {
        match &mut self.state {
            States::ControlInit(s) => s.handle_action_performed(ev.action),
            States::Playing(s) => s.handle_action_performed(ev.action, ctx),
            _ => (),
        };
    }
}
