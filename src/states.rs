use crate::data::get_action_text;
use crate::game_screen::GameScreen;
use crate::queue::{ClientMessage, ClientMessageProducer};
use crate::timing::{SpanStatus, TimeSpan};
use st_data::controls::{FourSwitch, ToggleSwitch, VentControl};
use st_data::{Action, GameMessage};
use st_device::game_pin::{GamePin, PinResult, PinValue};
use st_device::Device;

fn calc_blocks(remaining_ms: u32, total_ms: u32) -> u8 {
    return (20 * remaining_ms / total_ms) as u8;
}

pub struct GameState<'a> {
    producer: ClientMessageProducer<'a>,
    screen: GameScreen,
    directive_time_span: Option<TimeSpan>,
    eigenthrottle_pin: GamePin,
    gelatinous_darkbucket_pin: GamePin,
    vent_hydrogen_pin: GamePin,
    vent_water_vapor_pin: GamePin,
    vent_waste_pin: GamePin,
    vent_frustrations_pin: GamePin,
    newtonian_fibermist_one_pin: GamePin,
    newtonian_fibermist_two_pin: GamePin,
    newtonian_fibermist_three_pin: GamePin,
}

impl<'a> GameState<'a> {
    pub fn new(producer: ClientMessageProducer<'a>, device: &mut Device) -> Self {
        let mut screen = GameScreen::new();
        screen.init(&mut device.lcd);
        GameState {
            producer,
            screen,
            directive_time_span: None,
            eigenthrottle_pin: GamePin::new(&device.pin_d5, 0),
            gelatinous_darkbucket_pin: GamePin::new(&device.pin_d6, 0),
            vent_hydrogen_pin: GamePin::new(&device.pin_a2, 0),
            vent_water_vapor_pin: GamePin::new(&device.pin_a3, 0),
            vent_waste_pin: GamePin::new(&device.pin_a4, 0),
            vent_frustrations_pin: GamePin::new(&device.pin_a5, 0),
            newtonian_fibermist_one_pin: GamePin::new(&device.pin_d10, 800),
            newtonian_fibermist_two_pin: GamePin::new(&device.pin_d11, 800),
            newtonian_fibermist_three_pin: GamePin::new(&device.pin_d12, 800),
        }
    }

    pub fn update(&mut self, device: &mut Device) {
        let ms = device.ms();
        self.screen.update(&mut device.lcd);
        if let Some(span) = &self.directive_time_span {
            let status = span.status(ms);
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

        if let PinResult::Change(value) = self.eigenthrottle_pin.update(ms, &device.pin_d5) {
            let toggle_switch = match value {
                PinValue::Low => ToggleSwitch::Disabled,
                PinValue::High => ToggleSwitch::Enabled,
            };
            self.send_action_performed(Action::Eigenthrottle(toggle_switch));
        }

        if let PinResult::Change(value) = self.gelatinous_darkbucket_pin.update(ms, &device.pin_d6)
        {
            let toggle_switch = match value {
                PinValue::Low => ToggleSwitch::Disabled,
                PinValue::High => ToggleSwitch::Enabled,
            };
            self.send_action_performed(Action::GelatinousDarkbucket(toggle_switch));
        }

        if PinResult::Change(PinValue::High) == self.vent_hydrogen_pin.update(ms, &device.pin_a2) {
            self.send_action_performed(Action::VentControl(VentControl::Hydrogen));
        }

        if PinResult::Change(PinValue::High) == self.vent_water_vapor_pin.update(ms, &device.pin_a3)
        {
            self.send_action_performed(Action::VentControl(VentControl::WaterVapor));
        }

        if PinResult::Change(PinValue::High) == self.vent_waste_pin.update(ms, &device.pin_a4) {
            self.send_action_performed(Action::VentControl(VentControl::Waste));
        }

        if PinResult::Change(PinValue::High)
            == self.vent_frustrations_pin.update(ms, &device.pin_a5)
        {
            self.send_action_performed(Action::VentControl(VentControl::Frustrations));
        }

        if PinResult::Change(PinValue::High)
            == self.newtonian_fibermist_one_pin.update(ms, &device.pin_d10)
        {
            self.send_action_performed(Action::NewtonianFibermist(FourSwitch::One));
        }

        if PinResult::Change(PinValue::High)
            == self.newtonian_fibermist_two_pin.update(ms, &device.pin_d11)
        {
            self.send_action_performed(Action::NewtonianFibermist(FourSwitch::Two));
        }

        if PinResult::Change(PinValue::High)
            == self
                .newtonian_fibermist_three_pin
                .update(ms, &device.pin_d12)
        {
            self.send_action_performed(Action::NewtonianFibermist(FourSwitch::Three));
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

    fn send_action_performed(&mut self, action: Action) {
        let msg = ClientMessage::ActionPerformed(action);
        self.producer.enqueue(msg).unwrap();
    }
}
