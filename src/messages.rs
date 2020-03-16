pub enum Messages {
    UpdateDistance(u32),
    UpdateHullHealth(u8),
    UpdateDirectiveTimeRemaining(u16),
    NewDirective(Directive),
}

pub enum Interface {
    Eigenthrottle,
}

pub enum Value {
    Enable,
    Disable,
}

pub struct Action {
    pub interface: Interface,
    pub value: Value,
}

pub struct Directive {
    pub action: Action,
    pub time_ms: u16,
}
