use std::fmt;

use super::cell_content::{CellContent, CellContent::Empty };
use super::coord::Coord;
use crate::player::Player;

#[derive(Debug)]
pub struct Grid {
    grid: [[CellContent; 3]; 3],
    is_full: bool,
    filled_cells_count: i32,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: [
                [Empty, Empty, Empty],
                [Empty, Empty, Empty],
                [Empty, Empty, Empty],
            ],
            is_full: false,
            filled_cells_count: 0,
        }
    }

    pub fn show(&self) {
        println!("{}", self)
    }

    pub fn set_cell_to_player(&mut self, player: Player, coord: Coord) {
        self.set_cell(CellContent::Player(player), coord)
    }

    pub fn set_cell(&mut self, elem: CellContent, coord: Coord) {
        if elem != Empty {
            self.filled_cells_count += 1
        }
        if self.filled_cells_count >= 9 {
            self.is_full = true
        }
        match coord {
            Coord::TopLeft => self.grid[0][0] = elem,
            Coord::TopCenter => self.grid[0][1] = elem,
            Coord::TopRight => self.grid[0][2] = elem,
            Coord::MidLeft => self.grid[1][0] = elem,
            Coord::MidCenter => self.grid[1][1] = elem,
            Coord::MidRight => self.grid[1][2] = elem,
            Coord::BotLeft => self.grid[2][0] = elem,
            Coord::BotCenter => self.grid[2][1] = elem,
            Coord::BotRight => self.grid[2][2] = elem,
        }
    }

    pub fn is_full(&self) -> bool {
        self.is_full
    }

    pub fn get_cell(&self, coord: Coord) -> CellContent {
        match coord {
            Coord::TopLeft => self.grid[0][0],
            Coord::TopCenter => self.grid[0][1],
            Coord::TopRight => self.grid[0][2],
            Coord::MidLeft => self.grid[1][0],
            Coord::MidCenter => self.grid[1][1],
            Coord::MidRight => self.grid[1][2],
            Coord::BotLeft => self.grid[2][0],
            Coord::BotCenter => self.grid[2][1],
            Coord::BotRight => self.grid[2][2],
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " {} | {} | {}\n-----------\n {} | {} | {}\n-----------\n {} | {} | {}\n",
            self.get_cell(Coord::TopLeft),
            self.get_cell(Coord::TopCenter),
            self.get_cell(Coord::TopRight),
            self.get_cell(Coord::MidLeft),
            self.get_cell(Coord::MidCenter),
            self.get_cell(Coord::MidRight),
            self.get_cell(Coord::BotLeft),
            self.get_cell(Coord::BotCenter),
            self.get_cell(Coord::BotRight),
        )
    }
}
