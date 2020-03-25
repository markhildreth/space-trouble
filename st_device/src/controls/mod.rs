mod debounce;
mod four_switch;
mod push_button;
mod toggle_switch;

pub use debounce::Debounce;
pub use four_switch::FourSwitch;
pub use push_button::PushButton;
pub use toggle_switch::ToggleSwitch;

pub trait Control {}
