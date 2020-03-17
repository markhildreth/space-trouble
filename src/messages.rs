pub enum Messages {
    UpdateDistance(u32),
    UpdateHullHealth(u8),
    NewDirective(Directive),
    CompleteDirective,
}

#[derive(Copy, Clone)]
pub enum Interface {
    Eigenthrottle,
}

#[derive(Copy, Clone)]
pub enum Value {
    Enable,
    Disable,
}

#[derive(Copy, Clone)]
pub struct Action {
    pub interface: Interface,
    pub value: Value,
}

#[derive(Copy, Clone)]
pub struct Directive {
    pub action: Action,
    pub time_ms: u32,
}
