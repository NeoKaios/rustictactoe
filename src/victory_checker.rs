use crate::grid::{coord::Coord, grid::Grid};
use Coord::{ TopLeft, TopCenter, TopRight, MidLeft, MidCenter, MidRight, BotLeft, BotCenter, BotRight};

fn check_state(g: &Grid, cells_to_check: [Coord; 3]) -> bool {
    g.get_cell(cells_to_check[0]) == g.get_cell(cells_to_check[1])
        && g.get_cell(cells_to_check[1]) == g.get_cell(cells_to_check[2])
}

pub fn check_victory(g: &Grid, coord: Option<Coord>) -> bool {
    let check_states = vec![
        // rows
        [TopLeft, TopCenter, TopRight],
        [MidLeft, MidCenter, MidRight],
        [BotLeft, BotCenter, BotRight],
        // columns
        [TopLeft, MidLeft, BotLeft],
        [TopCenter, MidCenter, BotCenter],
        [TopRight, MidRight, BotRight],
        // diags
        [TopLeft, MidCenter, BotRight],
        [TopRight, MidCenter, BotLeft],
    ];
    let filtered_states: Vec<[Coord; 3]> = match coord {
        Some(player_input) => check_states
            .into_iter()
            .filter(|state| state.into_iter().any(|&c| c == player_input))
            .collect(),
        None => check_states,
    };
    filtered_states.iter().any(|&state| check_state(g, state))
}
