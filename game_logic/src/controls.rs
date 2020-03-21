use crate::EnumFill;
use core::fmt::Debug;
use heapless::consts::*;

#[derive(Debug)]
pub(crate) struct Stateful<T> {
    current: T,
    available: heapless::Vec<T, U4>,
}

impl<T> Stateful<T>
where
    T: Copy + Debug,
{
    pub fn perform(&mut self, option: T) {
        self.available.push(self.current).unwrap();
        self.current = option;
    }

    pub fn actions_available(&self) -> bool {
        self.available.len() > 0
    }

    pub fn generate(&mut self, rng: &mut impl rand::Rng) -> T {
        let i = rng.gen_range(0, self.available.len());
        self.available.swap_remove(i)
    }
}

impl<T> Default for Stateful<T>
where
    T: EnumFill + Debug,
{
    fn default() -> Stateful<T> {
        let mut available = heapless::Vec::new();
        T::fill(&mut available);

        let current = available.swap_remove(0);

        Stateful { current, available }
    }
}

#[derive(Debug)]
pub(crate) struct Stateless<T>
where
    T: EnumFill,
{
    available: heapless::Vec<T, U4>,
}

impl<T> Stateless<T>
where
    T: EnumFill,
{
    pub fn perform(&mut self, _option: T) {}

    pub fn actions_available(&self) -> bool {
        self.available.len() > 0
    }

    pub fn generate(&mut self, rng: &mut impl rand::Rng) -> T {
        let i = rng.gen_range(0, self.available.len());
        self.available.swap_remove(i)
    }
}

impl<T> Default for Stateless<T>
where
    T: EnumFill,
{
    fn default() -> Stateless<T> {
        let mut available = heapless::Vec::new();
        T::fill(&mut available);
        Stateless { available }
    }
}
