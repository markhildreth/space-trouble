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
    // There's probably a better way to do this. Using conservative_impl_trait is
    // an unstable feature that would resolve it, although not sure if it would be
    // allowed on the trait itself. Just going to accept the ugly for now.
    type Iter = PanelControlValueIterator;

    fn poll(&mut self, now: Instant) -> Self::Iter {
        let mut vec = Vec::<_, U8>::new();
        vec.extend_from_slice(&[
            self.eigenthrottle
                .update(now)
                .map(|x| Action::Eigenthrottle(x)),
            self.gelatinous_darkbucket
                .update(now)
                .map(|x| Action::GelatinousDarkbucket(x)),
            self.vent_hydrogen
                .update(now)
                .filter(push_button_only)
                .map(|_| Action::VentControl(VentControlValue::Hydrogen)),
            self.vent_water_vapor
                .update(now)
                .filter(push_button_only)
                .map(|_| Action::VentControl(VentControlValue::WaterVapor)),
            self.vent_waste
                .update(now)
                .filter(push_button_only)
                .map(|_| Action::VentControl(VentControlValue::Waste)),
            self.vent_frustrations
                .update(now)
                .filter(push_button_only)
                .map(|_| Action::VentControl(VentControlValue::Frustrations)),
            self.newtonian_fibermist
                .update(now)
                .map(|x| Action::NewtonianFibermist(x)),
        ])
        .unwrap();
        PanelControlValueIterator::new(vec)
    }
}

pub struct PanelControlValueIterator {
    results: heapless::Vec<Option<Action>, heapless::consts::U8>,
}

impl PanelControlValueIterator {
    fn new(
        results: heapless::Vec<Option<Action>, heapless::consts::U8>,
    ) -> PanelControlValueIterator {
        PanelControlValueIterator { results }
    }
}

impl Iterator for PanelControlValueIterator {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.results.pop() {
                Some(Some(action)) => return Some(action),
                None => return None,
                _ => (),
            }
        }
    }
}
