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
