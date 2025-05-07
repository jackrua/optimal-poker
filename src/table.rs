use crate::{Deck, Player}; 

/// A configurable Hold'em table with a fixed number of seats. 
pub struct Table {
    /// Seats are `None` when empty; otherwise `Some(Player)`; 
    seats: Vec<Option<Player>>,
    pub dealer_button: usize, 
}

impl Table {
    /// Create a new table with `seat_count` chairs (2‑10 typical).
    pub fn new(seat_count: usize) -> Self {
        assert!((2..=10).contains(&seat_count), "seat count must be 2‑10");
        Self {
            seats: vec![None; seat_count],
            dealer_button: 0,
        }
    }

    /// Try to sit a player in the first free seat. Return their seat index. 
    pub fn add_player(&mut self, player: Player) -> Option<usize> {
        if let Some((idx, seat)) = self
            .seats
            .iter_mut()
            .enumerate()
            .find(|(_, s)| s.is_none())
        {
            *seat = Some(player);
            Some(idx) 
        } else {
            None
        }
    }

    /// Remove and return the player at `seat_idx`
    pub fn remove_player(&mut self, seat_idx: usize) -> Option<Player> {
        self.seats.get_mut(seat_idx)?.take()
    }

    /// Count how many players are currently seated. 
    pub fn active_player_count(&self) -> usize {
        self.seats.iter().filter(|s| s.is_some()).count()
    }

    /// Deal two hole-cards to every seated player who is not folded/all-in.
    pub fn deal_hole_cards(&mut self, deck: &mut Deck) {
        // Deal oen card to each, then the second (round-robin) to mimic real dealing. 
        for _round in 0..2 {
            for seat in &mut self.seats {
                if let Some(p) = seat {
                    let card = deck.deal().expect("Deck ran out of cards"); 
                    p.receive_card(card); 
                }
            }
        }
    }

    /// Advance the dealer button to the next occupied seat. 
    pub fn advance_button(&mut self) {
        let seat_cnt = self.seats.len(); 
        for offset in 1..=seat_cnt {
            let idx = (self.dealer_button + offset) % seat_cnt; 
            if self.seats[idx].is_some() {
                self.dealer_button = idx;
                break;
            }
        }
    }

    /// Return the next seat with a player, skipping empties. 
    pub fn next_occupied(&self, from: usize) -> usize {
        let n = self.seats.len();
        (1..=n)
            .map(|i| (from + i) % n)
            .find(|&i| self.seats[i].is_some())
            .unwrap()
    }

    pub fn seats_mut(&mut self) -> &mut [Option<Player>] {
        &mut self.seats
    }

    pub fn seat_mut(&mut self, idx: usize) -> Option<&mut Player> {
        self.seats.get_mut(idx)?.as_mut()
    }

    /// For quick debugging / REPL:
    pub fn debug_show(&self) {
        println!("=== Table state ===");
        for (i, seat) in self.seats.iter().enumerate() {
            if let Some(p) = seat {
                let cards = p
                    .hole_cards()
                    .map(|(a, b)| format!("{}, {}", a, b))
                    .unwrap_or_else(|| "-".into());
                println!(
                    "Seat {}{} | {} | chips: {} | cards: {}",
                    i,
                    if i == self.dealer_button { " (D)" } else { "" },
                    p.name,
                    p.chips,
                    cards
                );
            } else {
                println!("Seat {} | <empty>", i);
            }
        }
        println!();
    }

}