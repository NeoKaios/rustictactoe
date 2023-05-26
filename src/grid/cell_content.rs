use std::fmt;

use crate::player::Player;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellContent {
    Empty,
    Player(Player),
}

impl fmt::Display for CellContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellContent::Player(player) => write!(f, "{}", player),
            CellContent::Empty => write!(f, "~"),
        }
    }
}
