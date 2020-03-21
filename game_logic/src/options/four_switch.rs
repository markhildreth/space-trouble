use crate::EnumFill;
use heapless::consts::*;
use heapless::Vec;

const ALL: [FourSwitch; 4] = [
    FourSwitch::Zero,
    FourSwitch::One,
    FourSwitch::Two,
    FourSwitch::Three,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FourSwitch {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl EnumFill for FourSwitch {
    fn fill(vec: &mut Vec<Self, U4>) {
        vec.extend(&ALL);
    }
}
