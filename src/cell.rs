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
        'x' => CellType::Rock,
        'o' => CellType::Boulder,
        '0' => CellType::BoulderAndDiams,
        _ => CellType::Mud
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
            CellType::BoulderAndDiams => "0".to_string(),
            CellType::Mud => {
                if self.diamands == 0 {
                    " ".to_string()
                } else {
                    self.diamands.to_string()
                }
            } 
        }
    }
}