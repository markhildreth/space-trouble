use super::ship_actions::ShipActions;

use crate::common::*;

#[derive(PartialEq)]
pub(super) enum ControlInitReadyState {
    Ready,
    NotReady,
}

#[derive(Default)]
pub(super) struct ControlInitState {
    ship_actions: ShipActions,
    received_eigenthrottle: bool,
    received_gelatinous_darkbucket: bool,
    received_newtonian_fibermist: bool,
}

impl ControlInitState {
    pub(super) fn handle_report(&mut self, action: Action) -> ControlInitReadyState {
        self.ship_actions.perform(action);
        match action {
            Action::Eigenthrottle(_) => self.received_eigenthrottle = true,
            Action::GelatinousDarkbucket(_) => self.received_gelatinous_darkbucket = true,
            Action::NewtonianFibermist(_) => self.received_newtonian_fibermist = true,
            _ => (),
        };

        if self.received_eigenthrottle
            && self.received_gelatinous_darkbucket
            && self.received_newtonian_fibermist
        {
            ControlInitReadyState::Ready
        } else {
            ControlInitReadyState::NotReady
        }
    }

    pub(super) fn handle_action_performed(&mut self, action: Action) {
        self.ship_actions.perform(action);
    }

    pub(super) fn finish(self) -> ShipActions {
        self.ship_actions
    }
}
