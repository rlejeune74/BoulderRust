use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub struct Coordinate {
    x: u32,
    y: u32,
}
impl Coordinate {
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
