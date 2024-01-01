use crate::coordinate::Coordinate;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
enum CellType {
    Mud,
    Rock,
    Boulder,
    BoulderAndDiams
}

fn char_to_cell_type(data:char) -> CellType {
    match data {
        'o' => CellType::Boulder,
        '0' => CellType::BoulderAndDiams,
        '1'..='9' | ' ' => CellType::Mud,
        'x' | _ => CellType::Rock
    }
}

pub struct Cell{
    coord: Coordinate,
    diamands:u32,
    cell_type:CellType
}

impl Cell{
    pub fn new(x:u32, y:u32, data:char) -> Cell {
        let _type = char_to_cell_type(data);
        let diams = {
            if _type == CellType::Mud && data != ' '{
                FromStr::from_str(&data.to_string()).unwrap()
            }else {
             0
            }
        };
        
        Cell{coord: Coordinate::new(x, y), diamands:diams, cell_type: _type}
         
    }

    pub fn to_string(&self) -> String{
        match self.cell_type {
            CellType::Rock => "x".to_string(),
            CellType::Boulder => "o".to_string(),
            CellType::BoulderAndDiams => format!("0{}", self.diamands).to_string(),
            CellType::Mud => {
                if self.diamands == 0 {
                    " ".to_string()
                } else {
                    self.diamands.to_string()
                }
            } 
        }
    }

    pub fn has_hidden_diamands(&self) -> bool {
        self.cell_type == CellType::BoulderAndDiams
    }

    pub fn set_diamands(&mut self, diams:u32) {
        self.diamands = diams;
    }
}