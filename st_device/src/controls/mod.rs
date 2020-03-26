mod four_switch;
mod push_button;
mod toggle_switch;

pub use four_switch::FourSwitch;
pub use push_button::PushButton;
pub use toggle_switch::ToggleSwitch;

pub enum UpdateResult<T> {
    NoChange,
    Change(T),
}

pub trait Control<T>
where
    T: Eq + Copy + Default,
    Self: Sized,
{
    fn stateful(self) -> StatefulControl<Self, T> {
        StatefulControl::new(self)
    }

    fn debounce(self, ms: u32) -> DebounceControl<Self, T> {
        DebounceControl::new(self, ms)
    }

    fn read(&self) -> T;
}

pub struct StatefulControl<TCon, TVal>
where
    TCon: Control<TVal>,
    TVal: Eq + Copy + Default,
{
    control: TCon,
    current_value: TVal,
}

impl<TCon, TVal> StatefulControl<TCon, TVal>
where
    TCon: Control<TVal>,
    TVal: Eq + Copy + Default,
{
    fn new(control: TCon) -> StatefulControl<TCon, TVal> {
        StatefulControl {
            control,
            current_value: TVal::default(),
        }
    }

    pub fn update(&mut self, _ms: u32) -> UpdateResult<TVal> {
        let value = self.control.read();
        if value == self.current_value {
            UpdateResult::NoChange
        } else {
            self.current_value = value;
            UpdateResult::Change(self.current_value)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum DebounceStatus<TVal> {
    Neutral,
    Debouncing { ends_at: u32, de_value: TVal },
}

pub struct DebounceControl<TCon, TVal>
where
    TCon: Control<TVal>,
    TVal: Eq + Copy + Default,
{
    control: TCon,
    debounce_time: u32,
    current_value: TVal,
    debounce_status: DebounceStatus<TVal>,
}

impl<TCon, TVal> DebounceControl<TCon, TVal>
where
    TCon: Control<TVal>,
    TVal: Eq + Copy + Default,
{
    fn new(control: TCon, debounce_time: u32) -> DebounceControl<TCon, TVal> {
        DebounceControl {
            control,
            debounce_time,
            current_value: TVal::default(),
            debounce_status: DebounceStatus::Neutral,
        }
    }

    pub fn update(&mut self, ms: u32) -> UpdateResult<TVal> {
        let new_value = self.control.read();
        match (self.debounce_status, self.current_value == new_value) {
            (DebounceStatus::Neutral, true) => UpdateResult::NoChange,
            (DebounceStatus::Neutral, false) => {
                self.start_debounce(ms, new_value);
                UpdateResult::NoChange
            }
            (DebounceStatus::Debouncing { .. }, true) => {
                self.stop_debouncing();
                UpdateResult::NoChange
            }
            (DebounceStatus::Debouncing { de_value, ends_at }, false) => {
                if de_value != new_value {
                    self.start_debounce(ms, new_value);
                    UpdateResult::NoChange
                } else if ms > ends_at {
                    self.current_value = de_value;
                    self.stop_debouncing();
                    UpdateResult::Change(self.current_value)
                } else {
                    UpdateResult::NoChange
                }
            }
        }
    }

    fn start_debounce(&mut self, ms: u32, value: TVal) {
        self.debounce_status = DebounceStatus::Debouncing {
            de_value: value,
            ends_at: ms + self.debounce_time,
        };
    }

    fn stop_debouncing(&mut self) {
        self.debounce_status = DebounceStatus::Neutral;
    }
}
