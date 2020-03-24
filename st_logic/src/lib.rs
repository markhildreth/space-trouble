#![cfg_attr(not(test), no_std)]

mod controls;
mod game;
mod ship_distance;
mod ship_state;

use st_data::GameMessage;

pub use game::Game;
pub use ship_state::ShipState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GenerateFailReason {
    NoActionsAvailable,
}

use heapless::consts::U4;
use heapless::spsc::{Consumer, Producer, Queue};
pub type GameMessageQueue = Queue<GameMessage, U4>;
pub type GameMessageProducer<'a> = Producer<'a, GameMessage, U4>;
pub type GameMessageConsumer<'a> = Consumer<'a, GameMessage, U4>;
