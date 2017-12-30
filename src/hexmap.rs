extern crate rand;

use std::collections::HashMap;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub enum TerrainKind {
    Dirt,
    Sand,
    Stone,
    Grass, // FIXME: this should be a decoration rather than a terrain type
    Ocean
}


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

#[derive(PartialEq, Eq, Hash)]
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
}

pub struct Hexmap {
    size: i32,
    // FIXME: use point for location?
    // FIXME: put offset in the hexstack to pass around?
    pub hexes: HashMap<Hexpoint,Vec<TerrainKind>>,
}

impl Hexmap {

    pub fn new() -> Hexmap {
        let m = Hexmap {
            size: 0,
            hexes: HashMap::new()
        };
        m
    }

    pub fn generate(&mut self,size: i32) {
        let mut h = HashMap::new();
        let mut rng = rand::thread_rng();
        
        h.insert(Hexpoint::new(0,0), vec![TerrainKind::Stone;rng.gen::<usize>()%5+5]);
        
        /*
        // FIXME: note that even-numbered maps are rouned up
        for r in -(size/2)..(size/2)+1 {
            for q in -(size/2)..(size/2)+1 {
                let z = -q-r;
                h.insert((r,q),vec![
                match z.abs().max(r.abs().max(q.abs())) {
                    0 => TerrainKind::Ocean,
                    1 => TerrainKind::Stone,
                    2 => TerrainKind::Stone,
                    3 => TerrainKind::Stone,                
                    4 => TerrainKind::Dirt,                
                    5 => TerrainKind::Dirt,                
                    6 => TerrainKind::Sand,                
                    7 => if q%2==0 { TerrainKind::Sand } else { TerrainKind::Dirt }
                    8 => if r%2==0 { TerrainKind::Sand } else { TerrainKind::Stone },
                    _ => TerrainKind::Ocean,
                }]);
            }
        }
        */

        self.size = size;
        self.hexes = h;
    }


    pub fn get_ranked(&self, orientation: Direction) -> Vec<((i32,i32),Option<&Vec<TerrainKind>>)> {
        match orientation {
            Direction::E  => self.get_ranked_horizontal(1),
            Direction::SE => self.get_ranked_diagonal(1),
            Direction::SW => self.get_ranked_vertical(1),
            Direction::W  => self.get_ranked_horizontal(-1),
            Direction::NW => self.get_ranked_diagonal(-1),
            Direction::NE => self.get_ranked_vertical(-1),
        }    
    }
    
    fn get_ranked_horizontal(&self,flip: i32) -> Vec<((	i32,i32),Option<&Vec<TerrainKind>>)> {
    
        let mut v: (Vec<((i32,i32),Option<&Vec<TerrainKind>>)>) = Vec::new();

        // This looks super-complicated but basically it's
        // https://www.redblobgames.com/grids/hexagons/#map-storage
        // for orientation East (top-left corner to bottom-right corner)
        // or West (flip = -1)
        
        for y in 0..self.size {
            let r=flip*(y-(self.size/2));
            for x in 0..self.size {
                let q=flip*(x-(self.size/2));
                let offset=(((x-(self.size/2))*2+(y-(self.size/2))),y-(self.size/2));
                v.push((offset,self.hexes.get(&Hexpoint::new(q,r))));
            }
        }
        v
    }

    fn get_ranked_vertical(&self,flip: i32) -> Vec<((i32,i32),Option<&Vec<TerrainKind>>)> {
    
        let mut v: (Vec<((i32,i32),Option<&Vec<TerrainKind>>)>) = Vec::new();

        // Same as above, but we're going through columns
        // first instead of rows (effectively a 90° rotation from
        // the other function
        for y in 0..self.size {
            let q=flip*(y-(self.size/2));
            for x in 0..self.size {
                let r=flip*(x-(self.size/2));
                let offset=(-1*((x-(self.size/2))*2+(y-(self.size/2))),y-(self.size/2));
                v.push((offset,self.hexes.get(&Hexpoint::new(q,r))));
            }
        }
                
        v
    }

    fn get_ranked_diagonal(&self,flip: i32) -> Vec<((i32,i32),Option<&Vec<TerrainKind>>)> {
    
        let mut v: (Vec<((i32,i32),Option<&Vec<TerrainKind>>)>) = Vec::new();

        // for orientation SouthEast, top row down
        // flip for NW. Kind of ugly. Could be prettier.
        for y in 0..self.size*2 {
            // start pointy, get broad, back to pointy
            let w=self.size-((y-self.size).abs()-1);
            for x in 0..w+self.size-3 { // FIXME: erm, I'm not sure why this upper bound works. but it does.
                let r=flip*(y-x-self.size/2);
                let q=flip*(y-self.size/2-flip*r-self.size/2);
                let offset=(x*2-y,y-self.size+1);
                v.push((offset,self.hexes.get(&Hexpoint::new(q,r))));
            }
        }
        v
    }

}
