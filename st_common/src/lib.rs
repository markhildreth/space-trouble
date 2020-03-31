#![cfg_attr(not(test), no_std)]

pub mod control_values;
pub mod messaging;
pub mod time;

use messaging::*;
use time::*;

use crate::control_values::{FourSwitchValue, ToggleSwitchValue, VentControlValue};
use heapless::consts::*;
use heapless::spsc::Queue;

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
pub struct TickEvent {
    pub now: Instant,
}

impl Message for TickEvent {}

pub type EventQueue = Queue<Event, U8>;

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

pub trait LCD: Sized + core::fmt::Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}
