use crate::control_values::EnumFill;
use heapless::consts::*;
use heapless::Vec;

const ALL: [FourSwitchValue; 4] = [
    FourSwitchValue::Zero,
    FourSwitchValue::One,
    FourSwitchValue::Two,
    FourSwitchValue::Three,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FourSwitchValue {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl EnumFill for FourSwitchValue {
    fn fill(vec: &mut Vec<Self, U4>) {
        vec.extend(&ALL);
    }
}
