// we may want to expand this
use crate::{Card, Rank}; 

/// The nine hand types ranked from weakest to strongest
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandCategory {
    HighCard = 0, 
    OnePair = 1, 
    TwoPair = 2, 
    ThreeOfAKind = 3, 
    Straight = 4, 
    Flush = 5, 
    FullHouse = 6, 
    FourOfAKind = 7, 
    StraightFlush = 8, 
}

/// A fully ordered score: higher compares > lower. 
#[derive(Eq, PartialEq, Debug)]
pub struct HandRank {
    pub category: HandCategory, 
    kickers: [Rank; 5],
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.category
            .cmp(&other.category)
            .then_with(|| self.kickers.cmp(&other.kickers))
    }
}
impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Evaluate the best 5-card hand out of 7. 
pub fn evaluate_seven(cards: &[Card; 7]) -> HandRank {
    use itertools::Itertools; 

    cards
        .iter()
        .copied()
        .combinations(5)
        .map(|c5| evaluate_five(&c5))
        .max()
        .unwrap()
}

/// Core evaluator for exactly 5 cards.
fn evaluate_five(cards: &[Card]) -> HandRank {
    debug_assert_eq!(cards.len(), 5);

    // ----- tally ranks & suits -----
    let mut rank_counts = [0u8; 15];     // index 2..14
    let mut suit_counts = [0u8; 4];      // Clubs=0 … Spades=3
    for c in cards {
        rank_counts[c.rank as usize] += 1;
        suit_counts[c.suit as usize] += 1;
    }

    let is_flush = suit_counts.iter().any(|&n| n == 5);

    // ----- detect straight (incl. wheel A‑2‑3‑4‑5) -----
    let mut straight_high: Option<Rank> = None;
    for hi in (5..=14).rev() {
        if (0..5).all(|i| rank_counts[hi - i] > 0) {
            straight_high = Some(num_to_rank(hi));
            break;
        }
    }
    // wheel check
    if straight_high.is_none()
        && (rank_counts[14] > 0 && (1..=4).all(|r| rank_counts[r + 1] > 0))
    {
        straight_high = Some(Rank::Five);
    }

    // ----- grouped ranks -----
    // collect (count, rank) pairs, sorted desc by count then rank
    let mut groups: Vec<(u8, Rank)> = (2..=14)
        .filter(|&v| rank_counts[v] > 0)
        .map(|v| (rank_counts[v], num_to_rank(v)))
        .collect();
    groups.sort_by(|a, b| b.cmp(a)); // biggest group first

    // helpers
    let take_kickers = |g: &[(u8, Rank)]| -> Vec<Rank> {
        g.iter()
            .flat_map(|&(cnt, r)| std::iter::repeat(r).take(cnt as usize))
            .collect()
    };

    // ----- classify -----
    let rank = if is_flush && straight_high.is_some() {
        // straight flush
        HandRank {
            category: HandCategory::StraightFlush,
            kickers: [straight_high.unwrap(), Rank::Two, Rank::Two, Rank::Two, Rank::Two],
        }
    } else if groups[0].0 == 4 {
        // quads
        let kickers = take_kickers(&groups);
        HandRank {
            category: HandCategory::FourOfAKind,
            kickers: [kickers[0], kickers[1], Rank::Two, Rank::Two, Rank::Two],
        }
    } else if groups[0].0 == 3 && groups[1].0 == 2 {
        HandRank {
            category: HandCategory::FullHouse,
            kickers: [groups[0].1, groups[1].1, Rank::Two, Rank::Two, Rank::Two],
        }
    } else if is_flush {
        let mut ranks = cards
            .iter()
            .map(|c| c.rank)
            .collect::<Vec<_>>();
        ranks.sort_by(|a, b| b.cmp(a));
        HandRank {
            category: HandCategory::Flush,
            kickers: [ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]],
        }
    } else if let Some(hi) = straight_high {
        HandRank {
            category: HandCategory::Straight,
            kickers: [hi, Rank::Two, Rank::Two, Rank::Two, Rank::Two],
        }
    } else if groups[0].0 == 3 {
        let kickers = take_kickers(&groups);
        HandRank {
            category: HandCategory::ThreeOfAKind,
            kickers: [kickers[0], kickers[1], kickers[2], kickers[3], kickers[4]],
        }
    } else if groups[0].0 == 2 && groups[1].0 == 2 {
        let kickers = take_kickers(&groups);
        HandRank {
            category: HandCategory::TwoPair,
            kickers: [kickers[0], kickers[1], kickers[2], kickers[3], Rank::Two],
        }
    } else if groups[0].0 == 2 {
        let kickers = take_kickers(&groups);
        HandRank {
            category: HandCategory::OnePair,
            kickers: [kickers[0], kickers[1], kickers[2], kickers[3], kickers[4]],
        }
    } else {
        // high card
        let mut ks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
        ks.sort_by(|a, b| b.cmp(a));
        HandRank {
            category: HandCategory::HighCard,
            kickers: [ks[0], ks[1], ks[2], ks[3], ks[4]],
        }
    };

    rank
}

/// helper: 2-14 -> Rank
#[inline]
fn num_to_rank(n: usize) -> Rank {
    unsafe { std::mem::transmute::<u8, Rank>(n as u8) }
}