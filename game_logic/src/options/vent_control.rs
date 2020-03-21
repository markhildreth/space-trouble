use crate::EnumFill;
use heapless::consts::*;
use heapless::Vec;

const ALL: [VentControl; 4] = [
    VentControl::Hydrogen,
    VentControl::WaterVapor,
    VentControl::Waste,
    VentControl::Frustrations,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VentControl {
    Hydrogen,
    WaterVapor,
    Waste,
    Frustrations,
}

impl EnumFill for VentControl {
    fn fill(vec: &mut Vec<Self, U4>) {
        vec.extend(&ALL);
    }
}
