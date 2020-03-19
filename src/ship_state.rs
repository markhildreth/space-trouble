use crate::messages::{Action, ToggleSwitch, VentControl};
use rand::Rng;

fn generate_random(current: ToggleSwitch) -> ToggleSwitch {
    match current {
        ToggleSwitch::Disabled => ToggleSwitch::Enabled,
        ToggleSwitch::Enabled => ToggleSwitch::Disabled,
    }
}

fn generate_new<T>(rng: &mut T) -> VentControl
where
    T: Rng,
{
    match rng.gen_range(1, 5) {
        1 => VentControl::Hydrogen,
        2 => VentControl::WaterVapor,
        3 => VentControl::Waste,
        4 => VentControl::Frustrations,
        _ => unreachable!(),
    }
}

#[derive(Default)]
pub struct ShipState {
    eigenthrottle: ToggleSwitch,
    // vent_control also here, but carries no state
}

impl ShipState {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Eigenthrottle(v) => self.eigenthrottle = v,
            Action::VentControl(_) => (),
        }
    }

    pub fn generate_action_needed<T>(&mut self, rng: &mut T) -> Action
    where
        T: Rng,
    {
        match rng.gen_range(1, 3) {
            1 => Action::Eigenthrottle(generate_random(self.eigenthrottle)),
            2 => Action::VentControl(generate_new(rng)),
            _ => unreachable!(),
        }
    }
}
