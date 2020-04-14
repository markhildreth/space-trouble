mod control_init_state;
mod playing_state;

mod controls;
mod ship_actions;

use crate::common::*;
use control_init_state::{ControlInitReadyState, ControlInitState};
use playing_state::PlayingState;

// TODO: The boilerplate code below could be done away with using
// macros most likely.

enum States {
    ControlInit(ControlInitState),
    Playing(PlayingState),
}

pub struct DirectivesActor {
    state: Option<States>,
}

impl Default for DirectivesActor {
    fn default() -> DirectivesActor {
        DirectivesActor {
            state: Some(States::ControlInit(ControlInitState::default())),
        }
    }
}

impl Handles<TickEvent> for DirectivesActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        match &mut self.state {
            Some(States::Playing(s)) => s.handle_tick(ctx),
            _ => (),
        }
    }
}

impl Handles<ControlInitReportedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ControlInitReportedEvent, ctx: &mut Context) {
        let ready_state = match &mut self.state {
            Some(States::ControlInit(s)) => s.handle_report(ev.action),
            _ => ControlInitReadyState::NotReady,
        };

        if ready_state == ControlInitReadyState::Ready {
            ctx.send(ControlInitFinishedEvent {});
        }
    }
}

impl Handles<GameStartedEvent> for DirectivesActor {
    fn handle(&mut self, _: GameStartedEvent, ctx: &mut Context) {
        let old_state = self.state.take();

        let new_state = match old_state {
            Some(States::ControlInit(s)) => {
                let ship_state = s.finish();
                Some(States::Playing(PlayingState::new(
                    0x1234_5678,
                    ship_state,
                    ctx.now(),
                )))
            }
            _ => old_state,
        };
        self.state = new_state;
    }
}

impl Handles<ActionPerformedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ActionPerformedEvent, ctx: &mut Context) {
        match &mut self.state {
            Some(States::ControlInit(s)) => s.handle_action_performed(ev.action),
            Some(States::Playing(s)) => s.handle_action_performed(ev.action, ctx),
            _ => (),
        };
    }
}

impl From<PlayingState> for States {
    fn from(s: PlayingState) -> States {
        States::Playing(s)
    }
}

impl From<ControlInitState> for States {
    fn from(s: ControlInitState) -> States {
        States::ControlInit(s)
    }
}
