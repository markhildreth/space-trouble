use crate::options::{FourSwitch, ToggleSwitch, VentControl};
use crate::Action;
use rand::Rng;

#[derive(Default)]
pub struct ShipState {
    eigenthrottle: ToggleSwitch,
    // vent_control also here, but carries no state.
    gelatinous_darkbucket: ToggleSwitch,
    newtonian_fibermist: FourSwitch,
}

impl ShipState {
    pub fn perform(&mut self, action: Action) {
        match action {
            Action::Eigenthrottle(v) => self.eigenthrottle = v,
            Action::VentControl(_) => (),
            Action::GelatinousDarkbucket(v) => self.gelatinous_darkbucket = v,
            Action::NewtonianFibermist(v) => self.newtonian_fibermist = v,
        }
    }

    pub fn generate_action_needed<T>(&self, rng: &mut T) -> Action
    where
        T: Rng,
    {
        match rng.gen_range(1, 5) {
            1 => Action::Eigenthrottle(self.eigenthrottle.random_other()),
            2 => Action::VentControl(VentControl::random(rng)),
            3 => {
                Action::GelatinousDarkbucket(ToggleSwitch::random_other(self.gelatinous_darkbucket))
            }
            4 => {
                Action::NewtonianFibermist(FourSwitch::random_other(self.newtonian_fibermist, rng))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_generates<'a>(state: &ShipState, actions: impl Iterator<Item = &'a Action>) {
        let mut rng = rand::thread_rng();
        let gen: Vec<Action> = (0..1000)
            .map(|_| state.generate_action_needed(&mut rng))
            .collect();

        for action in actions {
            assert!(gen.contains(action), "Did not generate {:?}", action);
        }
    }

    fn assert_does_not_generate<'a>(state: &ShipState, actions: impl Iterator<Item = &'a Action>) {
        let mut rng = rand::thread_rng();
        let gen: Vec<Action> = (0..1000)
            .map(|_| state.generate_action_needed(&mut rng))
            .collect();

        for action in actions {
            assert!(!gen.contains(action), "Did not generate {:?}", action);
        }
    }

    #[test]
    fn generates_all_available_actions() {
        let state = ShipState::default();

        assert_generates(
            &state,
            [
                Action::Eigenthrottle(ToggleSwitch::Enabled),
                Action::VentControl(VentControl::Hydrogen),
                Action::VentControl(VentControl::WaterVapor),
                Action::VentControl(VentControl::Waste),
                Action::VentControl(VentControl::Frustrations),
                Action::GelatinousDarkbucket(ToggleSwitch::Enabled),
                Action::NewtonianFibermist(FourSwitch::One),
                Action::NewtonianFibermist(FourSwitch::Two),
                Action::NewtonianFibermist(FourSwitch::Three),
            ]
            .iter(),
        );

        assert_does_not_generate(
            &state,
            [
                Action::Eigenthrottle(ToggleSwitch::Disabled),
                Action::GelatinousDarkbucket(ToggleSwitch::Disabled),
                Action::NewtonianFibermist(FourSwitch::Zero),
            ]
            .iter(),
        );
    }

    #[test]
    fn generates_after_perform() {
        let mut state = ShipState::default();
        state.perform(Action::Eigenthrottle(ToggleSwitch::Enabled));
        state.perform(Action::GelatinousDarkbucket(ToggleSwitch::Enabled));
        state.perform(Action::NewtonianFibermist(FourSwitch::One));

        assert_generates(
            &state,
            [
                Action::Eigenthrottle(ToggleSwitch::Disabled),
                Action::GelatinousDarkbucket(ToggleSwitch::Disabled),
                Action::NewtonianFibermist(FourSwitch::Zero),
            ]
            .iter(),
        );

        assert_does_not_generate(
            &state,
            [
                Action::Eigenthrottle(ToggleSwitch::Enabled),
                Action::GelatinousDarkbucket(ToggleSwitch::Enabled),
                Action::NewtonianFibermist(FourSwitch::One),
            ]
            .iter(),
        );
    }
}
