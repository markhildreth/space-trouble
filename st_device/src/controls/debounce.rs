use crate::controls::Control;

pub struct Debounce<T>
where
    T: Control,
{
    inner_control: T,
    debounce_ms: u32,
}

impl<T> Debounce<T>
where
    T: Control,
{
    pub fn new(inner_control: T, debounce_ms: u32) -> Debounce<T> {
        Debounce {
            inner_control,
            debounce_ms,
        }
    }
}

impl<T> Control for Debounce<T> where T: Control {}
