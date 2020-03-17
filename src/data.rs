use crate::messages::{Action, Interface as IF, Value, Value as V};

const EIGENTHROTTLE_ONE_ENABLE: &str = "      Enable";
const EIGENTHROTTLE_ONE_DISABLE: &str = "     Disable";
const EIGENTHROTTLE_TWO: &str = "   Eigenthrottle";

pub fn get_action_text(action: Action) -> (&'static str, &'static str) {
    match (action.interface, action.value) {
        (IF::Eigenthrottle, V::Enable) => (EIGENTHROTTLE_ONE_ENABLE, EIGENTHROTTLE_TWO),
        (IF::Eigenthrottle, V::Disable) => (EIGENTHROTTLE_ONE_DISABLE, EIGENTHROTTLE_TWO),
    }
}
