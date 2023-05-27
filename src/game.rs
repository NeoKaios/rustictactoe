use crate::{
    grid::{cell_content::CellContent, grid::Grid},
    input_system::{char_to_coord, player_input},
    player::Player,
    victory_checker::check_victory,
};

pub struct Game {
    grid: Grid,
    active_player: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            grid: Grid::new(),
            active_player: Player::O,
        }
    }

    pub fn play(self: &mut Game) {
        self.grid.show();
        while !self.grid.is_full() {
            println!("Player {}, Input your move (1 - 9):", self.active_player);
            let coord = char_to_coord(player_input(false));
            if self.grid.get_cell(coord) == CellContent::Empty {
                self.grid.set_cell_to_player(self.active_player, coord);
                self.grid.show();
                if check_victory(&self.grid, Some(coord)) {
                    println!("\nCongrats, player {} won !", self.active_player);
                    break;
                }
                self.active_player.switch();
            } else {
                println!("This cell is already full");
            }
        }
    }
}
