#![cfg_attr(not(test), no_std)]

mod controls;
mod game;
mod options;
mod ship_state;

pub use game::Game;
use options::{EnumFill, FourSwitch, ToggleSwitch, VentControl};
pub use ship_state::ShipState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Eigenthrottle(ToggleSwitch),
    GelatinousDarkbucket(ToggleSwitch),
    VentControl(VentControl),
    NewtonianFibermist(FourSwitch),
}

pub enum GenerateFailReason {
    NoActionsAvailable,
}

pub struct Directive {
    pub action: Action,
    pub expiration: u32,
}

use heapless::consts::U4;
use heapless::spsc::{Consumer, Producer, Queue};
pub type GameMessageQueue = Queue<GameMessage, U4>;
pub type GameMessageProducer<'a> = Producer<'a, GameMessage, U4>;
pub type GameMessageConsumer<'a> = Consumer<'a, GameMessage, U4>;

pub enum GameMessage {
    NewDirective(Directive),
    HullHealthUpdated(u32),
}
