use crate::control_values::EnumFill;
use heapless::consts::*;
use heapless::Vec;

const ALL: [ToggleSwitchValue; 2] = [ToggleSwitchValue::Disabled, ToggleSwitchValue::Enabled];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSwitchValue {
    Disabled,
    Enabled,
}

impl EnumFill for ToggleSwitchValue {
    fn fill(vec: &mut Vec<Self, U4>) {
        vec.extend(&ALL);
    }
}
