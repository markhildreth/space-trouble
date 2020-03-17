#[derive(Debug)]
pub enum Messages {
    UpdateDistance(u32),
    UpdateHullHealth(u8),
    NewDirective(Directive),
    CompleteDirective,
}

#[derive(Copy, Clone, Debug)]
pub enum Interface {
    Eigenthrottle,
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Enable,
    Disable,
}

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub interface: Interface,
    pub value: Value,
}

#[derive(Copy, Clone, Debug)]
pub struct Directive {
    pub action: Action,
    pub time_ms: u32,
}
