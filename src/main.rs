mod board;
mod cell;
mod coordinate;
mod errors;

use board::Board;

const DEFAULT_MAP:&str = "\
20 6 15\n\
xxxxxxxxxxxxxxxxxxxx\n\
x                  x\n\
x x x x x x x x x xx\n\
xo o   o  o   02    x\n\
x  1x 2 x3 4  3    x\n\
xxxxxxxxxxxxxxxxxxxx\n";


fn main() {
    let mut board = Board::new();
    match board.load_map(DEFAULT_MAP) {
        Err(e) => println!("Error: {}", e),
        _ => println!("Map loaded")
    }
    println!("-------------------------\n");
    println!("{}\n", board.to_string());
}
