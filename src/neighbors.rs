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
pub(crate) enum Direction {
    /// North
    N,
    /// North-east
    Ne,
    /// Eeast
    E,
    /// South-east
    Se,
    /// South
    S,
    /// South-west
    Sw,
    /// West
    W,
    /// North-west
    Nw,
}

impl Direction {
    pub fn to_tuple(&self) -> (i8, i8) {
        match self {
            Direction::Sw => (-1, -1),
            Direction::S => (-1, 0),
            Direction::Se => (-1, 1),
            Direction::W => (0, -1),
            Direction::E => (0, 1),
            Direction::Nw => (1, -1),
            Direction::N => (1, 0),
            Direction::Ne => (1, 1),
        }
    }
}
