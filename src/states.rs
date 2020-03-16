use crate::device::Device;
use crate::game_screen::GameScreen;

pub struct GameState<'a> {
    device: Device,
    screen: GameScreen<'a>,
}

impl GameState<'_> {
    pub fn new(mut device: Device) -> Self {
        let mut screen = GameScreen::new();
        screen.init(&mut device.lcd);
        GameState { device, screen }
    }

    pub fn tick(&mut self) {
        self.screen.update(&mut self.device.lcd);
    }

    pub fn change_timer(&mut self, n: Option<u8>) {
        self.screen.update_timer(n);
    }
}
