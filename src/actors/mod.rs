mod display;
mod game_logic;
mod panel;

pub use display::DisplayActor;
pub use game_logic::GameLogicActor;
pub use panel::PanelActor;

use crate::common::*;
use crate::lcd::LCD;
use crate::panels::Panel;

pub struct Context {
    pub queue: EventQueue,
    pub panel: Panel,
    pub lcd: LCD,
    pub now: Instant,
}
