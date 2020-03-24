use crate::controls::EnumFill;
use heapless::consts::*;
use heapless::Vec;

const ALL: [ToggleSwitch; 2] = [ToggleSwitch::Disabled, ToggleSwitch::Enabled];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSwitch {
    Disabled,
    Enabled,
}

impl EnumFill for ToggleSwitch {
    fn fill(vec: &mut Vec<Self, U4>) {
        vec.extend(&ALL);
    }
}
