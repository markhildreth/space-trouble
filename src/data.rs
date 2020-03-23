use game_logic::{Action, FourSwitch, ToggleSwitch, VentControl};

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
            ToggleSwitch::Enabled => (ENABLE, EIGENTHROTTLE),
            ToggleSwitch::Disabled => (DISABLE, EIGENTHROTTLE),
        },
        Action::VentControl(vc) => match vc {
            VentControl::Hydrogen => (VENT_HYDROGEN, EMPTY),
            VentControl::WaterVapor => (VENT_WATER_VAPOR, EMPTY),
            VentControl::Waste => (VENT_WASTE, EMPTY),
            VentControl::Frustrations => (VENT_FRUSTRATIONS, EMPTY),
        },
        Action::GelatinousDarkbucket(gdb) => match gdb {
            ToggleSwitch::Enabled => (ENABLE_GELATINOUS_DARK_BUCKET, GELATINOUS_DARK_BUCKET),
            ToggleSwitch::Disabled => (DISABLE_GELATINOUS_DARK_BUCKET, GELATINOUS_DARK_BUCKET),
        },
        Action::NewtonianFibermist(nwt) => match nwt {
            FourSwitch::Zero => (TURN_OFF, NEWTONIAN_FIBERMIST),
            FourSwitch::One => (NEWTONIAN_FIBERMIST, TO_ONE),
            FourSwitch::Two => (NEWTONIAN_FIBERMIST, TO_TWO),
            FourSwitch::Three => (NEWTONIAN_FIBERMIST, TO_THREE),
        },
    }
}
