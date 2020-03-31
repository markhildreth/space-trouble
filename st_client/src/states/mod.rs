mod game_state;

pub(crate) use game_state::GameState;

pub(crate) enum ClientState {
    GameState(GameState),
}
