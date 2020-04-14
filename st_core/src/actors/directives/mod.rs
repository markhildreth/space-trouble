mod control_init_state;
mod playing_state;

mod controls;
mod ship_actions;

use crate::common::*;
use control_init_state::ControlInitState;
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
    fn handle(&mut self, ev: TickEvent, ctx: &mut Context) {
        let old_state = self.state.take().unwrap();
        let new_state = match old_state {
            States::Playing(s) => s.handle_tick(ev, ctx).into(),
            _ => old_state,
        };
        self.state.replace(new_state);
    }
}

impl Handles<ControlInitReportedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ControlInitReportedEvent, ctx: &mut Context) {
        let old_state = self.state.take().unwrap();
        let new_state = match old_state {
            States::ControlInit(s) => s.handle_report_init_control_value(ev, ctx),
            _ => old_state,
        };
        self.state.replace(new_state);
    }
}

impl Handles<ActionPerformedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ActionPerformedEvent, ctx: &mut Context) {
        let old_state = self.state.take().unwrap();
        let new_state = match old_state {
            States::ControlInit(s) => s.handle_action_performed(ev, ctx),
            States::Playing(s) => s.handle_action_performed(ev, ctx),
        };
        self.state.replace(new_state);
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
