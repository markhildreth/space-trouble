#![cfg_attr(not(test), no_std)]

pub mod options;
mod ship_state;

use options::{FourSwitch, ToggleSwitch, VentControl};
pub use ship_state::ShipState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Eigenthrottle(ToggleSwitch),
    GelatinousDarkbucket(ToggleSwitch),
    VentControl(VentControl),
    NewtonianFibermist(FourSwitch),
}
