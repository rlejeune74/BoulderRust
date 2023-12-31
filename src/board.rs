use crate::cell::Cell;
use std::str::FromStr;

pub struct Board{
    width:u32,
    height:u32,
    diamands:u32,
    map:Vec<Vec<Cell>>
}
impl Board {
    pub fn new(input:&str) -> Board{
        let mut board = Board{width: 0, height: 0, diamands: 0, map:Vec::new()};
        for line in input.lines(){
            if board.width == 0 {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                if parts.len() != 3 {
                    panic!("Height or width missing in map definition.")
                }
                board.width = FromStr::from_str(parts[0]).unwrap();
                board.height = FromStr::from_str(parts[1]).unwrap();
                board.diamands = FromStr::from_str(parts[2]).unwrap();
                continue;
            }
            let mut cells = Vec::new();
            for char in line.chars() {
                cells.push(Cell::new(0, 0, char));
            }
            board.map.push(cells);
        }
        board
    }
    pub fn display(&self){
        for line in &self.map {
            let mut str_out = String::new();
            for char in line {
                str_out.push_str(char.to_string().as_str());
            }
            println!("{}\n", str_out);
        }
    }
}