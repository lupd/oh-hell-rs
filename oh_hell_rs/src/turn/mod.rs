use crate::{card::Card, player::Player};

/// Action by a player.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Turn {
    player: Player,
    card: Card,
}

impl Turn {
    #[must_use]
    pub const fn new(player: Player, card: Card) -> Self {
        Self { player, card }
    }

    /// Get the player who made this turn.
    #[must_use]
    pub const fn player(&self) -> Player {
        self.player
    }

    /// Get the card played on this turn.
    #[must_use]
    pub const fn card(&self) -> Card {
        self.card
    }
}
