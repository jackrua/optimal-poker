use poker_engine::{GameState, Player, Table}; 

fn main() {
    let mut table = Table::new(6);
    
    table.add_player(Player::new(1, "Alice", 1_000)); 
    table.add_player(Player::new(2, "Bob", 1_000)); 
    table.add_player(Player::new(3, "Sebastian", 1_000)); 

    let mut game = GameState::new(table, 10); 

    println!("--- New hand ---");
    println!("Dealer at seat {}", game.table.dealer_button);
    println!("Pot: {}", game.pot); 

    for _ in 0..3 {
        game.deal_next_street(); 
        println!("Street: {:?}, board: {:?}", game.street, game.board); 
    }
}