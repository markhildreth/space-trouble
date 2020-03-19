use crate::messages::{Action, FourSwitch, ToggleSwitch, VentControl};
use rand::Rng;

#[derive(Default)]
pub struct ShipState {
    eigenthrottle: ToggleSwitch,
    // vent_control also here, but carries no state
    gelatinous_darkbucket: ToggleSwitch,
    newtonian_fibermist: FourSwitch,
}

impl ShipState {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Eigenthrottle(v) => self.eigenthrottle = v,
            Action::VentControl(_) => (),
            Action::GelatinousDarkbucket(v) => self.gelatinous_darkbucket = v,
            Action::NewtonianFibermist(v) => self.newtonian_fibermist = v,
        }
    }

    pub fn generate_action_needed<T>(&mut self, rng: &mut T) -> Action
    where
        T: Rng,
    {
        match rng.gen_range(1, 5) {
            1 => Action::Eigenthrottle(ToggleSwitch::generate_new(self.eigenthrottle)),
            2 => Action::VentControl(VentControl::generate_new(rng)),
            3 => {
                Action::GelatinousDarkbucket(ToggleSwitch::generate_new(self.gelatinous_darkbucket))
            }
            4 => {
                Action::NewtonianFibermist(FourSwitch::generate_new(self.newtonian_fibermist, rng))
            }
            _ => unreachable!(),
        }
    }
}
