use crate::{Table, Action}; 

/// Per-street betting state. 
pub struct BetRound {
    /// how much each seat has put this steet so far 
    contributed: Vec<u32>, 
    current_bet: u32, 
    opener: Option<usize>, 
    last_to_act: usize, 
}

impl BetRound {
    pub fn new(table: &Table, dealer_button: usize) -> Self {
        let seats = table.seat_count(); 

        let last = table.next_occupied(dealer_button); 
        Self {
            contributed: vec![0; seats], 
            current_bet: 0,
            opener: None,
            last_to_act: last,
        }
    }

    /// Apply an action, mutate player stacks, and return the seat index that
    /// should act next (Or `None` if the round is finished)
    pub fn act(
        &mut self, 
        table: &mut Table, 
        seat_idx: usize,
        action: Action,
    ) -> Option<usize> {
        let player = table.seat_mut(seat_idx).expect("empty seat cannot act"); 

        match action {

            Action::Fold => {
                player.folded = true;
            }

            Action::Check => {}
            Action::Call | Action::Allin | Action::Bet(_) | Action::Raise(_) => {
                // compute the amount they must put in
                let want_to_put = match action {
                    Action::Call => self.current_bet,
                    Action::Allin => player.chips + self.contributed[seat_idx], 
                    Action::Bet(amt) => amt, 
                    Action::Raise(inc) => self.current_bet + inc,
                    _ => unreachable!(), 
                };

                let already_in = self.contributed[seat_idx]; 
                let missing = want_to_put.saturating_sub(already_in); 

                // cap by their stack 
                let pay = missing.min(player.chips); 
                player.chips -= pay; 
                self.contributed[seat_idx] += pay;

                // update current bet & opener
                if self.contributed[seat_idx] > self.current_bet {
                    self.current_bet = self.contributed[seat_idx]; 
                    self.opener = Some(seat_idx); 
                    self.last_to_act = table.next_occupied(seat_idx); 
                }
            }  

        }

        // is betting round closed? 
        let mut idx = table.next_occupied(seat_idx); 
        loop {
            if idx == seat_idx {
                break; // full - loop nobody left to act
            }

            let p = table.seat(idx).unwrap(); 
            if !p.folded && self.contributed[idx] != self.current_bet {
                return Some(idx); // someone still owes chips
            }

            idx = table.next_occupied(idx); 
        }

        // everybody matched (or folded) -> round over
        None
    }

    /// Consume the BetRound and return a list of `(pot_size, winners_mask)` side-pots
    pub fn into_sidepots(&self, table: &Table) -> Vec<(u32, Vec<usize>)> {
        // gather (seat, contributed) for players still in hand
        let mut pairs: Vec<(usize, u32)> = self
            .contributed
            .iter()
            .enumerate()
            .filter(|(i, _)| table.seat(*i).map(|p| !p.folded).unwrap_or(false))
            .map(|(i, &c)| (i, c))
            .collect(); 

        // sort by contribution (ascending) to peel side-pots
        pairs.sort_by_key(|&(_, c)| c); 

        let mut pots = Vec::new(); 
        let mut running_total = 0; 

        while !pairs.is_empty() && pairs[0].1 > running_total {
            let level = pairs[0].1; 
            let level_size = level - running_total; 

            let contestants: Vec<usize> = pairs.iter().map(|&(i, _)| i).collect(); 
            let pot_size = level_size * contestants.len() as u32; 
            pots.push((pot_size, contestants)); 

            running_total = level;
            pairs.remove(0);
        }
        pots
    }
}