mod betting; 
mod player;
mod table; 
mod evaluator; 
mod game; 

pub use betting::BetRound; 
pub use player::{Action, Player};
pub use table::Table; 
pub use evaluator::{evaluate_seven, HandCategory, HandRank}; 
pub use game::{GameState, Street}; 

use rand::{seq::SliceRandom, thread_rng}; 
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Suit {
    Clubs, 
    Diamonds,
    Hearts, 
    Spades
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_symbol = match self.suit {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        write!(f, "{:?}{}", self.rank, suit_symbol)
    }
}

/// A standard 52‑card deck.
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Returns a shuffled deck. 
    pub fn new_shuffled() -> Self {
        let mut cards = Vec::with_capacity(52); 
        for &suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank_val in 2u8..=14 {
                let rank = match rank_val {
                    2 => Rank::Two,
                    3 => Rank::Three,
                    4 => Rank::Four,
                    5 => Rank::Five,
                    6 => Rank::Six,
                    7 => Rank::Seven,
                    8 => Rank::Eight,
                    9 => Rank::Nine,
                    10 => Rank::Ten,
                    11 => Rank::Jack,
                    12 => Rank::Queen,
                    13 => Rank::King,
                    14 => Rank::Ace,
                    _ => unreachable!(), // TODO: I don't think this is necessary
                };
                cards.push(Card { rank, suit }); 
            }
        }

        cards.shuffle(&mut thread_rng()); 
        Self { cards } 
    }

    /// Pops one card off the top; returns `None` when empty. 
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop() 
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn deck_has_52_unique_cards() {
        let mut deck = Deck::new_shuffled(); 
        let mut seen = std::collections::HashSet::new();
        while let Some(card) = deck.deal() {
            assert!(seen.insert(card));
        }
        assert_eq!(seen.len(), 52); 
    }
}
