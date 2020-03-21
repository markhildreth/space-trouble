mod four_switch;
mod toggle_switch;
mod vent_control;

pub(crate) use four_switch::FourSwitch;
pub(crate) use toggle_switch::ToggleSwitch;
pub(crate) use vent_control::VentControl;

pub(crate) trait EnumFill
where
    Self: core::marker::Sized,
{
    fn fill(vec: &mut heapless::Vec<Self, heapless::consts::U4>);
}
