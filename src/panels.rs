use crate::controls::{DebounceControl, FourSwitch, PushButton, StatefulControl, ToggleSwitch};
use feather_m0::gpio::*;
use heapless::consts::*;
use heapless::Vec;
use st_core::common::*;
use st_core::device::Panel;

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

fn push_button_only(&x: &PushButtonValue) -> bool {
    x == PushButtonValue::Pushed
}

impl Panel for PanelOne {
    // Note that this will not return values for momentary push buttons, as this
    // is designed to get the current state, whereas push buttons are designed to
    // not really have a "state" so much as a moment where they are pushed
    fn poll_all(&mut self, _: Instant) -> Vec<Action, U8> {
        let mut vec: Vec<Action, U8> = Vec::new();
        vec.extend_from_slice(&[
            Action::Eigenthrottle(self.eigenthrottle.read()),
            Action::GelatinousDarkbucket(self.gelatinous_darkbucket.read()),
            Action::NewtonianFibermist(self.newtonian_fibermist.read()),
        ])
        .unwrap();
        vec
    }

    fn poll_changed(&mut self, now: Instant) -> Vec<Action, U8> {
        let mut vec: Vec<Action, U8> = Vec::new();
        self.eigenthrottle
            .update(now)
            .map(|x| vec.push(Action::Eigenthrottle(x)));
        self.gelatinous_darkbucket
            .update(now)
            .map(|x| vec.push(Action::GelatinousDarkbucket(x)));
        self.vent_hydrogen
            .update(now)
            .filter(push_button_only)
            .map(|_| vec.push(Action::VentControl(VentControlValue::Hydrogen)));
        self.vent_water_vapor
            .update(now)
            .filter(push_button_only)
            .map(|_| vec.push(Action::VentControl(VentControlValue::WaterVapor)));
        self.vent_waste
            .update(now)
            .filter(push_button_only)
            .map(|_| vec.push(Action::VentControl(VentControlValue::Waste)));
        self.vent_frustrations
            .update(now)
            .filter(push_button_only)
            .map(|_| vec.push(Action::VentControl(VentControlValue::Frustrations)));
        self.newtonian_fibermist
            .update(now)
            .map(|x| vec.push(Action::NewtonianFibermist(x)));
        vec
    }
}
