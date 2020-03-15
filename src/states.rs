use crate::device::Device;
use crate::game_screen::GameScreen;

pub struct GameState<'a> {
    device: Device,
    screen: GameScreen<'a>,
}

impl GameState<'_> {
    pub fn new(device: Device) -> Self {
        let screen = GameScreen::new();
        GameState { device, screen }
    }

    pub fn tick(&mut self) {
        self.screen.update(&mut self.device.lcd);
    }
}
