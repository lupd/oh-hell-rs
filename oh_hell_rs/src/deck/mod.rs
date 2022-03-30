use rand::{prelude::SliceRandom, Rng};

/// A card's value, Two through Ace.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    // Order lowest to highest for a convenient derive of PartialOrd.
    Two,
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

/// A card's suit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// A playing card, with a suit and rank.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Get the card's rank.
    #[must_use]
    pub const fn rank(&self) -> Rank {
        self.rank
    }

    /// Get the card's suit.
    #[must_use]
    pub const fn suit(&self) -> Suit {
        self.suit
    }
}

/// A deck of playing cards.
#[derive(Clone, Copy, Debug, Hash)]
pub struct Deck {
    cards: [Card; 52],
}

impl Deck {
    /// Create a `Deck` with the standard 52 cards.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cards: [
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spades,
                },
            ],
        }
    }

    /// Shuffle the `Deck` using the given source of randomness.
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.cards.shuffle(rng);
    }
}

impl Default for Deck {
    /// Create a `Deck` with the standard 52 cards.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_cmp() {
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::Ace >= Rank::King);
        assert!(Rank::Ace == Rank::Ace);
        assert!(Rank::King < Rank::Ace);
        assert!(Rank::King <= Rank::Ace);
        assert!(Rank::King > Rank::Queen);
        assert!(Rank::Queen > Rank::Jack);
        assert!(Rank::Jack > Rank::Ten);
        assert!(Rank::Ten > Rank::Nine);
        assert!(Rank::Nine > Rank::Eight);
        assert!(Rank::Eight > Rank::Seven);
        assert!(Rank::Seven > Rank::Six);
        assert!(Rank::Six > Rank::Five);
        assert!(Rank::Five > Rank::Four);
        assert!(Rank::Four > Rank::Three);
        assert!(Rank::Three > Rank::Two);
        assert!(Rank::Two < Rank::Ace);
        assert!(Rank::Ace > Rank::Two);
    }
}
