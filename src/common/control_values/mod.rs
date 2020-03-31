mod four_switch;
mod toggle_switch;
mod vent_control;

pub use four_switch::FourSwitchValue;
pub use toggle_switch::ToggleSwitchValue;
pub use vent_control::VentControlValue;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PushButtonValue {
    NotPushed,
    Pushed,
}

impl Default for PushButtonValue {
    fn default() -> PushButtonValue {
        PushButtonValue::NotPushed
    }
}

pub trait EnumFill
where
    Self: core::marker::Sized,
{
    fn fill(vec: &mut heapless::Vec<Self, heapless::consts::U4>);
}
