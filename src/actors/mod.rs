mod panels;

use crate::panels::Panel;
use st_common::messaging::*;
use st_common::time::*;

struct Context {
    pub producer: Queue<Event, U8>
    pub panel: Panel,
}

pub struct Tick {
    pub now: Instant,
}

pub enum Event {
    Tick(Tick),
}

impl Message for Tick {}
