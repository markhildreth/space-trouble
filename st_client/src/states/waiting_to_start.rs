use crate::states::StateUpdate;
use st_common::time::Instant;
use st_common::Event;

pub(crate) struct WaitingToStartState {
    input_found: bool,
}

impl WaitingToStartState {
    pub fn new() -> WaitingToStartState {
        Self { input_found: false }
    }

    pub fn update(&mut self, _: Instant) -> Option<StateUpdate> {
        match self.input_found {
            true => Some(StateUpdate::GameState),
            false => None,
        }
    }

    pub fn handle(&mut self, _now: Instant, _: Event) {}
}
