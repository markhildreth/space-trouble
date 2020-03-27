use st_data::control_values::ToggleSwitchValue;
use st_data::time::*;
use st_data::{Action, Directive, GameMessage, GameMessageConsumer, GameMessageQueue};
use st_server::Game;

fn drain(queue: &mut GameMessageConsumer) -> Vec<GameMessage> {
    let mut v = Vec::new();
    while queue.ready() {
        v.push(queue.dequeue().unwrap());
    }
    v
}

fn find_directives(msgs: &Vec<GameMessage>) -> Vec<Directive> {
    msgs.iter()
        .filter_map(|msg| match msg {
            GameMessage::NewDirective(d) => Some(d),
            _ => None,
        })
        .cloned()
        .collect()
}

fn find_distance_updates(msgs: &Vec<GameMessage>) -> Vec<u32> {
    msgs.iter()
        .filter_map(|msg| match msg {
            GameMessage::ShipDistanceUpdated(new_d) => Some(new_d),
            _ => None,
        })
        .cloned()
        .collect()
}

fn find_hull_health_updates(msgs: &Vec<GameMessage>) -> Vec<u8> {
    msgs.iter()
        .filter_map(|msg| match msg {
            GameMessage::HullHealthUpdated(new_h) => Some(new_h),
            _ => None,
        })
        .cloned()
        .collect()
}

#[test]
fn integration() {
    let mut queue = GameMessageQueue::new();
    let (producer, mut consumer) = queue.split();
    let mut game = Game::new(producer);

    let mut clock = Instant::ZERO;

    // At time zero, nothing of importance should be happening.
    game.update(clock);
    assert_eq!(consumer.ready(), false);

    game.perform(clock, Action::Eigenthrottle(ToggleSwitchValue::Disabled));
    let msgs = drain(&mut consumer);
    let hull_health_msgs = find_hull_health_updates(&msgs);
    assert_eq!(
        hull_health_msgs,
        [98],
        "No hull health msgs found in {:?}",
        msgs
    );

    // Advance to when a directive is given & ship distance updates
    clock += Duration::from_secs(2);
    game.update(clock);
    let msgs = drain(&mut consumer);
    let directives = find_directives(&msgs);
    assert!(directives.len() == 1, "No directives found in {:?}", msgs);
    let distance_updates = find_distance_updates(&msgs);
    assert!(
        distance_updates.len() == 1,
        "No distance updates found in {:?}",
        msgs
    );

    // Perform the action that we were directed to perform
    clock += Duration::from_secs(1);
    let directive = directives[0];
    game.perform(clock, directive.action);
    let msgs = drain(&mut consumer);
    assert_eq!(msgs, [GameMessage::DirectiveCompleted]);

    // Let's generate another action.
    clock += Duration::from_secs(2);
    game.update(clock);
    let msgs = drain(&mut consumer);
    let directives = find_directives(&msgs);
    assert!(directives.len() == 1, "No directives found in {:?}", msgs);
    let directive = directives[0];

    // And we should fail it.
    clock += directive.time_limit;
    game.update(clock);
    let msgs = drain(&mut consumer);
    assert!(
        msgs.contains(&GameMessage::HullHealthUpdated(94)),
        "Messages: {:?}",
        msgs
    );
}
