#![cfg(not(test))]
#![no_std]

pub mod controls;

use controls::{FourSwitch, ToggleSwitch, VentControl};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Eigenthrottle(ToggleSwitch),
    GelatinousDarkbucket(ToggleSwitch),
    VentControl(VentControl),
    NewtonianFibermist(FourSwitch),
}
