mod game_state;
mod waiting_to_start;

pub(crate) use game_state::GameState;
pub(crate) use waiting_to_start::WaitingToStartState;

pub(crate) enum ClientState {
    WaitingToStart(WaitingToStartState),
    GameState(GameState),
}

pub(crate) enum StateUpdate {
    GameState,
}
