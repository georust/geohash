#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors {
    pub sw: String,
    pub s: String,
    pub se: String,
    pub w: String,
    pub e: String,
    pub nw: String,
    pub n: String,
    pub ne: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// North
    N,
    /// North-east
    NE,
    /// Eeast
    E,
    /// South-east
    SE,
    /// South
    S,
    /// South-west
    SW,
    /// West
    W,
    /// North-west
    NW,
}

impl Direction {
    pub fn to_tuple(self) -> (f64, f64) {
        match self {
            Direction::SW => (-1f64, -1f64),
            Direction::S => (-1f64, 0f64),
            Direction::SE => (-1f64, 1f64),
            Direction::W => (0f64, -1f64),
            Direction::E => (0f64, 1f64),
            Direction::NW => (1f64, -1f64),
            Direction::N => (1f64, 0f64),
            Direction::NE => (1f64, 1f64),
        }
    }
}
