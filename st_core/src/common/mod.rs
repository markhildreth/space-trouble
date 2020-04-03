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
    InitGame(InitGameEvent),
    ReportInitialControlState(ReportInitialControlStateEvent),
    ControlInitFinished(ControlInitFinishedEvent),
    GameStarted(GameStartedEvent),
    NewDirective(NewDirectiveEvent),
    UpdateHullHealth(UpdateHullHealthEvent),
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
pub struct InitGameEvent;
impl Event for InitGameEvent {}
impl From<InitGameEvent> for Events {
    fn from(ev: InitGameEvent) -> Events {
        Events::InitGame(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ReportInitialControlStateEvent {
    pub action: Action,
}
impl Event for ReportInitialControlStateEvent {}
impl From<ReportInitialControlStateEvent> for Events {
    fn from(ev: ReportInitialControlStateEvent) -> Events {
        Events::ReportInitialControlState(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ControlInitFinishedEvent;
impl Event for ControlInitFinishedEvent {}
impl From<ControlInitFinishedEvent> for Events {
    fn from(ev: ControlInitFinishedEvent) -> Events {
        Events::ControlInitFinished(ev)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GameStartedEvent;
impl Event for GameStartedEvent {}
impl From<GameStartedEvent> for Events {
    fn from(ev: GameStartedEvent) -> Events {
        Events::GameStarted(ev)
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
pub struct UpdateHullHealthEvent {
    pub delta: i8,
}
impl Event for UpdateHullHealthEvent {}
impl From<UpdateHullHealthEvent> for Events {
    fn from(ev: UpdateHullHealthEvent) -> Events {
        Events::UpdateHullHealth(ev)
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
