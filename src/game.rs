//! game.rs - hand flow & street progression (no betting yet)

use crate::{Action, BetRound, Deck, Table, Card}; 

/// The five phases of a Hold'em hand. 
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Street {
    Preflop, 
    Flop, 
    Turn, 
    River, 
    Showdown,
} 

/// A running hand of poker. 
pub struct GameState {
    pub table: Table, 
    pub deck: Deck,
    pub street: Street, 
    pub board: Vec<Card>, 
    pub bet_round: Option<BetRound>,
    pub pot: u32, 

    /// Seat index of the player whose turn is to act 
    pub to_act: usize, 

    /// Size of blinds (SB = small blind, BB = 2xSB). 
    pub small_blind: u32
}

impl GameState {
    /// Create a fresh game around an existing `Table`
    pub fn new(table: Table, small_blind: u32) -> Self {
        Self {
            table, 
            deck: Deck::new_shuffled(),
            street: Street::Preflop, // nothing running yet 
            board: Vec::with_capacity(5), 
            pot: 0, 
            bet_round: None, 
            to_act: 0,
            small_blind, 
        }
    }

    pub fn start_hand(&mut self) {
        // advance to the next player
        self.table.advance_button();

        self.board.clear(); 
        self.pot = 0; 

        // reset players
        for seat in self.table.seats_mut() {
            if let Some(p) = seat {
                p.clear_hand(); 
            }
        }

        self.table.deal_hole_cards(&mut self.deck); 

        // the small blind is put by the player to the left of the dealer button
        let sb_idx = self.table.next_occupied(self.table.dealer_button); 
        let bb_idx = self.table.next_occupied(sb_idx);  
    
        // set action pointer to UTG (first to act pre-flop)
        self.to_act = self.table.next_occupied(bb_idx); 

        self.street = Street::Preflop; 
        self.bet_round = Some(BetRound::new(&self.table, self.table.dealer_button)); 
    }

    /// Move from preflop > flop > turn > river > showdows
    pub fn deal_next_street(&mut self) {
        match self.street {
            Street::Preflop => {
                // Burn 1, deal 3
                self.deck.deal(); // at each betting round a card is discarded
                for _ in 0..3 {
                    self.board.push(self.deck.deal().unwrap())
                }

                self.street = Street::Flop; 
            }

            Street::Flop => {
                self.deck.deal();
                self.board.push(self.deck.deal().unwrap()); 
                self.street = Street::Turn; 
            }

            Street::Turn => {
                self.deck.deal(); 
                self.board.push(self.deck.deal().unwrap()); 
                self.street = Street::River; 
            }

            Street::River => {
                self.street = Street::Showdown;
            }

            Street::Showdown => {
                self.bet_round = Some(BetRound::new(&self.table, self.table.dealer_button));
            }
        }

        // first player to act is left of dealer except pre-flop
        self.to_act = self.table.next_occupied(self.table.dealer_button); 
    }

    /// Helper: pull a blind from a player into the pot (no side-pot handling)
    pub fn collect_blind(&mut self, seat_idx: usize, amount: u32) {
        if let Some(player) = self.table.seat_mut(seat_idx) {
            // if you can't call the full amount you put what you got
            let contrib = amount.min(player.chips);  
            player.chips -= contrib; 
            self.pot += contrib; 
        }
     }
    
    /// Advance action pointer to next active seat
    pub fn advance_action(&mut self) {
        self.to_act = self.table.next_occupied(self.to_act); 
    } 

    pub fn player_action(&mut self, seat_idx: usize, action: Action) {
        if let Some(round) = &mut self.bet_round {
            let next = round.act(&mut self.table, seat_idx, action);    
            match next {
                Some(idx) => self.to_act = idx, 
                None => {
                    // betting round finished: move chips to pot and maybe next street
                    let sidepots = round.into_sidepots(&self.table); 
                    self.pot += sidepots.iter().map(|(p, _)| *p).sum::<u32>(); 

                    if self.street != Street::River {
                        self.deal_next_street();
                    } else {
                        self.street = Street::Showdown; 
                    }
                }
            }
        }
    }

}