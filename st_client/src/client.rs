use crate::states::GameState;
use crate::{Panel, LCD};
use st_data::time::Instant;
use st_data::{ClientMessageProducer, GameMessage};

enum ClientState<'a, TPanel, TLCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    GameState(GameState<'a, TPanel, TLCD>),
}

pub struct Client<'a, TPanel: Panel, TLCD: LCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    state: ClientState<'a, TPanel, TLCD>,
}

impl<'a, TPanel: Panel, TLCD: LCD> Client<'_, TPanel, TLCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    pub fn new(
        producer: ClientMessageProducer<'a>,
        panel: TPanel,
        lcd: TLCD,
    ) -> Client<TPanel, TLCD> {
        Client {
            state: ClientState::GameState(GameState::new(producer, panel, lcd)),
        }
    }

    pub fn update(&mut self, now: Instant) {
        match &mut self.state {
            ClientState::GameState(state) => state.update(now),
        }
    }

    pub fn handle(&mut self, now: Instant, msg: GameMessage) {
        match &mut self.state {
            ClientState::GameState(state) => state.handle(now, msg),
        }
    }
}
