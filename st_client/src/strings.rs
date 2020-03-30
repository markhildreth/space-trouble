use st_common::control_values::{FourSwitchValue, ToggleSwitchValue, VentControlValue};
use st_common::Action;

const EMPTY: &str = "";
const ENABLE: &str = "      Enable";
const DISABLE: &str = "     Disable";
const TO_ONE: &str = "      to One";
const TO_TWO: &str = "      to Two";
const TO_THREE: &str = "     to Three";
const TURN_OFF: &str = "      Turn off";

const EIGENTHROTTLE: &str = "   Eigenthrottle";

const VENT_HYDROGEN: &str = "   Vent Hydrogen";
const VENT_WATER_VAPOR: &str = "  Vent Water Vapor";
const VENT_WASTE: &str = "     Vent Waste";
const VENT_FRUSTRATIONS: &str = "  Vent Frustrations";

const ENABLE_GELATINOUS_DARK_BUCKET: &str = " Enable Gelatinous";
const DISABLE_GELATINOUS_DARK_BUCKET: &str = " Disable Gelatinous";
const GELATINOUS_DARK_BUCKET: &str = "     Darkbucket";

const NEWTONIAN_FIBERMIST: &str = "Newtonian Fibermist";

pub fn get_action_text(action: Action) -> (&'static str, &'static str) {
    match action {
        Action::Eigenthrottle(et) => match et {
            ToggleSwitchValue::Enabled => (ENABLE, EIGENTHROTTLE),
            ToggleSwitchValue::Disabled => (DISABLE, EIGENTHROTTLE),
        },
        Action::VentControl(vc) => match vc {
            VentControlValue::Hydrogen => (VENT_HYDROGEN, EMPTY),
            VentControlValue::WaterVapor => (VENT_WATER_VAPOR, EMPTY),
            VentControlValue::Waste => (VENT_WASTE, EMPTY),
            VentControlValue::Frustrations => (VENT_FRUSTRATIONS, EMPTY),
        },
        Action::GelatinousDarkbucket(gdb) => match gdb {
            ToggleSwitchValue::Enabled => (ENABLE_GELATINOUS_DARK_BUCKET, GELATINOUS_DARK_BUCKET),
            ToggleSwitchValue::Disabled => (DISABLE_GELATINOUS_DARK_BUCKET, GELATINOUS_DARK_BUCKET),
        },
        Action::NewtonianFibermist(nwt) => match nwt {
            FourSwitchValue::Zero => (TURN_OFF, NEWTONIAN_FIBERMIST),
            FourSwitchValue::One => (NEWTONIAN_FIBERMIST, TO_ONE),
            FourSwitchValue::Two => (NEWTONIAN_FIBERMIST, TO_TWO),
            FourSwitchValue::Three => (NEWTONIAN_FIBERMIST, TO_THREE),
        },
    }
}
