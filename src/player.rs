use crate::coordinate::Coordinate;
use std::fmt;

#[derive(PartialEq, Eq)]
pub enum Status {
    Idle,
    Dead,
}

pub struct Player {
    name: String,
    coord: Coordinate,
    score: u32,
    status: Status,
}

impl Player {
    pub fn new(name: String, coord: Coordinate) -> Player {
        Player {
            name,
            coord,
            score: 0,
            status: Status::Idle,
        }
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.coord, self.score)
    }
}
