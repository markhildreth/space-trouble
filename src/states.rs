use crate::device::Device;
use crate::game_screen::GameScreen;
use crate::messages::Messages;

pub struct GameState {
    device: Device,
    screen: GameScreen,
}

impl GameState {
    pub fn new(mut device: Device) -> Self {
        let mut screen = GameScreen::new();
        screen.init(&mut device.lcd);

        GameState { device, screen }
    }

    pub fn update(&mut self) {
        self.screen.update(&mut self.device.lcd);
    }

    pub fn handle(&mut self, msg: Messages) {
        match msg {
            Messages::UpdateDistance(distance) => {
                self.screen.update_distance(distance);
            }
            Messages::UpdateHullHealth(health) => {
                self.screen.update_hull_health(health);
            }
            Messages::UpdateDirectiveTimeRemaining(remaining_ms) => {
                self.screen.update_timer(Some(remaining_ms as u8));
            }
            Messages::NewDirective(directive) => {
                let (msg1, msg2) = ("      Enable", "   Eigenthrottle");
                self.screen.update_command_text(Some(msg1), Some(msg2));
                self.screen.update_timer(Some(directive.time_ms as u8));
            }
        }
    }
}
