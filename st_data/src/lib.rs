#![cfg_attr(not(test), no_std)]

pub mod controls;

use crate::controls::{FourSwitch, ToggleSwitch, VentControl};

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
