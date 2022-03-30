use crate::{card::Suit, player::Player, trump::Trump, turn::Turn};

pub struct Trick {
    cards: [Turn; 4],
    trump: Trump,
}

impl Trick {
    #[must_use]
    pub const fn new(cards: [Turn; 4], trump: Trump) -> Self {
        Self { cards, trump }
    }

    /// Get a reference to the trick's cards.
    #[must_use]
    pub const fn cards(&self) -> &[Turn; 4] {
        &self.cards
    }

    /// Get the trump suit.
    #[must_use]
    pub const fn trump(&self) -> Trump {
        self.trump
    }

    /// Get the suit that was lead.
    #[must_use]
    pub const fn lead_suit(&self) -> Suit {
        self.cards[0].card().suit()
    }

    /// Return the player who wins the trick, or None if a winner could not be determined.
    ///
    /// The result of a trick with duplicate cards is not explicitly specified, but
    /// will not be undefined behavior.
    #[must_use]
    pub fn find_winning_player(&self) -> Option<Player> {
        if let Trump::Suit(suit) = self.trump() {
            // The winning trick is the strongest trump card, if one was played.
            if let Some(trick_card) = find_highest_card_of_trick_with_suit(&self.cards, suit) {
                return Some(trick_card.player());
            }
        }

        // The winning trick is the strongest card of the suit that was led.
        find_highest_card_of_trick_with_suit(&self.cards, self.lead_suit()).map(Turn::player)
    }
}

fn find_highest_card_of_trick_with_suit(cards: &[Turn; 4], suit: Suit) -> Option<&Turn> {
    cards
        .iter()
        .filter(|trick_card| trick_card.card().suit() == suit)
        .max_by(|a, b| a.card().rank().cmp(&b.card().rank()))
}

#[cfg(test)]
mod tests {
    use crate::{
        card::{Card, Rank, Suit},
        player::Player,
        trump::Trump,
        turn::Turn,
    };

    use super::*;

    #[test]
    fn no_trump_with_no_one_following_suit() {
        let trick = Trick {
            cards: [
                Turn::new(
                    Player::North,
                    Card::from_rank_and_suit(Rank::Two, Suit::Spades),
                ),
                Turn::new(
                    Player::East,
                    Card::from_rank_and_suit(Rank::Ace, Suit::Diamonds),
                ),
                Turn::new(
                    Player::South,
                    Card::from_rank_and_suit(Rank::Ace, Suit::Hearts),
                ),
                Turn::new(
                    Player::West,
                    Card::from_rank_and_suit(Rank::Ace, Suit::Clubs),
                ),
            ],
            trump: Trump::None,
        };
        assert_eq!(trick.find_winning_player().unwrap(), Player::North);
    }

    #[test]
    fn no_trump_with_everyone_following_suit_with_lead_winning() {
        let trick = Trick {
            cards: [
                Turn::new(
                    Player::North,
                    Card::from_rank_and_suit(Rank::Ace, Suit::Spades),
                ),
                Turn::new(
                    Player::East,
                    Card::from_rank_and_suit(Rank::King, Suit::Spades),
                ),
                Turn::new(
                    Player::South,
                    Card::from_rank_and_suit(Rank::Two, Suit::Spades),
                ),
                Turn::new(
                    Player::West,
                    Card::from_rank_and_suit(Rank::Eight, Suit::Spades),
                ),
            ],
            trump: Trump::None,
        };
        assert_eq!(trick.find_winning_player().unwrap(), Player::North);
    }

    #[test]
    fn no_trump_with_everyone_following_suit_with_lead_losing() {
        let trick = Trick {
            cards: [
                Turn::new(
                    Player::North,
                    Card::from_rank_and_suit(Rank::Four, Suit::Spades),
                ),
                Turn::new(
                    Player::East,
                    Card::from_rank_and_suit(Rank::King, Suit::Spades),
                ),
                Turn::new(
                    Player::South,
                    Card::from_rank_and_suit(Rank::Two, Suit::Spades),
                ),
                Turn::new(
                    Player::West,
                    Card::from_rank_and_suit(Rank::Eight, Suit::Spades),
                ),
            ],
            trump: Trump::None,
        };
        assert_eq!(trick.find_winning_player().unwrap(), Player::East);
    }

    #[test]
    fn trump_suit_with_everyone_leading_trump_with_lead_losing() {
        let trick = Trick {
            cards: [
                Turn::new(
                    Player::North,
                    Card::from_rank_and_suit(Rank::Four, Suit::Spades),
                ),
                Turn::new(
                    Player::East,
                    Card::from_rank_and_suit(Rank::King, Suit::Spades),
                ),
                Turn::new(
                    Player::South,
                    Card::from_rank_and_suit(Rank::Two, Suit::Spades),
                ),
                Turn::new(
                    Player::West,
                    Card::from_rank_and_suit(Rank::Eight, Suit::Spades),
                ),
            ],
            trump: Trump::Suit(Suit::Spades),
        };
        assert_eq!(trick.find_winning_player().unwrap(), Player::East);
    }

    #[test]
    fn trump_suit_with_everyone_leading_trump_with_lead_winning() {
        let trick = Trick {
            cards: [
                Turn::new(
                    Player::North,
                    Card::from_rank_and_suit(Rank::Ace, Suit::Spades),
                ),
                Turn::new(
                    Player::East,
                    Card::from_rank_and_suit(Rank::King, Suit::Spades),
                ),
                Turn::new(
                    Player::South,
                    Card::from_rank_and_suit(Rank::Two, Suit::Spades),
                ),
                Turn::new(
                    Player::West,
                    Card::from_rank_and_suit(Rank::Eight, Suit::Spades),
                ),
            ],
            trump: Trump::Suit(Suit::Spades),
        };
        assert_eq!(trick.find_winning_player().unwrap(), Player::North);
    }
}
