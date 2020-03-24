#![cfg_attr(not(test), no_std)]

pub mod controls;

use crate::controls::{FourSwitch, ToggleSwitch, VentControl};
use heapless::consts::U4;
use heapless::spsc::{Consumer, Producer, Queue};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Eigenthrottle(ToggleSwitch),
    GelatinousDarkbucket(ToggleSwitch),
    VentControl(VentControl),
    NewtonianFibermist(FourSwitch),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Directive {
    pub action: Action,
    pub expiration: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameMessage {
    NewDirective(Directive),
    HullHealthUpdated(u8),
    ShipDistanceUpdated(u32),
    DirectiveCompleted,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClientMessage {
    ActionPerformed(Action),
}

pub type ClientMessageQueue = Queue<ClientMessage, U4>;
pub type ClientMessageProducer<'a> = Producer<'a, ClientMessage, U4>;
pub type ClientMessageConsumer<'a> = Consumer<'a, ClientMessage, U4>;

pub type GameMessageQueue = Queue<GameMessage, U4>;
pub type GameMessageProducer<'a> = Producer<'a, GameMessage, U4>;
pub type GameMessageConsumer<'a> = Consumer<'a, GameMessage, U4>;
