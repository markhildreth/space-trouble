use super::controls::{Control, Stateful, Stateless};
use super::GenerateFailReason;
use crate::common::*;
use core::fmt::Debug;

#[derive(Default, Debug)]
pub struct ShipState {
    eigenthrottle: Stateful<ToggleSwitchValue>,
    vent_control: Stateless<VentControlValue>,
    gelatinous_darkbucket: Stateful<ToggleSwitchValue>,
    newtonian_fibermist: Stateful<FourSwitchValue>,
}

impl ShipState {
    pub fn perform(&mut self, action: Action) {
        match action {
            Action::Eigenthrottle(v) => self.eigenthrottle.perform(v),
            Action::VentControl(v) => self.vent_control.perform(v),
            Action::GelatinousDarkbucket(v) => self.gelatinous_darkbucket.perform(v),
            Action::NewtonianFibermist(v) => self.newtonian_fibermist.perform(v),
        }
    }

    pub fn clear(&mut self, action: Action) {
        match action {
            Action::Eigenthrottle(v) => self.eigenthrottle.clear(v),
            Action::VentControl(v) => self.vent_control.clear(v),
            Action::GelatinousDarkbucket(v) => self.gelatinous_darkbucket.clear(v),
            Action::NewtonianFibermist(v) => self.newtonian_fibermist.clear(v),
        }
    }

    pub fn generate_action(
        &mut self,
        rng: &mut impl rand::Rng,
    ) -> Result<Action, GenerateFailReason> {
        let total_available = self.eigenthrottle.actions_available() as u8
            + self.vent_control.actions_available() as u8
            + self.gelatinous_darkbucket.actions_available() as u8
            + self.newtonian_fibermist.actions_available() as u8;

        if total_available == 0 {
            return Err(GenerateFailReason::NoActionsAvailable);
        }

        let r = rng.gen_range(0, total_available);
        let mut i = 0;

        if self.eigenthrottle.actions_available() {
            if r == i {
                return Ok(Action::Eigenthrottle(self.eigenthrottle.generate(rng)));
            }
            i += 1;
        }

        if self.vent_control.actions_available() {
            if r == i {
                return Ok(Action::VentControl(self.vent_control.generate(rng)));
            }
            i += 1;
        }

        if self.gelatinous_darkbucket.actions_available() {
            if r == i {
                return Ok(Action::GelatinousDarkbucket(
                    self.gelatinous_darkbucket.generate(rng),
                ));
            }
            i += 1;
        }

        if self.newtonian_fibermist.actions_available() && r == i {
            return Ok(Action::NewtonianFibermist(
                self.newtonian_fibermist.generate(rng),
            ));
        }

        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn generate_all_actions(ship_state: &mut ShipState) -> Vec<Action> {
        let mut rng = rand::thread_rng();
        let mut results = Vec::with_capacity(16);
        let mut i = 0;
        loop {
            match ship_state.generate_action(&mut rng) {
                Ok(a) => results.push(a),
                Err(GenerateFailReason::NoActionsAvailable) => break,
            }

            i += 1;
            if i == 100 {
                panic!("Generating actions hit 100 items. Stopping");
            }
        }
        results
    }

    #[test]
    fn can_generate_actions_from_default_state() {
        let mut ship = ShipState::default();
        println!("{:?}", ship);
        let actions = generate_all_actions(&mut ship);

        let items = [
            Action::Eigenthrottle(ToggleSwitchValue::Disabled),
            Action::GelatinousDarkbucket(ToggleSwitchValue::Disabled),
            Action::NewtonianFibermist(FourSwitchValue::Zero),
        ];
        items.iter().for_each(|default_action| {
            assert!(
                !actions.contains(default_action),
                "Action {:?} was generated",
                default_action
            );
        });
    }

    #[test]
    fn can_generate_other_actions_from_default_state() {
        let mut ship = ShipState::default();
        let actions = generate_all_actions(&mut ship);

        let items = [
            Action::Eigenthrottle(ToggleSwitchValue::Enabled),
            Action::VentControl(VentControlValue::Hydrogen),
            Action::VentControl(VentControlValue::WaterVapor),
            Action::VentControl(VentControlValue::Waste),
            Action::VentControl(VentControlValue::Frustrations),
            Action::GelatinousDarkbucket(ToggleSwitchValue::Enabled),
            Action::NewtonianFibermist(FourSwitchValue::One),
            Action::NewtonianFibermist(FourSwitchValue::Two),
            Action::NewtonianFibermist(FourSwitchValue::Three),
        ];

        items.iter().for_each(|action| {
            assert!(
                actions.contains(action),
                "Action {:?} was not generated",
                action
            );
        });
    }
}
