use crate::game_screen::GameScreen;
use crate::strings::get_action_text;
use crate::timing::{SpanStatus, TimeSpan};
use crate::Panel;
use st_data::{ClientMessageProducer, GameMessage};
use st_device::lcd::LCD;
use st_device::Device;

fn calc_blocks(remaining_ms: u32, total_ms: u32) -> u8 {
    return (20 * remaining_ms / total_ms) as u8;
}

pub struct GameState<'a, TPanel>
where
    TPanel: Panel,
{
    producer: ClientMessageProducer<'a>,
    panel: TPanel,
    screen: GameScreen,
    directive_time_span: Option<TimeSpan>,
}

impl<'a, TPanel> GameState<'a, TPanel>
where
    TPanel: Panel,
{
    pub fn new(producer: ClientMessageProducer<'a>, panel: TPanel, lcd: LCD) -> Self {
        let screen = GameScreen::new(lcd);
        GameState {
            producer,
            panel,
            screen,
            directive_time_span: None,
        }
    }

    pub fn update(&mut self, ms: u32) {
        self.screen.update();
        self.panel.update(&mut self.producer, ms);

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
