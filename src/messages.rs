#[derive(Debug)]
pub enum Messages {
    UpdateDistance(u32),
    UpdateHullHealth(u8),
    NewDirective(Directive),
    DirectiveComplete,
}

#[derive(Debug)]
pub enum ClientMessages {
    ActionPerformed(Action),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Eigenthrottle(ToggleSwitch),
    VentControl(VentControl),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSwitch {
    Disabled,
    Enabled,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VentControl {
    Hydrogen,
    WaterVapor,
    Waste,
    Frustrations,
}

#[derive(Copy, Clone, Debug)]
pub struct Directive {
    pub action: Action,
    pub time_ms: u32,
}
