use super::playing_state::PlayingState;
use super::ship_state::ShipState;
use super::States;

use crate::common::*;

#[derive(Default)]
pub struct AwaitingControlValuesState {
    ship_state: ShipState,
    received_eigenthrottle: bool,
    received_gelatinous_darkbucket: bool,
    received_newtonian_fibermist: bool,
}

impl AwaitingControlValuesState {
    pub(super) fn handle_report_initial_control_state(
        mut self,
        ev: ReportInitialControlStateEvent,
        ctx: &mut Context,
    ) -> States {
        self.ship_state.perform(ev.action);
        match ev.action {
            Action::Eigenthrottle(_) => self.received_eigenthrottle = true,
            Action::GelatinousDarkbucket(_) => self.received_gelatinous_darkbucket = true,
            Action::NewtonianFibermist(_) => self.received_newtonian_fibermist = true,
            _ => (),
        };

        if self.received_eigenthrottle
            && self.received_gelatinous_darkbucket
            && self.received_newtonian_fibermist
        {
            ctx.send(ControlInitFinishedEvent {});
            States::Playing(PlayingState::new(0x1234_5678, self.ship_state, ctx.now()))
        } else {
            States::AwaitingControlValues(self)
        }
    }

    pub(super) fn handle_action_performed(
        mut self,
        ev: ActionPerformedEvent,
        _ctx: &mut Context,
    ) -> States {
        self.ship_state.perform(ev.action);
        States::AwaitingControlValues(self)
    }
}
