use crate::messages::{Action, ToggleSwitch};

const EIGENTHROTTLE_ONE_ENABLE: &str = "      Enable";
const EIGENTHROTTLE_ONE_DISABLE: &str = "     Disable";
const EIGENTHROTTLE_TWO: &str = "   Eigenthrottle";

pub fn get_action_text(action: Action) -> (&'static str, &'static str) {
    match action {
        Action::Eigenthrottle(ToggleSwitch::Enabled) => {
            (EIGENTHROTTLE_ONE_ENABLE, EIGENTHROTTLE_TWO)
        }
        Action::Eigenthrottle(ToggleSwitch::Disabled) => {
            (EIGENTHROTTLE_ONE_DISABLE, EIGENTHROTTLE_TWO)
        }
        _ => ("Unknown", "Thingy!"),
    }
}
