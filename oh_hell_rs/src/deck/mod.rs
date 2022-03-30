use rand::{prelude::SliceRandom, Rng};

use crate::card::{Card, Rank, Suit};

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
                Card::from_rank_and_suit(Rank::Ace, Suit::Clubs),
                Card::from_rank_and_suit(Rank::King, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Queen, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Jack, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Ten, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Nine, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Eight, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Seven, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Six, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Five, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Four, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Three, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Two, Suit::Clubs),
                Card::from_rank_and_suit(Rank::Ace, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::King, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Queen, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Jack, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Ten, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Nine, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Eight, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Seven, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Six, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Five, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Four, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Three, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Two, Suit::Diamonds),
                Card::from_rank_and_suit(Rank::Ace, Suit::Hearts),
                Card::from_rank_and_suit(Rank::King, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Queen, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Jack, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Ten, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Nine, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Eight, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Seven, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Six, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Five, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Four, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Three, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Two, Suit::Hearts),
                Card::from_rank_and_suit(Rank::Ace, Suit::Spades),
                Card::from_rank_and_suit(Rank::King, Suit::Spades),
                Card::from_rank_and_suit(Rank::Queen, Suit::Spades),
                Card::from_rank_and_suit(Rank::Jack, Suit::Spades),
                Card::from_rank_and_suit(Rank::Ten, Suit::Spades),
                Card::from_rank_and_suit(Rank::Nine, Suit::Spades),
                Card::from_rank_and_suit(Rank::Eight, Suit::Spades),
                Card::from_rank_and_suit(Rank::Seven, Suit::Spades),
                Card::from_rank_and_suit(Rank::Six, Suit::Spades),
                Card::from_rank_and_suit(Rank::Five, Suit::Spades),
                Card::from_rank_and_suit(Rank::Four, Suit::Spades),
                Card::from_rank_and_suit(Rank::Three, Suit::Spades),
                Card::from_rank_and_suit(Rank::Two, Suit::Spades),
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
