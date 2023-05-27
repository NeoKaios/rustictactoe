mod game;
mod grid;
mod input_system;
mod player;
mod victory_checker;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.play();
}
