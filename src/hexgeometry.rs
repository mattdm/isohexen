use std::ops::Sub;
use std::ops::Add;

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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Hexpoint {
    pub x: i32,
    pub y: i32,
    z: i32,
}

impl Hexpoint {
    
    pub fn new(x: i32, y: i32) -> Hexpoint {
        Hexpoint {
            x,
            y,
            z: -x-y,
        }
    }
    
    pub fn neighbor(&self,direction: Direction) -> Hexpoint{
        match direction {
            Direction::E  => Hexpoint::new(self.x+1,self.y  ),
            Direction::SE => Hexpoint::new(self.x  ,self.y+1),
            Direction::SW => Hexpoint::new(self.x-1,self.y+1),
            Direction::W  => Hexpoint::new(self.x-1,self.y  ),
            Direction::NW => Hexpoint::new(self.x  ,self.y-1),
            Direction::NE => Hexpoint::new(self.x+1,self.y-1),
        }
    }

    pub fn neighbors(&self) -> Vec<Hexpoint>{
        vec![Hexpoint::new(self.x+1,self.y  ),
             Hexpoint::new(self.x  ,self.y+1),
             Hexpoint::new(self.x-1,self.y+1),
             Hexpoint::new(self.x-1,self.y  ),
             Hexpoint::new(self.x  ,self.y-1),
             Hexpoint::new(self.x+1,self.y-1)]
    }

    pub fn inward_neighbors(&self) -> Vec<Hexpoint>{
    
        // This could be optimized in many ways, but 
        // I don't think it's a bottleneck anywhere...
        let mut v=Vec::new();
        for n in self.neighbors() {
            if n.ring_number() == self.ring_number() -1 {
                v.push(n);
            }
        }
        v
    }
    
    pub fn ring_number(&self) -> i32 {
	self.x.abs().max(self.y.abs().max(self.z.abs()))
    }
    
    pub fn ring(&self) -> Vec<Hexpoint> {
        let mut v = Vec::new();
        let r = self.ring_number();
        
        // top
        for i in 0..r {
            v.push(Hexpoint::new(i,-r));
        } 
        // top-right
        for i in 0..r {
            v.push(Hexpoint::new(r,-r+i));
        }
        // bottom-right
        for i in 0..r {
            v.push(Hexpoint::new(r-i,i));
        }
        // bottom
        for i in 0..r {
            v.push(Hexpoint::new(-i,r));
        } 
        // bottom-left
        for i in 0..r {
            v.push(Hexpoint::new(-r,r-i));
        }
        // top-left
        for i in 0..r {
            v.push(Hexpoint::new(i-r,-i));
        }
        v
    }
    
    pub fn direction_to(&self, other: Hexpoint) -> Direction {
        match other-*self {
            Hexpoint {x:  1, y:  0, z: -1} => Direction::E,
            Hexpoint {x:  0, y:  1, z: -1} => Direction::SE,
            Hexpoint {x: -1, y:  1, z:  0} => Direction::SW,
            Hexpoint {x: -1, y:  0, z:  1} => Direction::W,
            Hexpoint {x:  0, y: -1, z:  1} => Direction::NW,
            Hexpoint {x:  1, y: -1, z:  0} => Direction::NE,
            _ => unreachable!(),
        }
    }

}

impl Sub for Hexpoint {
    type Output = Hexpoint;

    fn sub(self, other: Hexpoint) -> Hexpoint {
        Hexpoint {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Hexpoint {
    type Output = Hexpoint;
    
    fn add(self, other: Hexpoint) -> Hexpoint {
        Hexpoint {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


