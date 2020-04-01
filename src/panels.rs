use crate::controls::{
    DebounceControl, FourSwitch, PushButton, StatefulControl, ToggleSwitch, UpdateResult,
};
use feather_m0::gpio::*;
use st_core::actors::Panel;
use st_core::common::*;

type D5 = Pa15<Input<PullDown>>;
type D6 = Pa20<Input<PullDown>>;
type D10 = Pa18<Input<PullDown>>;
type D11 = Pa16<Input<PullDown>>;
type D12 = Pa19<Input<PullDown>>;

type A2 = Pb9<Input<PullDown>>;
type A3 = Pa4<Input<PullDown>>;
type A4 = Pa5<Input<PullDown>>;
type A5 = Pb2<Input<PullDown>>;

pub struct PanelOne {
    pub eigenthrottle: StatefulControl<ToggleSwitch<D5>>,
    pub gelatinous_darkbucket: StatefulControl<ToggleSwitch<D6>>,
    pub vent_hydrogen: StatefulControl<PushButton<A2>>,
    pub vent_water_vapor: StatefulControl<PushButton<A3>>,
    pub vent_waste: StatefulControl<PushButton<A4>>,
    pub vent_frustrations: StatefulControl<PushButton<A5>>,
    pub newtonian_fibermist: DebounceControl<FourSwitch<D10, D11, D12>>,
}

impl PanelOne {
    fn perform(&self, queue: &mut EventsQueue, action: Action) {
        let ev = ActionPerformedEvent { action }.into();
        queue.enqueue(ev).unwrap();
    }
}

impl Panel for PanelOne {
    fn update(&mut self, now: Instant, queue: &mut EventsQueue) {
        if let UpdateResult::Change(value) = self.eigenthrottle.update(now) {
            let action = Action::Eigenthrottle(value);
            self.perform(queue, action);
        }

        if let UpdateResult::Change(value) = self.gelatinous_darkbucket.update(now) {
            let action = Action::GelatinousDarkbucket(value);
            self.perform(queue, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_hydrogen.update(now) {
            let action = Action::VentControl(VentControlValue::Hydrogen);
            self.perform(queue, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_water_vapor.update(now) {
            let action = Action::VentControl(VentControlValue::WaterVapor);
            self.perform(queue, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_waste.update(now) {
            let action = Action::VentControl(VentControlValue::Waste);
            self.perform(queue, action);
        }

        if let UpdateResult::Change(PushButtonValue::Pushed) = self.vent_frustrations.update(now) {
            let action = Action::VentControl(VentControlValue::Frustrations);
            self.perform(queue, action);
        }

        if let UpdateResult::Change(value) = self.newtonian_fibermist.update(now) {
            let action = Action::NewtonianFibermist(value);
            self.perform(queue, action);
        }
    }
}
