use crate::states::*;
use crate::{Components, ComponentsDef, ComponentsDefImpl, Panel, LCD};
use st_common::time::Instant;
use st_common::{ClientMessageProducer, GameMessage};

pub struct Client<'a, CD: ComponentsDef> {
    components: Components<'a, CD>,
    state: ClientState,
}

impl<'a, TPanel: Panel, TLCD: LCD> Client<'a, ComponentsDefImpl<TPanel, TLCD>> {
    pub fn new(producer: ClientMessageProducer<'a>, panel: TPanel, lcd: TLCD) -> Self {
        Client {
            components: Components::new(producer, panel, lcd),
            state: ClientState::WaitingToStart(WaitingToStartState::new()),
        }
    }

    pub fn update(&mut self, now: Instant) {
        let result = match &mut self.state {
            ClientState::WaitingToStart(s) => s.update(&mut self.components, now),
            ClientState::GameState(s) => s.update(&mut self.components, now),
        };

        if let Some(state_update) = result {
            self.state = match state_update {
                StateUpdate::GameState => ClientState::GameState(GameState::new()),
            }
        }
    }

    pub fn handle(&mut self, now: Instant, msg: GameMessage) {
        match &mut self.state {
            ClientState::WaitingToStart(s) => s.handle(now, msg),
            ClientState::GameState(s) => s.handle(now, msg),
        }
    }
}
