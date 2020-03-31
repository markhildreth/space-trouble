use st_common::control_values::{FourSwitchValue, ToggleSwitchValue, VentControlValue};
use st_common::Action;

pub type GameString = &'static str;
const EMPTY: GameString = "";
const ENABLE: GameString = "      Enable";
const DISABLE: GameString = "     Disable";
const TO_ONE: GameString = "      to One";
const TO_TWO: GameString = "      to Two";
const TO_THREE: GameString = "     to Three";
const TURN_OFF: GameString = "      Turn off";

const EIGENTHROTTLE: GameString = "   Eigenthrottle";

const VENT_HYDROGEN: GameString = "   Vent Hydrogen";
const VENT_WATER_VAPOR: GameString = "  Vent Water Vapor";
const VENT_WASTE: GameString = "     Vent Waste";
const VENT_FRUSTRATIONS: GameString = "  Vent Frustrations";

const ENABLE_GELATINOUS_DARK_BUCKET: GameString = " Enable Gelatinous";
const DISABLE_GELATINOUS_DARK_BUCKET: GameString = " Disable Gelatinous";
const GELATINOUS_DARK_BUCKET: GameString = "     Darkbucket";

const NEWTONIAN_FIBERMIST: GameString = "Newtonian Fibermist";

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
