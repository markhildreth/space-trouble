mod four_switch;
mod push_button;
mod toggle_switch;

pub use four_switch::FourSwitch;
pub use push_button::PushButton;
pub use toggle_switch::ToggleSwitch;

use crate::device::Device;

pub enum UpdateResult<T> {
    NoChange,
    Change(T),
}

pub trait Control<T>
where
    T: Eq + Copy + Default,
    Self: Sized,
{
    fn stateful(self) -> StatefulControl<Self, T> {
        StatefulControl::new(self)
    }

    fn read(&self, device: &Device) -> T;
}

pub struct StatefulControl<TCon, TVal>
where
    TCon: Control<TVal>,
    TVal: Eq + Copy + Default,
{
    control: TCon,
    current_value: TVal,
}

impl<TCon, TVal> StatefulControl<TCon, TVal>
where
    TCon: Control<TVal>,
    TVal: Eq + Copy + Default,
{
    fn new(control: TCon) -> StatefulControl<TCon, TVal> {
        StatefulControl {
            control,
            current_value: TVal::default(),
        }
    }

    pub fn update(&mut self, device: &Device) -> UpdateResult<TVal> {
        let value = self.control.read(device);
        if value == self.current_value {
            UpdateResult::NoChange
        } else {
            self.current_value = value;
            UpdateResult::Change(self.current_value)
        }
    }
}
