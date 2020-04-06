mod awaiting_control_values_state;
mod playing_state;

mod controls;
mod ship_state;

use crate::common::*;
use awaiting_control_values_state::AwaitingControlValuesState;
use playing_state::PlayingState;

enum States {
    AwaitingControlValues(AwaitingControlValuesState),
    Playing(PlayingState),
}

pub struct DirectivesActor {
    state: Option<States>,
}

impl Default for DirectivesActor {
    fn default() -> DirectivesActor {
        DirectivesActor {
            state: Some(States::AwaitingControlValues(
                AwaitingControlValuesState::default(),
            )),
        }
    }
}

impl Handles<TickEvent> for DirectivesActor {
    fn handle(&mut self, ev: TickEvent, ctx: &mut Context) {
        let old_state = self.state.take().unwrap();
        self.state.replace(match old_state {
            States::Playing(s) => s.handle_tick(ev, ctx),
            _ => old_state,
        });
    }
}

impl Handles<ReportInitialControlStateEvent> for DirectivesActor {
    fn handle(&mut self, ev: ReportInitialControlStateEvent, ctx: &mut Context) {
        let old_state = self.state.take().unwrap();
        self.state.replace(match old_state {
            States::AwaitingControlValues(s) => s.handle_report_initial_control_state(ev, ctx),
            _ => old_state,
        });
    }
}

impl Handles<ActionPerformedEvent> for DirectivesActor {
    fn handle(&mut self, ev: ActionPerformedEvent, ctx: &mut Context) {
        let old_state = self.state.take().unwrap();
        self.state.replace(match old_state {
            States::AwaitingControlValues(s) => s.handle_action_performed(ev, ctx),
            States::Playing(s) => s.handle_action_performed(ev, ctx),
        });
    }
}
