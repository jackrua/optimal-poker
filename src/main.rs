use poker_engine::*;
fn main() {
    let mut table = Table::new(3);
    table.add_player(Player::new(1, "Alice", 100));
    table.add_player(Player::new(2, "Bob",   100));
    table.add_player(Player::new(3, "Carol", 50));

    let mut game = GameState::new(table, 2);
    game.start_hand();


    // TODO: 
    // * built a bot that tries a naive strategy 



    // pre-flop actions
    game.player_action(game.to_act, Action::Call);                  // UTG limp
    game.player_action(game.to_act, Action::Raise(6));              // SB raises to 8
    game.player_action(game.to_act, Action::Allin);                 // BB shoves 50
    game.player_action(game.to_act, Action::Call);                  // UTG calls full 50
    game.player_action(game.to_act, Action::Call);                  // SB covers 50

    // automatically deals flop, turn, river when rounds close
    println!("Final board: {:?}", game.board);
    println!("Pot: {}", game.pot);          // 150 chips total (with side-pots inside)
}
