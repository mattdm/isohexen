extern crate rand;

use std::collections::HashMap;
use rand::Rng;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TerrainKind {
    Dirt,
    Sand,
    Stone,
    Grass, // FIXME: this should be a decoration rather than a terrain type
    //Ocean
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

    pub fn generate(&mut self) {
        self.hexes = HashMap::new();
        
        let mut rng = rand::thread_rng();
        
        // center peak
        let center_tile= Hexpoint::new(0,0);
        self.hexes.insert(center_tile, vec![TerrainKind::Stone;rng.gen::<usize>()%8+16]);
        
        // core ring
        for t in Hexpoint::new(1,0).ring() {
            self.hexes.insert(t, vec![TerrainKind::Stone;rng.gen::<usize>()%4+16]);
        }
        
        // mountain "arms"
        let mut dirs = [Direction::E,Direction::SE,Direction::SW,Direction::W,Direction::NW,Direction::NE];
        rng.shuffle(&mut dirs);
        // this gives us a weighted chance of number of arms, tending towards 3
        //let armdirs = dirs.get(0..1).unwrap();
        let armdirs = match rng.gen::<usize>()%12 {
            0    => dirs.get(0..1).unwrap(),
            1|2|3|4 => dirs.get(0..2).unwrap(),
            5|6|7|8|9|10 => dirs.get(0..3).unwrap(),
            11 => dirs.get(0..4).unwrap(),
            _ => unreachable!(),
        };
        for armdir in armdirs {
            // note -- this will become a recursive function once I figure out what I want it to do
            let a1_tile=center_tile.neighbor(*armdir);
            let a1_height=self.hexes[&a1_tile].len();
            self.hexes.insert(a1_tile.neighbor(*armdir),                   vec![TerrainKind::Grass;rng.gen::<usize>()%4+a1_height/4*3]);
            self.hexes.insert(a1_tile.neighbor(armdir.clockwise()),        vec![TerrainKind::Dirt;rng.gen::<usize>()%4+a1_height/2]);
            self.hexes.insert(a1_tile.neighbor(armdir.counterclockwise()), vec![TerrainKind::Dirt;rng.gen::<usize>()%4+a1_height/2]);
            let mut a2_tile =  Hexpoint::new(0,0); // temporary
            
            if rng.gen_weighted_bool(2) { //coin flip
                a2_tile = a1_tile.neighbor(*armdir);
            } else if rng.gen_weighted_bool(2) { // another coin flip
                a2_tile = a1_tile.neighbor(armdir.clockwise());    
            } else {
                a2_tile = a1_tile.neighbor(armdir.counterclockwise());
            }
            let a2_height=self.hexes[&a2_tile].len();
            self.hexes.insert(a2_tile.neighbor(*armdir),                   vec![TerrainKind::Sand;rng.gen::<usize>()%4+a2_height/4*3]);
            self.hexes.insert(a2_tile.neighbor(armdir.clockwise()),        vec![TerrainKind::Stone;rng.gen::<usize>()%4+a2_height/2]);
            self.hexes.insert(a2_tile.neighbor(armdir.counterclockwise()), vec![TerrainKind::Stone;rng.gen::<usize>()%4+a2_height/2]);
            
        };
        
        
        /*
        for i in 4..7 {
            for t in Hexpoint::new(i,0).ring() {
                self.hexes.insert(t, vec![TerrainKind::Stone;rng.gen::<usize>()%2+1]);
                self.hexes.get_mut(&t).unwrap().push(TerrainKind::Dirt);
                if rng.gen::<usize>()%3 > 0 {
                    self.hexes.get_mut(&t).unwrap().push(TerrainKind::Grass);
                }
            }
        }
        for i in 7..10 {
            for t in Hexpoint::new(i,0).ring() {
                self.hexes.insert(t, vec![TerrainKind::Stone]);
                self.hexes.get_mut(&t).unwrap().push(TerrainKind::Dirt);
                if rng.gen::<usize>()%3 > 0 {
                    self.hexes.get_mut(&t).unwrap().push(TerrainKind::Grass);
                }
            }
        }
        for i in 10..12 {
            for t in Hexpoint::new(i,0).ring() {
                self.hexes.insert(t, vec![TerrainKind::Sand;rng.gen::<usize>()%2+1]);
            }
        }
        for i in 12..13 {
            for t in Hexpoint::new(i,0).ring() {
                if rng.gen::<usize>()%3 > 0 {
                    self.hexes.insert(t, vec![TerrainKind::Sand]);
                }
            }
        }
        for i in 13..15 {
            for t in Hexpoint::new(i,0).ring() {
                if rng.gen::<usize>()%4 == 0 {
                    self.hexes.insert(t, vec![TerrainKind::Sand]);
                }
            }
        }
        */
        
        
        self.size = 29;
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
