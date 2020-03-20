#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToggleSwitch {
    Disabled,
    Enabled,
}

impl ToggleSwitch {
    pub fn random_other(self) -> ToggleSwitch {
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
        assert_eq!(ToggleSwitch::Enabled.random_other(), ToggleSwitch::Disabled);
        assert_eq!(ToggleSwitch::Disabled.random_other(), ToggleSwitch::Enabled);
    }
}
