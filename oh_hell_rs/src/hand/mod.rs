use std::collections::HashMap;

use crate::{player::Player, trick::Trick};

pub struct Hand {
    leader_of_first_trick: Player,
    bids: HashMap<Player, u32>,
    tricks: Vec<Trick>,
}

impl Hand {
    pub fn with_leader(player: Player) -> Self {
        Self {
            leader_of_first_trick: player,
            bids: HashMap::with_capacity(4),
            tricks: Vec::with_capacity(13),
        }
    }

    pub fn add_trick(&mut self, trick: Trick) {
        self.tricks.push(trick)
    }

    pub fn add_bid(&mut self, bidder: Player, amount: u32) {
        self.bids.insert(bidder, amount);
    }

    /// Get the leader of the first trick.
    pub fn leader_of_first_trick(&self) -> Player {
        self.leader_of_first_trick
    }

    /// Get the leader of the first trick.
    pub fn winner_of_latest_trick(&self) -> Option<Player> {
        self.tricks[self.tricks.len() - 1].find_winning_player()
    }
}
