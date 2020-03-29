use crate::states::StateUpdate;
use crate::{Components, ComponentsDef};
use st_data::time::Instant;

pub(crate) struct WaitingToStartState {}

impl WaitingToStartState {
    pub fn update<CD: ComponentsDef>(
        &mut self,
        _c: &mut Components<CD>,
        now: Instant,
    ) -> Option<StateUpdate> {
        if now > Instant::from_millis(2000) {
            Some(StateUpdate::GameState)
        } else {
            None
        }
    }
}
