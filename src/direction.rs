// these are the pointy-topped-hexagon directions
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE
}

impl Direction {
    pub fn clockwise(&self) -> Direction {
        match self {
            &Direction::E  => Direction::SE,
            &Direction::SE => Direction::SW,
            &Direction::SW => Direction::W,
            &Direction::W  => Direction::NW,
            &Direction::NW => Direction::NE,
            &Direction::NE => Direction::E,
        }
    }
    pub fn counterclockwise(&self) -> Direction {
        match self {
            &Direction::E  => Direction::NE,
            &Direction::SE => Direction::E,
            &Direction::SW => Direction::SE,
            &Direction::W  => Direction::SW,
            &Direction::NW => Direction::W,
            &Direction::NE => Direction::NW,
        }
    }
}
