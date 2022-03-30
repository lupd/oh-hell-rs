use crate::card::Suit;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Trump {
    /// The specified suit is trump.
    Suit(Suit),
    /// There is no trump suit.
    None,
}
