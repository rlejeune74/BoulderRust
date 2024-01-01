use crate::cell::Cell;
use crate::errors::StrError;
use std::str::FromStr;

pub struct Board{
    width:u32,
    height:u32,
    diamands:u32,
    map:Vec<Vec<Cell>>
}
impl Board {
    pub fn new() -> Board {
        Board{width:0, height:0, diamands:0, map:Vec::new()}
    }

    pub fn load_map(&mut self, map:&str) -> Result<(), StrError>{
        let lines:Vec<&str> = map.lines().collect();
        if lines.is_empty() {
            return Err(StrError::new("Map is empty".to_string()));
        } else {
            let line = lines[0];
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts.len() != 3 {
                panic!("Height or width missing in map definition.")
            }
            self.width = FromStr::from_str(parts[0]).unwrap();
            self.height = FromStr::from_str(parts[1]).unwrap();
            self.diamands = FromStr::from_str(parts[2]).unwrap();
        }

        if self.height != (lines.len() - 1) as u32 {
            return Err(StrError::new("Height param and number of lines differ".to_string()));
        }
        // Number of line include the parameter line
        for i in 1..(self.height+1) as usize{
            let line = lines[i];
            let mut cells = Vec::new();
            let chars = line.chars().collect::<Vec<char>>();
            let mut j:u32 = 0;
            let mut _j = 0;
            while _j < chars.len() as usize {
                let mut cell = Cell::new(j as u32, i as u32, chars[_j]);
                if cell.has_hidden_diamands() && _j < (chars.len() - 1){
                    _j += 1;
                    cell.set_diamands( chars[_j].to_digit(10).unwrap());
                }
                cells.push(cell);
                j += 1;
                _j+=1;
            }
            // force missing cell description to be a solid Rock.
            while j < self.width {
                cells.push(Cell::new(j as u32, i as u32, 'x'));
                j+=1;
            }
            self.map.push(cells);
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{} {} {}\n", self.width, self.height, self.diamands));
        for row in &self.map {
            let mut str_out = String::new();
            for cell in row {
                str_out.push_str(cell.to_string().as_str());
            }
            output += &str_out;
            output.push('\n');
        }
        return output;
    }

    fn get(&self, x:u32, y:u32) -> Option<&Cell> {
        self.map.get(x as usize)?.get(y as usize)
    }
}