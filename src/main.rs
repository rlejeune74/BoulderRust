mod board;
mod cell;
mod coordinate;
mod errors;
mod player;

use board::Board;

const DEFAULT_MAP: &str = "\
20 6 15\n\
--------------------\n\
--------------------\n\
x-x-x-x-x-x-x-x-x-x-\n\
-o-o---o--o---02-----\n\
---1x-2-x3-4--3-----\n\
--------------------\n";

fn main() {
    // simulate command `MAP [map.desc]`
    let mut board = TryInto::<Board>::try_into(DEFAULT_MAP)
        .map_err(|e| println!("Error: {}", e))
        .unwrap();
    println!("Map loaded");
    println!("-------------------------\n");
    println!("{}\n", board);

    // simulate command `PLAYER [NAME]`
    board
        .add_player("player1".to_string())
        .map_err(|e| println!("Error: {}", e))
        .unwrap();
    println!("Player added");
    println!("-------------------------\n");
    println!("{}\n", board);

    // simulate command `GAME START`
}
