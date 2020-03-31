mod panel;

pub use panel::PanelActor;

use crate::panels::Panel;
use st_common::*;

pub struct Context {
    pub queue: EventQueue,
    pub panel: Panel,
}
