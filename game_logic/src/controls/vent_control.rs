use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VentControl {
    Hydrogen,
    WaterVapor,
    Waste,
    Frustrations,
}

impl VentControl {
    pub fn random(rng: &mut impl Rng) -> VentControl {
        let value = rng.gen_range(0, 4);
        match value {
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

        let values: Vec<_> = (0..100).map(|_| VentControl::random(&mut rng)).collect();

        assert!(values.iter().any(|&v| v == VentControl::Hydrogen));
        assert!(values.iter().any(|&v| v == VentControl::WaterVapor));
        assert!(values.iter().any(|&v| v == VentControl::Waste));
        assert!(values.iter().any(|&v| v == VentControl::Frustrations));
    }
}
