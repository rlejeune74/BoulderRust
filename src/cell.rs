use crate::coordinate::Coordinate;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
enum CellType {
    Mud,
    Rock,
    Boulder,
    BoulderAndDiams
}

pub struct Cell{
    coord: Coordinate,
    diamands:u32,
    cell_type:CellType
}

impl Cell{
    const MUD_CHAR:char = '-';
    const ROCK_CHAR:char = 'x';
    const BOULDER_CHAR:char = 'o';
    const BOULDER_DIAM_CHAR:char = '0';

    pub fn new(x:u32, y:u32, data:char) -> Cell {
        let _type = Cell::char_to_cell_type(data);
        let diams = {
            if _type == CellType::Mud && data != Cell::MUD_CHAR {
                FromStr::from_str(&data.to_string()).unwrap()
            } else {
             0
            }
        };
        
        Cell{coord: Coordinate::new(x, y), diamands:diams, cell_type: _type}
    }

    pub fn has_hidden_diamands(&self) -> bool {
        self.cell_type == CellType::BoulderAndDiams
    }

    pub fn set_diamands(&mut self, diams:u32) {
        self.diamands = diams;
    }

    pub fn match_coord(&self, x:u32, y:u32) -> bool {
        self.coord == Coordinate::new(x, y)
    }

    fn char_to_cell_type(data:char) -> CellType {
        match data {
            Cell::BOULDER_CHAR => CellType::Boulder,
            Cell::BOULDER_DIAM_CHAR => CellType::BoulderAndDiams,
            '1'..='9' | Cell::MUD_CHAR => CellType::Mud,
             _ => CellType::Rock
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self.cell_type {
            CellType::Rock => Cell::ROCK_CHAR.to_string(),
            CellType::Boulder => Cell::BOULDER_CHAR.to_string(),
            CellType::BoulderAndDiams => format!("{}{}", Cell::BOULDER_DIAM_CHAR, self.diamands).to_string(),
            CellType::Mud => {
                if self.diamands == 0 {
                    Cell::MUD_CHAR.to_string()
                } else {
                    self.diamands.to_string()
                }
            } 
        };
        write!(f, "{}", output)
    }
}