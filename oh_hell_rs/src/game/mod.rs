use crate::{hand::Hand, player::Player};

#[derive(Default)]
pub struct Game {
    hands: Vec<Hand>,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_hand(&mut self, hand: Hand) {
        self.hands.push(hand);
    }

    pub fn leader_of_last_hand(&self) -> Option<Player> {
        if self.hands.is_empty() {
            return None;
        }

        Some(self.hands[self.hands.len() - 1].leader_of_first_trick())
    }
}
