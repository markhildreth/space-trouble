use crate::messages::{Action, ToggleSwitch, VentControl};
use rand::Rng;

impl Default for ToggleSwitch {
    fn default() -> Self {
        ToggleSwitch::Disabled
    }
}

trait StatefulRandom {
    fn new_random_value(&self) -> Self;
}

trait StatelessRandom {
    fn random_value<T: Rng>(rng: &mut T) -> Self;
}

impl StatefulRandom for ToggleSwitch {
    fn new_random_value(&self) -> ToggleSwitch {
        if *self == ToggleSwitch::Disabled {
            ToggleSwitch::Enabled
        } else {
            ToggleSwitch::Disabled
        }
    }
}

impl StatelessRandom for VentControl {
    fn random_value<TRng: Rng>(rng: &mut TRng) -> VentControl {
        VentControl::Hydrogen
    }
}

#[derive(Default)]
struct Stateful<T: StatefulRandom> {
    current_value: T,
}

impl<T: StatefulRandom> Stateful<T> {
    pub fn update(&mut self, new_value: T) {
        self.current_value = new_value;
    }

    pub fn generate_new_value<TRng: Rng>(&self, rng: &mut TRng) -> T {
        self.current_value.new_random_value()
    }
}

struct Stateless<T>
where
    T: StatelessRandom,
{
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Default for Stateless<T>
where
    T: StatelessRandom,
{
    fn default() -> Self {
        Stateless {
            _phantom: core::marker::PhantomData,
        }
    }
}

#[derive(Default)]
pub struct ShipState {
    eigenthrottle: Stateful<ToggleSwitch>,
    vent_control: Stateless<VentControl>,
}

impl ShipState {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Eigenthrottle(v) => self.eigenthrottle.update(v),
            Action::VentControl(v) => (),
        }
    }

    pub fn generate_action_needed<T>(&mut self, rng: &mut T) -> Action
    where
        T: Rng,
    {
        Action::Eigenthrottle(self.eigenthrottle.generate_new_value(rng))
    }
}
