use crate::common::*;
use core::fmt::Debug;
use heapless::consts::*;
use rand::Rng;

pub(super) trait Control<T> {
    fn perform(&mut self, option: T);
    fn clear(&mut self, option: T);
    fn actions_available(&self) -> bool;
    fn generate(&mut self, rng: &mut impl Rng) -> T;
}

#[derive(Debug)]
pub(super) struct Stateful<T> {
    current: T,
    available: heapless::Vec<T, U4>,
}

impl<T> Control<T> for Stateful<T>
where
    T: Copy + Debug + Eq,
{
    fn perform(&mut self, option: T) {
        self.available.push(self.current).unwrap();
        self.current = option;

        // See if the option being performed is available. If so, remove it.
        let index = self
            .available
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v == option { Some(i) } else { None })
            .nth(0);

        if let Some(i) = index {
            self.available.swap_remove(i);
        }
    }

    fn clear(&mut self, option: T) {
        if !self.available.contains(&option) {
            self.available.push(option).unwrap();
        }
    }

    fn actions_available(&self) -> bool {
        self.available.len() > 0
    }

    fn generate(&mut self, rng: &mut impl Rng) -> T {
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
pub(super) struct Stateless<T>
where
    T: EnumFill,
{
    available: heapless::Vec<T, U4>,
}

impl<T> Control<T> for Stateless<T>
where
    T: EnumFill + Debug + Eq,
{
    fn perform(&mut self, option: T) {
        if !self.available.contains(&option) {
            self.available.push(option).unwrap();
        }
    }

    fn clear(&mut self, option: T) {
        if !self.available.contains(&option) {
            self.available.push(option).unwrap();
        }
    }

    fn actions_available(&self) -> bool {
        self.available.len() > 0
    }

    fn generate(&mut self, rng: &mut impl Rng) -> T {
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

#[cfg(test)]
mod test_stateful {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum MyEnum {
        First,
        Second,
        Third,
    }

    fn generate_all_options(c: &mut impl Control<MyEnum>) -> Vec<MyEnum> {
        let mut rng = rand::thread_rng();
        let mut results = Vec::with_capacity(4);
        let mut loops = 0;
        while c.actions_available() {
            results.push(c.generate(&mut rng));
            loops += 1;
            if loops > 3 {
                panic!(
                    "The generate_actions method generated too many actions! {:?}",
                    results
                );
            }
        }
        results
    }

    impl EnumFill for MyEnum {
        fn fill(vec: &mut heapless::Vec<Self, U4>) {
            vec.push(MyEnum::First).unwrap();
            vec.push(MyEnum::Second).unwrap();
            vec.push(MyEnum::Third).unwrap();
        }
    }

    mod stateful {
        use super::*;
        #[test]
        fn test_stateful_default() {
            let mut control = Stateful::<MyEnum>::default();
            let options = generate_all_options(&mut control);
            assert!(!options.contains(&MyEnum::First));
            assert!(options.iter().filter(|&&x| x == MyEnum::Second).count() == 1);
            assert!(options.iter().filter(|&&x| x == MyEnum::Third).count() == 1);
        }

        #[test]
        fn test_generate_option_after_perform() {
            let mut control = Stateful::<MyEnum>::default();
            control.perform(MyEnum::Second);
            let options = generate_all_options(&mut control);
            assert!(options.contains(&MyEnum::First));
        }

        #[test]
        fn test_generate_after_clear() {
            let mut rng = rand::thread_rng();
            let mut control = Stateful::<MyEnum>::default();
            let generated_option = control.generate(&mut rng);
            control.clear(generated_option);

            let options = generate_all_options(&mut control);
            assert!(options.contains(&generated_option));
        }

        #[test]
        fn test_can_flip_flop() {
            let mut control = Stateful::<MyEnum>::default();
            control.perform(MyEnum::First);
            control.perform(MyEnum::Second);
            control.perform(MyEnum::First);
            control.perform(MyEnum::Second);
        }
    }

    mod stateless {
        use super::*;

        #[test]
        fn test_stateless_default() {
            let mut control = Stateless::<MyEnum>::default();
            let options = generate_all_options(&mut control);
            assert!(options.contains(&MyEnum::First));
            assert!(options.contains(&MyEnum::Second));
            assert!(options.contains(&MyEnum::Third));
        }

        #[test]
        fn test_stateless_after_generation() {
            let mut rng = rand::thread_rng();
            let mut control = Stateless::<MyEnum>::default();
            let generated_option = control.generate(&mut rng);
            let options = generate_all_options(&mut control);
            assert!(!options.contains(&generated_option));
        }

        #[test]
        fn test_stateless_after_perform() {
            let mut rng = rand::thread_rng();
            let mut control = Stateless::<MyEnum>::default();
            let generated_option = control.generate(&mut rng);
            control.perform(generated_option);
            let options = generate_all_options(&mut control);
            assert!(options.contains(&generated_option));
        }
    }
}
