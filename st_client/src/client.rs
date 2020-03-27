use crate::states::GameState;
use crate::{ClientComponents, ComponentDef, Components};
use crate::{Panel, LCD};
use st_data::time::Instant;
use st_data::GameMessage;

enum ClientState {
    GameState(GameState),
}

pub struct Client<'a, CDef: ComponentDef> {
    components: Components<'a, CDef>,
    state: ClientState,
}

impl<'a, TPanel: Panel, TLCD: LCD, CDef: ComponentDef<Panel = TPanel, LCD = TLCD>>
    Client<'_, CDef>
{
    pub fn new(components: ClientComponents<'a, TPanel, TLCD>) -> Client<'a, CDef> {
        Client {
            components: components.into(),
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
