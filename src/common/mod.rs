mod control_values;
mod messaging;
mod time;

use heapless::consts::*;
use heapless::spsc::Queue;

pub use control_values::*;
pub use messaging::*;
pub use time::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Events {
    Tick(TickEvent),
    NewDirective(NewDirectiveEvent),
    HullHealthUpdated(HullHealthUpdatedEvent),
    ShipDistanceUpdated(ShipDistanceUpdatedEvent),
    DirectiveCompleted(DirectiveCompletedEvent),
    ActionPerformed(ActionPerformedEvent),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TickEvent;
impl Event for TickEvent {}
impl From<TickEvent> for Events {
    fn from(ev: TickEvent) -> Events {
        Events::Tick(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NewDirectiveEvent {
    pub directive: Directive,
}
impl Event for NewDirectiveEvent {}
impl From<NewDirectiveEvent> for Events {
    fn from(ev: NewDirectiveEvent) -> Events {
        Events::NewDirective(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HullHealthUpdatedEvent {
    pub health: u8,
}
impl Event for HullHealthUpdatedEvent {}
impl From<HullHealthUpdatedEvent> for Events {
    fn from(ev: HullHealthUpdatedEvent) -> Events {
        Events::HullHealthUpdated(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ShipDistanceUpdatedEvent {
    pub distance: u32,
}
impl Event for ShipDistanceUpdatedEvent {}
impl From<ShipDistanceUpdatedEvent> for Events {
    fn from(ev: ShipDistanceUpdatedEvent) -> Events {
        Events::ShipDistanceUpdated(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DirectiveCompletedEvent;
impl Event for DirectiveCompletedEvent {}
impl From<DirectiveCompletedEvent> for Events {
    fn from(ev: DirectiveCompletedEvent) -> Events {
        Events::DirectiveCompleted(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActionPerformedEvent {
    pub action: Action,
}
impl Event for ActionPerformedEvent {}
impl From<ActionPerformedEvent> for Events {
    fn from(ev: ActionPerformedEvent) -> Events {
        Events::ActionPerformed(ev)
    }
}

pub type EventsQueue = Queue<Events, U8>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Directive {
    pub action: Action,
    pub time_limit: Duration,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Eigenthrottle(ToggleSwitchValue),
    GelatinousDarkbucket(ToggleSwitchValue),
    VentControl(VentControlValue),
    NewtonianFibermist(FourSwitchValue),
}
