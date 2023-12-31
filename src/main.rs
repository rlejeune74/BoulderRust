mod board;
mod cell;
mod coordinate;

use board::Board;

const DEFAULT_MAP:&str = "\
20 6 15\n\
xxxxxxxxxxxxxxxxxxxx\n\
x                  x\n\
x x x x x x x x x xx\n\
xo o   o  o   o    x\n\
x  1x 2 x3 4  5    x\n\
xxxxxxxxxxxxxxxxxxxx\n";


fn main() {
    let map = Board::new(DEFAULT_MAP);
    map.display();
}
