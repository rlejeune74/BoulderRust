use crate::coordinate::Coordinate;

enum Status{
    Idle,
    Pending,
    Dead
}
pub struct Player {
    name: String,
    coord: Coordinate,
    score: u32,
    status: Status
}

impl Player {
    pub fn new(name:String) -> Player {
        Player{name, coord:Coordinate::new(0, 0), score:0, status:Status::Idle}
    }
}