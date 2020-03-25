#![cfg_attr(not(test), no_std)]

mod controls;
mod game;
mod ship_distance;
mod ship_state;

pub use game::Game;
pub use ship_state::ShipState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GenerateFailReason {
    NoActionsAvailable,
}
