use game_logic::{Directive, Game, GameMessage, GameMessageConsumer, GameMessageQueue};

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

#[test]
fn integration() {
    let mut rng = rand::thread_rng();
    let mut queue = GameMessageQueue::new();
    let (producer, mut consumer) = queue.split();
    let mut game = Game::new(producer);

    let mut clock = 0;

    // At time zero, nothing of importance should be happening.
    game.update(clock, &mut rng);
    assert_eq!(consumer.ready(), false);

    // Advance to when a directive is given
    clock += 2_000;
    game.update(clock, &mut rng);
    let msgs = drain(&mut consumer);
    let directives = find_directives(&msgs);
    assert_eq!(directives.len(), 1, "No directives found in {:?}", msgs);

    // Perform the action that we were directed to perform
    clock += 1_000;
    let directive = directives[0];
    game.perform(clock, directive.action);
    assert_eq!(consumer.ready(), false);

    // Let's generate another action.
    clock += 2_000;
    game.update(clock, &mut rng);
    let msgs = drain(&mut consumer);
    let directives = find_directives(&msgs);
    assert_eq!(directives.len(), 1, "No directives found in {:?}", msgs);
    let directive = directives[0];

    // And we should fail it.
    clock += directive.expiration;
    game.update(clock, &mut rng);
    let msgs = drain(&mut consumer);
    assert!(
        msgs.contains(&GameMessage::HullHealthUpdated(96)),
        "Messages: {:?}",
        msgs
    );
}
