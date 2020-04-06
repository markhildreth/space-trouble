use super::playing_state::PlayingState;
use super::ship_actions::ShipActions;
use super::States;

use crate::common::*;

#[derive(Default)]
pub struct ControlInitState {
    ship_actions: ShipActions,
    received_eigenthrottle: bool,
    received_gelatinous_darkbucket: bool,
    received_newtonian_fibermist: bool,
}

impl ControlInitState {
    pub(super) fn handle_report_init_control_value(
        mut self,
        ev: ReportInitControlValueEvent,
        ctx: &mut Context,
    ) -> States {
        self.ship_actions.perform(ev.action);
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
            PlayingState::new(0x1234_5678, self.ship_actions, ctx.now()).into()
        } else {
            self.into()
        }
    }

    pub(super) fn handle_action_performed(
        mut self,
        ev: ActionPerformedEvent,
        _ctx: &mut Context,
    ) -> States {
        self.ship_actions.perform(ev.action);
        self.into()
    }
}
