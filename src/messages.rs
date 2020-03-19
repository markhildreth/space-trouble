use rand::Rng;

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
    GelatinousDarkbucket(ToggleSwitch),
    VentControl(VentControl),
    NewtonianFibermist(FourSwitch),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSwitch {
    Disabled,
    Enabled,
}

impl ToggleSwitch {
    pub fn generate_new(current: ToggleSwitch) -> ToggleSwitch {
        match current {
            ToggleSwitch::Disabled => ToggleSwitch::Enabled,
            ToggleSwitch::Enabled => ToggleSwitch::Disabled,
        }
    }
}

impl Default for ToggleSwitch {
    fn default() -> Self {
        ToggleSwitch::Disabled
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FourSwitch {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl FourSwitch {
    pub fn generate_new(current: FourSwitch, rng: &mut impl Rng) -> FourSwitch {
        let new = (current as u8 + rng.gen_range(1, 3)) % 4;
        match new {
            0 => FourSwitch::Zero,
            1 => FourSwitch::One,
            2 => FourSwitch::Two,
            3 => FourSwitch::Three,
            _ => unreachable!(),
        }
    }
}

impl Default for FourSwitch {
    fn default() -> Self {
        FourSwitch::Zero
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VentControl {
    Hydrogen,
    WaterVapor,
    Waste,
    Frustrations,
}

impl VentControl {
    pub fn generate_new(rng: &mut impl Rng) -> VentControl {
        match rng.gen_range(1, 5) {
            0 => VentControl::Hydrogen,
            1 => VentControl::WaterVapor,
            2 => VentControl::Waste,
            3 => VentControl::Frustrations,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Directive {
    pub action: Action,
    pub time_ms: u32,
}
