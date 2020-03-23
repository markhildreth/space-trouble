use crate::data::get_action_text;
use crate::device::Device;
use crate::game_pin::{GamePin, PinResult, PinValue};
use crate::game_screen::GameScreen;
use crate::queue::{ClientMessage, ClientMessageProducer};
use crate::timing::{SpanStatus, TimeSpan};
use embedded_hal::digital::v2::InputPin;
use game_logic::{Action, GameMessage, ToggleSwitch};

fn calc_blocks(remaining_ms: u32, total_ms: u32) -> u8 {
    // +1 ensures that the time will run out with one
    // block left, and the time will start with all blocks
    // showing.
    return (20 * remaining_ms / total_ms + 1) as u8;
}

pub struct GameState<'a> {
    producer: ClientMessageProducer<'a>,
    screen: GameScreen,
    directive_time_span: Option<TimeSpan>,
    eigenthrottle_pin: GamePin,
}

impl<'a> GameState<'a> {
    pub fn new(producer: ClientMessageProducer<'a>, device: &mut Device) -> Self {
        let mut screen = GameScreen::new();
        screen.init(&mut device.lcd);
        GameState {
            producer,
            screen,
            directive_time_span: None,
            eigenthrottle_pin: GamePin::new(device.button_pin.is_high().unwrap().into()),
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

        if let PinResult::Change(new_value) = self
            .eigenthrottle_pin
            .update(device.ms(), &device.button_pin)
        {
            let switch = match new_value {
                PinValue::Low => ToggleSwitch::Disabled,
                PinValue::High => ToggleSwitch::Enabled,
            };
            let msg = ClientMessage::ActionPerformed(Action::Eigenthrottle(switch));
            self.producer.enqueue(msg).unwrap();
        }
    }

    pub fn handle(&mut self, ms: u32, msg: GameMessage) {
        match msg {
            GameMessage::ShipDistanceUpdated(distance) => {
                self.screen.update_distance(distance);
            }
            GameMessage::HullHealthUpdated(health) => {
                self.screen.update_hull_health(health);
            }
            GameMessage::NewDirective(directive) => {
                let (text_1, text_2) = get_action_text(directive.action);
                self.screen.update_command_text(Some(text_1), Some(text_2));
                let blocks = calc_blocks(0, directive.expiration);
                self.screen.update_timer(blocks);
                self.directive_time_span = Some(TimeSpan::new(ms, directive.expiration as u32));
            }
            GameMessage::DirectiveCompleted => {
                self.screen.update_command_text(None, None);
                self.screen.update_timer(0);
                self.directive_time_span = None;
            }
        }
    }
}
