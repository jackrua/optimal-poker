use crate::{Card}; 

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize, 
    pub name: String, 
    pub chips: u32, 
    hole_cards: [Option<Card>; 2],
    pub folded: bool,
    pub all_in: bool,
}

/// A no-limit Texas Hold'em action expressed in chips
#[derive(Debug)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet(u32),
    Raise(u32), 
    Allin, 
}


impl Player {
    pub fn new(id: usize, name: impl Into<String>, chips: u32) -> Self {
        Self {
            id, 
            name: name.into(), 
            chips,
            hole_cards: [None, None], 
            folded: false,
            all_in: false
        }
    }

    /// Give the player one card. Returns `false` if they already have two.
    pub fn receive_card(&mut self, card: Card) -> bool {
        for slot in &mut self.hole_cards {
            if slot.is_none() {
                *slot = Some(card);
                return true;
            }
        }
        false
    }

    pub fn clear_hand(&mut self) {
        self.hole_cards = [None, None]; 
        self.folded = false;
        self.all_in = false;
    }

    pub fn hole_cards(&self) -> Option<(Card, Card)> {
        match self.hole_cards {
            [Some(a), Some(b)] => Some((a, b)), 
            _ => None,
        }
    }

}