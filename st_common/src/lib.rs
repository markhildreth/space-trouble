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
    NewDirective(NewDirectiveEvent),
    HullHealthUpdated(HullHealthUpdatedEvent),
    ShipDistanceUpdated(ShipDistanceUpdatedEvent),
    DirectiveCompleted(DirectiveCompletedEvent),
    ActionPerformed(ActionPerformedEvent),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TickEvent;
impl Message for TickEvent {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NewDirectiveEvent {
    pub directive: Directive,
}
impl Message for NewDirectiveEvent {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HullHealthUpdatedEvent {
    pub health: u8,
}
impl Message for HullHealthUpdatedEvent {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ShipDistanceUpdatedEvent {
    pub distance: u32,
}
impl Message for ShipDistanceUpdatedEvent {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DirectiveCompletedEvent;
impl Message for DirectiveCompletedEvent {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActionPerformedEvent {
    pub action: Action,
}
impl Message for ActionPerformedEvent {}

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
