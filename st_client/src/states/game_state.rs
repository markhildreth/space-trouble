use crate::game_screen::GameScreen;
use crate::states::StateUpdate;
use crate::strings::get_action_text;
use crate::timing::{SpanStatus, TimeSpan};
use crate::{Panel, LCD};
use st_common::time::*;
use st_common::{Event, EventQueueProducer};

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

    pub fn update(
        &mut self,
        now: Instant,
        producer: &mut EventQueueProducer,
        panel: &mut impl Panel,
        lcd: &mut impl LCD,
    ) -> Option<StateUpdate> {
        self.screen.update(lcd);
        panel.update(producer, now);

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

        None
    }

    pub(crate) fn handle(&mut self, now: Instant, ev: Event) {
        match ev {
            Event::ShipDistanceUpdated(distance) => {
                self.screen.update_distance(distance);
            }
            Event::HullHealthUpdated(health) => {
                self.screen.update_hull_health(health);
            }
            Event::NewDirective(directive) => {
                let (text_1, text_2) = get_action_text(directive.action);
                self.screen.update_command_text(Some(text_1), Some(text_2));
                let blocks = calc_blocks(Duration::from_millis(0), directive.time_limit);
                self.screen.update_timer(blocks);
                self.directive_time_span = Some(TimeSpan::new(now, directive.time_limit));
            }
            Event::DirectiveCompleted => {
                self.screen.update_command_text(None, None);
                self.screen.update_timer(0);
                self.directive_time_span = None;
            }
            Event::ActionPerformed(_) => (),
        }
    }
}
