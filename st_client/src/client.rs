use crate::states::*;
use st_common::time::*;
use st_common::*;

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
        producer: &mut EventQueue,
        lcd: &mut impl LCD,
    ) {
        match &mut self.state {
            ClientState::GameState(s) => s.handle(now, ev, producer, lcd),
        }
    }
}
