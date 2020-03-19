use crate::messages::{Action, ToggleSwitch, VentControl};

const EMPTY: &str = "";
const EIGENTHROTTLE_ONE_ENABLE: &str = "      Enable";
const EIGENTHROTTLE_ONE_DISABLE: &str = "     Disable";
const EIGENTHROTTLE_TWO: &str = "   Eigenthrottle";

const VENT_HYDROGEN: &str = "   Vent Hydrogen";
const VENT_WATER_VAPOR: &str = "  Vent Water Vapor";
const VENT_WASTE: &str = "     Vent Waste";
const VENT_FRUSTRATIONS: &str = "  Vent Frustrations";

pub fn get_action_text(action: Action) -> (&'static str, &'static str) {
    match action {
        Action::Eigenthrottle(eigenthrottle) => match eigenthrottle {
            ToggleSwitch::Enabled => (EIGENTHROTTLE_ONE_ENABLE, EIGENTHROTTLE_TWO),
            ToggleSwitch::Disabled => (EIGENTHROTTLE_ONE_DISABLE, EIGENTHROTTLE_TWO),
        },
        Action::VentControl(vent_control) => match vent_control {
            VentControl::Hydrogen => (VENT_HYDROGEN, EMPTY),
            VentControl::WaterVapor => (VENT_WATER_VAPOR, EMPTY),
            VentControl::Waste => (VENT_WASTE, EMPTY),
            VentControl::Frustrations => (VENT_FRUSTRATIONS, EMPTY),
        },
    }
}
