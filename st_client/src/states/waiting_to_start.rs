use crate::states::StateUpdate;
use crate::{Components, ComponentsDef};
use st_data::time::Instant;
use st_data::GameMessage;

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
        now: Instant,
    ) -> Option<StateUpdate> {
        match self.input_found {
            true => Some(StateUpdate::GameState),
            false => None,
        }
    }

    pub fn handle(&mut self, now: Instant, msg: GameMessage) {}
}
