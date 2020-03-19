use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VentControl {
    Hydrogen,
    WaterVapor,
    Waste,
    Frustrations,
}

impl VentControl {
    pub fn generate_new(self, rng: &mut impl Rng) -> VentControl {
        let new_value = (self as u8) + rng.gen_range(1, 4);
        match new_value % 4 {
            0 => VentControl::Hydrogen,
            1 => VentControl::WaterVapor,
            2 => VentControl::Waste,
            3 => VentControl::Frustrations,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VentControl;
    use rand::thread_rng;

    #[test]
    fn can_generate() {
        let mut rng = thread_rng();
        let from_hydrogen: Vec<_> = (0..100)
            .map(|_| VentControl::Hydrogen.generate_new(&mut rng))
            .collect();

        assert!(from_hydrogen.iter().all(|&v| v != VentControl::Hydrogen));
        assert!(from_hydrogen.iter().any(|&v| v == VentControl::WaterVapor));
        assert!(from_hydrogen.iter().any(|&v| v == VentControl::Waste));
        assert!(from_hydrogen
            .iter()
            .any(|&v| v == VentControl::Frustrations));

        let from_water_vapor: Vec<_> = (0..100)
            .map(|_| VentControl::WaterVapor.generate_new(&mut rng))
            .collect();

        assert!(from_water_vapor.iter().any(|&v| v == VentControl::Hydrogen));
        assert!(from_water_vapor
            .iter()
            .all(|&v| v != VentControl::WaterVapor),);
        assert!(from_water_vapor.iter().any(|&v| v == VentControl::Waste));
        assert!(from_water_vapor
            .iter()
            .any(|&v| v == VentControl::Frustrations),);
    }
}
