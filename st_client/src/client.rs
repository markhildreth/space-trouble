use crate::states::GameState;
use crate::{ClientComponents, Components, ComponentsDefImpl};
use crate::{Panel, LCD};
use st_data::time::Instant;
use st_data::GameMessage;

enum ClientState {
    GameState(GameState),
}

pub struct Client<'a, TPanel, TLCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    components: Components<'a, ComponentsDefImpl<TPanel, TLCD>>,
    state: ClientState,
}

impl<'a, TPanel, TLCD> Client<'a, TPanel, TLCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    pub fn new(client_components: ClientComponents<'a, TPanel, TLCD>) -> Client<'a, TPanel, TLCD> {
        let components = Components {
            panel: client_components.panel,
            lcd: client_components.lcd,
            producer: client_components.producer,
        };

        Client {
            components,
            state: ClientState::GameState(GameState::new()),
        }
    }

    pub fn update(&mut self, now: Instant) {
        match &mut self.state {
            ClientState::GameState(state) => state.update(&mut self.components, now),
        }
    }

    pub fn handle(&mut self, now: Instant, msg: GameMessage) {
        match &mut self.state {
            ClientState::GameState(state) => state.handle(now, msg),
        }
    }
}
