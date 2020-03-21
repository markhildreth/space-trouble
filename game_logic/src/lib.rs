#![cfg_attr(not(test), no_std)]

mod controls;
mod options;
mod ship_state;

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
