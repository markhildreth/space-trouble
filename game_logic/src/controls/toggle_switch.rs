#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSwitch {
    Disabled,
    Enabled,
}

impl ToggleSwitch {
    pub fn generate_other(self) -> ToggleSwitch {
        match self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_select_new_toggle_switch() {
        assert_eq!(
            ToggleSwitch::Enabled.generate_other(),
            ToggleSwitch::Disabled
        );
        assert_eq!(
            ToggleSwitch::Disabled.generate_other(),
            ToggleSwitch::Enabled
        );
    }
}
