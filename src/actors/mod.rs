mod display;
mod panel;

pub use display::DisplayActor;
pub use panel::PanelActor;

use crate::lcd::LCD;
use crate::panels::Panel;
use st_common::time::*;
use st_common::*;

pub struct Context {
    pub queue: EventQueue,
    pub panel: Panel,
    pub lcd: LCD,
    pub now: Instant,
}
