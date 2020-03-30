use crate::states::StateUpdate;
use crate::{Components, ComponentsDef};
use st_common::time::Instant;
use st_common::GameMessage;

pub(crate) struct WaitingToStartState {
    input_found: bool,
}

impl WaitingToStartState {
    pub fn new() -> WaitingToStartState {
        Self { input_found: false }
    }

    pub fn update<CD: ComponentsDef>(
        &mut self,
        _c: &mut Components<CD>,
        _now: Instant,
    ) -> Option<StateUpdate> {
        match self.input_found {
            true => Some(StateUpdate::GameState),
            false => None,
        }
    }

    pub fn handle(&mut self, _now: Instant, _msg: GameMessage) {}
}
