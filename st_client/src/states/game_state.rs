use crate::game_screen::GameScreen;
use crate::strings::get_action_text;
use crate::timing::{SpanStatus, TimeSpan};
use crate::{ComponentDef, Components, Panel};
use st_data::time::*;
use st_data::GameMessage;

fn calc_blocks(remaining: Duration, total: Duration) -> u8 {
    return (20 * remaining.as_millis() / total.as_millis()) as u8;
}

pub(crate) struct GameState {
    screen: GameScreen,
    directive_time_span: Option<TimeSpan>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            screen: GameScreen::new(),
            directive_time_span: None,
        }
    }

    pub fn update<CDef: ComponentDef>(&mut self, c: &mut Components<CDef>, now: Instant) {
        self.screen.update(&mut c.lcd);
        c.panel.update(&mut c.producer, now);

        if let Some(span) = &self.directive_time_span {
            let status = span.status(now);
            match status {
                SpanStatus::Ongoing { remaining, total } => {
                    let blocks = calc_blocks(remaining, total);
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

    pub(crate) fn handle(&mut self, now: Instant, msg: GameMessage) {
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
                let blocks = calc_blocks(Duration::from_millis(0), directive.time_limit);
                self.screen.update_timer(blocks);
                self.directive_time_span = Some(TimeSpan::new(now, directive.time_limit));
            }
            GameMessage::DirectiveCompleted => {
                self.screen.update_command_text(None, None);
                self.screen.update_timer(0);
                self.directive_time_span = None;
            }
        }
    }
}
