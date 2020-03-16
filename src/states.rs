use crate::device::Device;
use crate::game_screen::GameScreen;
use crate::messages::Messages;
use crate::timing::{SpanStatus, TimeSpan};

fn calc_blocks(remaining_ms: u32, total_ms: u32) -> u8 {
    // +1 ensures that the time will run out with one
    // block left, and the time will start with all blocks
    // showing.
    return (20 * remaining_ms / total_ms + 1) as u8;
}

pub struct GameState {
    screen: GameScreen,
    directive_time_span: Option<TimeSpan>,
}

impl GameState {
    pub fn new(device: &mut Device) -> Self {
        let mut screen = GameScreen::new();
        screen.init(&mut device.lcd);
        GameState {
            screen,
            directive_time_span: None,
        }
    }

    pub fn update(&mut self, device: &mut Device) {
        self.screen.update(&mut device.lcd);
        if let Some(span) = &self.directive_time_span {
            let status = span.status(device.ms());
            match status {
                SpanStatus::Ongoing {
                    remaining_ms,
                    total_ms,
                } => {
                    let blocks = calc_blocks(remaining_ms, total_ms);
                    self.screen.update_timer(blocks);
                }
                SpanStatus::Completed => {
                    self.screen.update_command_text(None, None);
                    self.screen.update_timer(0);
                    self.directive_time_span = None;
                }
            }
        }
    }

    pub fn handle(&mut self, device: &mut Device, msg: Messages) {
        match msg {
            Messages::UpdateDistance(distance) => {
                self.screen.update_distance(distance);
            }
            Messages::UpdateHullHealth(health) => {
                self.screen.update_hull_health(health);
            }
            Messages::NewDirective(directive) => {
                let (msg1, msg2) = ("      Enable", "   Eigenthrottle");
                self.screen.update_command_text(Some(msg1), Some(msg2));
                let blocks = calc_blocks(0, directive.time_ms as u32);
                self.screen.update_timer(blocks);
                self.directive_time_span =
                    Some(TimeSpan::new(device.ms(), directive.time_ms as u32));
            }
            Messages::CompleteDirective => {
                self.screen.update_command_text(None, None);
                self.screen.update_timer(0);
                self.directive_time_span = None;
            }
        }
    }
}
