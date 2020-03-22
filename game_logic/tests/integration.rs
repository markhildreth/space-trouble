use game_logic::{Game, GameMessage, GameMessageConsumer, GameMessageQueue};

fn drain(queue: &mut GameMessageConsumer) -> Vec<GameMessage> {
    let mut v = Vec::new();
    while queue.ready() {
        v.push(queue.dequeue().unwrap());
    }
    v
}

#[test]
fn integration() {
    let mut rng = rand::thread_rng();
    let mut queue = GameMessageQueue::new();
    let (producer, mut consumer) = queue.split();
    let mut game = Game::new(producer);

    // At time zero, nothing of importance should be happening.
    game.update(0, &mut rng);
    assert_eq!(consumer.ready(), false);

    // At time 2_000, a new directive should be sent
    game.update(2_000, &mut rng);
    let msgs = drain(&mut consumer);
    let directives: Vec<_> = msgs
        .iter()
        .filter_map(|msg| match msg {
            GameMessage::NewDirective(d) => Some(d),
            _ => None,
        })
        .collect();;
    assert!(directives.len() > 0);
}
