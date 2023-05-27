use crate::grid::coord::Coord;

pub fn player_input(has_failed: bool) -> char {
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

pub fn char_to_coord(num: char) -> Coord {
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

