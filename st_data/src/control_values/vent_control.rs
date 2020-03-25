use crate::control_values::EnumFill;
use heapless::consts::*;
use heapless::Vec;

const ALL: [VentControlValue; 4] = [
    VentControlValue::Hydrogen,
    VentControlValue::WaterVapor,
    VentControlValue::Waste,
    VentControlValue::Frustrations,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VentControlValue {
    Hydrogen,
    WaterVapor,
    Waste,
    Frustrations,
}

impl EnumFill for VentControlValue {
    fn fill(vec: &mut Vec<Self, U4>) {
        vec.extend(&ALL);
    }
}
