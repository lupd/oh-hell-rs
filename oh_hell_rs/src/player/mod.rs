#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Player {
    North,
    West,
    East,
    South,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::North => write!(f, "North"),
            Player::West => write!(f, "West"),
            Player::East => write!(f, "East"),
            Player::South => write!(f, "South"),
        }
    }
}
