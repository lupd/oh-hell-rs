use crate::{player::Player, card::Card};

/// Action by a player.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Turn {
    player: Player,
    card: Card,
}

impl Turn {
    /// Get the player who made this turn.
    pub const fn player(&self) -> Player {
        self.player
    }

    /// Get the card played on this turn.
    pub const fn card(&self) -> Card {
        self.card
    }
}
