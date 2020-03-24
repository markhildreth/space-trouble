mod four_switch;
mod toggle_switch;
mod vent_control;

pub use four_switch::FourSwitch;
pub use toggle_switch::ToggleSwitch;
pub use vent_control::VentControl;

pub trait EnumFill
where
    Self: core::marker::Sized,
{
    fn fill(vec: &mut heapless::Vec<Self, heapless::consts::U4>);
}
