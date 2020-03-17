use crate::device::Device;
use crate::game_screen::GameScreen;
use crate::messages::{Action, ClientMessages, Interface, Messages, Value};
use crate::queue::OutgoingProducer;
use crate::timing::{SpanStatus, TimeSpan};
use embedded_hal::digital::v2::InputPin;

fn calc_blocks(remaining_ms: u32, total_ms: u32) -> u8 {
    // +1 ensures that the time will run out with one
    // block left, and the time will start with all blocks
    // showing.
    return (20 * remaining_ms / total_ms + 1) as u8;
}

pub struct GameState<'a> {
    producer: OutgoingProducer<'a>,
    screen: GameScreen,
    directive_time_span: Option<TimeSpan>,
    button_is_down: bool,
}

impl<'a> GameState<'a> {
    pub fn new(producer: OutgoingProducer<'a>, device: &mut Device) -> Self {
        let mut screen = GameScreen::new();
        screen.init(&mut device.lcd);
        GameState {
            producer,
            screen,
            directive_time_span: None,
            button_is_down: device.button_pin.is_high().unwrap(),
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

        match (self.button_is_down, device.button_pin.is_high().unwrap()) {
            (false, true) => {
                self.button_is_down = true;
                self.producer
                    .enqueue(ClientMessages::ActionPerformed(Action {
                        interface: Interface::Eigenthrottle,
                        value: Value::Enable,
                    }))
                    .unwrap();
            }
            (true, false) => {
                self.button_is_down = false;
            }
            _ => (),
        }
    }

    pub fn handle(&mut self, ms: u32, msg: Messages) {
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
                self.directive_time_span = Some(TimeSpan::new(ms, directive.time_ms as u32));
            }
            Messages::DirectiveComplete => {
                self.screen.update_command_text(None, None);
                self.screen.update_timer(0);
                self.directive_time_span = None;
            }
        }
    }
}
