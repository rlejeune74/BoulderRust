use crate::cell::Cell;
use crate::coordinate::Coordinate;
use crate::errors::StrError;
use crate::player::{self, Player};

use core::panic;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

pub struct Board {
    width: u32,
    height: u32,
    diamands: u32,
    map: Vec<Vec<Cell>>,
    players: HashMap<String, Player>,
}
impl Board {
    pub fn new() -> Board {
        Board {
            width: 0,
            height: 0,
            diamands: 0,
            map: Vec::new(),
            players: HashMap::new(),
        }
    }

    pub fn load_map(&mut self, map: &str) -> Result<(), StrError> {
        let lines: Vec<&str> = map.lines().collect();
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
            return Err(StrError::new(
                "Height param and number of lines differ".to_string(),
            ));
        }
        // Number of line include the parameter line
        (1..(self.height + 1) as usize).for_each(|i| {
            let line = lines[i];
            let mut cells = Vec::new();
            let chars = line.chars().collect::<Vec<char>>();
            let mut j: u32 = 0;
            let mut _j = 0;
            while _j < chars.len() {
                let mut cell = Cell::new(j, (i - 1) as u32, chars[_j]);
                if cell.has_hidden_diamands() && _j < (chars.len() - 1) {
                    _j += 1;
                    cell.set_diamands(chars[_j].to_digit(10).unwrap());
                }
                cells.push(cell);
                j += 1;
                _j += 1;
            }
            // force missing cell description to be a solid Rock.
            while j < self.width {
                cells.push(Cell::new(j, (i - 1) as u32, 'x'));
                j += 1;
            }
            self.map.push(cells);
        });
        Ok(())
    }

    pub fn add_player(&mut self, name: String) -> Result<(), StrError> {
        if self.players.contains_key(&name) {
            return Err(StrError::new(
                format!("Player {} already exists", name).to_string(),
            ));
        }
        match self.players.len() {
            0 => self.players.insert(
                name.clone(),
                Player::new(name.clone(), Coordinate::new(0, 0)),
            ),
            1 => self.players.insert(
                name.clone(),
                Player::new(name.clone(), Coordinate::new(self.width - 1, 0)),
            ),
            2 => self.players.insert(
                name.clone(),
                Player::new(name.clone(), Coordinate::new(0, self.height - 1)),
            ),
            3 => self.players.insert(
                name.clone(),
                Player::new(
                    name.clone(),
                    Coordinate::new(self.width - 1, self.height - 1),
                ),
            ),
            _ => return Err(StrError::new("Max number of player reached".to_string())),
        };
        Ok(())
    }

    fn get(&self, x: u32, y: u32) -> Option<&Cell> {
        let cell = self.map.get(y as usize)?.get(x as usize);
        match cell {
            None => None,
            _ => {
                if cell.unwrap().match_coord(x, y) {
                    cell
                } else {
                    panic!("returned cell does not match asked coordinates {} {}", x, y);
                }
            }
        }
    }

    fn list_players_by_state(&self, status: player::Status, exclude: Option<bool>) -> Vec<&Player> {
        let mut count = Vec::new();
        let exclude = exclude.unwrap_or(false);
        for player in self.players.values() {
            match player.status() == &status {
                true => {
                    if !exclude {
                        count.push(player);
                    }
                }
                false => {
                    if exclude {
                        count.push(player);
                    }
                }
            };
        }
        count
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let players = self.list_players_by_state(player::Status::Dead, Some(true));
        if !players.is_empty() {
            output.push_str(&format!("{}\n", players.len()));
            for player in players {
                output += &player.to_string();
                output.push('\n');
            }
        }

        output.push_str(&format!(
            "{} {} {}\n",
            self.width, self.height, self.diamands
        ));
        for i in 0..self.height {
            let mut str_out = String::new();
            for j in 0..self.width {
                match self.get(j, i) {
                    Some(cell) => str_out.push_str(cell.to_string().as_str()),
                    None => panic!("internal error"),
                }
            }
            output += &str_out;
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}
