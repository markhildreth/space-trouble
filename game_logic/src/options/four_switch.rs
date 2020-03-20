use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FourSwitch {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl FourSwitch {
    pub fn random_other(self, rng: &mut impl Rng) -> FourSwitch {
        let new = self as u8 + rng.gen_range(1, 4);
        match new % 4 {
            0 => FourSwitch::Zero,
            1 => FourSwitch::One,
            2 => FourSwitch::Two,
            3 => FourSwitch::Three,
            _ => unreachable!(),
        }
    }
}

impl Default for FourSwitch {
    fn default() -> Self {
        FourSwitch::Zero
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn generates_not_same() {
        let mut rng = thread_rng();
        let from_zero: Vec<_> = (0..100)
            .map(|_| FourSwitch::Zero.random_other(&mut rng))
            .collect();

        assert!(from_zero.iter().all(|&v| v != FourSwitch::Zero));
        assert!(from_zero.iter().any(|&v| v == FourSwitch::One));
        assert!(from_zero.iter().any(|&v| v == FourSwitch::Two));
        assert!(from_zero.iter().any(|&v| v == FourSwitch::Three));

        let from_one: Vec<_> = (0..100)
            .map(|_| FourSwitch::One.random_other(&mut rng))
            .collect();

        println!("{:?}", from_one);
        assert!(from_one.iter().any(|&v| v == FourSwitch::Zero));
        assert!(from_one.iter().all(|&v| v != FourSwitch::One));
        assert!(from_one.iter().any(|&v| v == FourSwitch::Two));
        assert!(from_one.iter().any(|&v| v == FourSwitch::Three));
    }
}
