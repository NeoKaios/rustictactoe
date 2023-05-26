mod grid;
mod player;
mod victory_checker;

use grid::{cell_content::CellContent, coord::Coord, grid::Grid};
use player::Player;
use victory_checker::check_victory;

fn player_input(has_failed: bool) -> char {
    let mut s = String::new();
    if has_failed {
        println!("Incorrect input please enter your move again:");
    }
    std::io::stdin().read_line(&mut s).unwrap();
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let char = s.chars().next().unwrap_or('x');
    if char >= '1' && char <= '9' {
        char
    } else {
        player_input(true)
    }
}

fn char_to_coord(num: char) -> Coord {
    match num {
        '1' => Coord::TopLeft,
        '2' => Coord::TopCenter,
        '3' => Coord::TopRight,
        '4' => Coord::MidLeft,
        '5' => Coord::MidCenter,
        '6' => Coord::MidRight,
        '7' => Coord::BotLeft,
        '8' => Coord::BotCenter,
        '9' => Coord::BotRight,
        _ => panic!("Unexpected char given"),
    }
}

fn get_player_action(player: Player) -> Coord {
    println!("Player {}, Input your move (1 - 9):", player);
    char_to_coord(player_input(false))
}

fn main() {
    let mut g = Grid::new();
    g.show();
    let mut player = Player::O;
    while !g.is_full() {
        let coord = get_player_action(player);
        if g.get_cell(coord) == CellContent::Empty {
            g.set_cell_player(player, coord);
            g.show();
            if check_victory(&g, Some(coord)) {
                println!("\nCongrats, player {} won !", player);
                break;
            }
            player.switch();
        } else {
            println!("This cell is already full");
        }
    }
}
