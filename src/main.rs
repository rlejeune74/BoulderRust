mod board;
mod cell;
mod coordinate;
mod errors;
mod player;

use board::Board;

const DEFAULT_MAP:&str = "\
20 6 15\n\
--------------------\n\
--------------------\n\
x-x-x-x-x-x-x-x-x-x-\n\
-o-o---o--o---02-----\n\
---1x-2-x3-4--3-----\n\
--------------------\n";


fn main() {
    let mut board = Board::new();

    // simulate command `MAP [map.desc]`
    match board.load_map(DEFAULT_MAP) {
        Err(e) => println!("Error: {}", e),
        _ => {
            println!("Map loaded");
            println!("-------------------------\n");
            println!("{}\n", board);
        }
    }

    // simulate command `PLAYER [NAME]`
    match board.add_player("player1".to_string()) {
        Err(e) => println!("Error: {}", e),
        _ => {
            println!("Player added");
            println!("-------------------------\n");
            println!("{}\n", board);
        }
    }

    // simulate command `GAME START`
    
}
