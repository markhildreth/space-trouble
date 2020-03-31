use crate::states::*;
use crate::{Panel, LCD};
use st_common::time::Instant;
use st_common::{Event, EventQueueProducer};

pub struct Client {
    state: ClientState,
}

impl Client {
    pub fn new() -> Self {
        Client {
            state: ClientState::WaitingToStart(WaitingToStartState::new()),
        }
    }

    pub fn update(
        &mut self,
        now: Instant,
        producer: &mut EventQueueProducer,
        panel: &mut impl Panel,
        lcd: &mut impl LCD,
    ) {
        let result = match &mut self.state {
            ClientState::WaitingToStart(s) => s.update(now),
            ClientState::GameState(s) => s.update(now, producer, panel, lcd),
        };

        if let Some(state_update) = result {
            self.state = match state_update {
                StateUpdate::GameState => ClientState::GameState(GameState::new()),
            }
        }
    }

    pub fn handle(&mut self, now: Instant, ev: Event, producer: &mut EventQueueProducer) {
        match &mut self.state {
            ClientState::WaitingToStart(s) => s.handle(now, ev),
            ClientState::GameState(s) => s.handle(now, ev),
        }
    }
}
