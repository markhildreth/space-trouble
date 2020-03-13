use core::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ElapsedMs(u32);

impl ElapsedMs {
    pub fn new(initial: u32) -> Self {
        ElapsedMs(initial)
    }

    pub fn has_elapsed_by(&self, other: ElapsedMs, time: ElapsedMs) -> bool {
        self.0 - other.0 >= time.0
    }
}

impl Add for ElapsedMs {
    type Output = ElapsedMs;

    fn add(self, other: ElapsedMs) -> ElapsedMs {
        ElapsedMs(self.0 + other.0)
    }
}

impl AddAssign for ElapsedMs {
    fn add_assign(&mut self, other: ElapsedMs) {
        self.0 += other.0;
    }
}

impl From<u32> for ElapsedMs {
    fn from(x: u32) -> ElapsedMs {
        return ElapsedMs(x);
    }
}
