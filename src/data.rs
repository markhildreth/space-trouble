use crate::messages::{Action, FourSwitch, ToggleSwitch, VentControl};

const EMPTY: &str = "";
const ENABLE: &str = "      Enable";
const DISABLE: &str = "     Disable";
const TO_ZERO: &str = "      to Zero";
const TO_ONE: &str = "      to One";
const TO_TWO: &str = "      to Two";
const TO_THREE: &str = "     to Three";

const EIGENTHROTTLE: &str = "   Eigenthrottle";

const VENT_HYDROGEN: &str = "   Vent Hydrogen";
const VENT_WATER_VAPOR: &str = "  Vent Water Vapor";
const VENT_WASTE: &str = "     Vent Waste";
const VENT_FRUSTRATIONS: &str = "  Vent Frustrations";

const GELATINOUS_DARK_BUCKET: &str = "Gelatinous Darkbucket";

const NEWTONIAN_FIBERMIST: &str = "Newtonian Fibermist";

pub fn get_action_text(action: Action) -> (&'static str, &'static str) {
    match action {
        Action::Eigenthrottle(eigenthrottle) => match eigenthrottle {
            ToggleSwitch::Enabled => (ENABLE, EIGENTHROTTLE),
            ToggleSwitch::Disabled => (DISABLE, EIGENTHROTTLE),
        },
        Action::VentControl(vent_control) => match vent_control {
            VentControl::Hydrogen => (VENT_HYDROGEN, EMPTY),
            VentControl::WaterVapor => (VENT_WATER_VAPOR, EMPTY),
            VentControl::Waste => (VENT_WASTE, EMPTY),
            VentControl::Frustrations => (VENT_FRUSTRATIONS, EMPTY),
        },
        Action::GelatinousDarkbucket(gdb) => match gdb {
            ToggleSwitch::Enabled => (ENABLE, GELATINOUS_DARK_BUCKET),
            ToggleSwitch::Disabled => (DISABLE, GELATINOUS_DARK_BUCKET),
        },
        Action::NewtonianFibermist(gdb) => match gdb {
            FourSwitch::Zero => (NEWTONIAN_FIBERMIST, TO_ZERO),
            FourSwitch::One => (NEWTONIAN_FIBERMIST, TO_ONE),
            FourSwitch::Two => (NEWTONIAN_FIBERMIST, TO_TWO),
            FourSwitch::Three => (NEWTONIAN_FIBERMIST, TO_THREE),
        },
    }
}
