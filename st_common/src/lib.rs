#![cfg_attr(not(test), no_std)]

pub mod control_values;
mod messaging;
pub mod time;

pub use time::*;

use crate::control_values::{FourSwitchValue, ToggleSwitchValue, VentControlValue};
use heapless::consts::*;
use heapless::spsc::{Consumer, Producer, Queue};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Tick(TickEvent),
    NewDirective(Directive),
    HullHealthUpdated(u8),
    ShipDistanceUpdated(u32),
    DirectiveCompleted,
    ActionPerformed(Action),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TickEvent;

pub type EventQueue = Queue<Event, U8>;
pub type EventQueueProducer<'a> = Producer<'a, Event, U8>;
pub type EventQueueConsumer<'a> = Consumer<'a, Event, U8>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Directive {
    pub action: Action,
    pub time_limit: Duration,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Eigenthrottle(ToggleSwitchValue),
    GelatinousDarkbucket(ToggleSwitchValue),
    VentControl(VentControlValue),
    NewtonianFibermist(FourSwitchValue),
}
