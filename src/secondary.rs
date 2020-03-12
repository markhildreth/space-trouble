use crate::lcd::LCD;

struct Secondary {
    lcd: LCD,
    ship_health: u16,
    distance_traveled: u16,
}

impl Secondary {
    pub fn new(lcd: LCD) {
        Secondary {
            lcd,
            ship_health,
            distance_traveled,
        }
    }
}
