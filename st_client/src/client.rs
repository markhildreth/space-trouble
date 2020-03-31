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
            state: ClientState::GameState(GameState::new()),
        }
    }

    pub fn handle(
        &mut self,
        now: Instant,
        ev: Event,
        producer: &mut EventQueueProducer,
        panel: &mut impl Panel,
        lcd: &mut impl LCD,
    ) {
        match &mut self.state {
            ClientState::GameState(s) => s.handle(now, ev, producer, panel, lcd),
        }
    }
}
