use std::collections::HashMap;

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


pub struct Hexmap {
    size: i32,
    // FIXME: use point for location?
    // FIXME: put offset in the hexstack to pass around?
    pub hexes: HashMap<(i32,i32),Vec<TerrainKind>>,
}

impl Hexmap {

    pub fn new(size: i32) -> Hexmap {
        let mut h = HashMap::new();
        
        // FIXME: generate map in game engine rather than fake data below.

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
            h.insert((0,-8), vec![TerrainKind::Ocean]);
        h.insert(( 6, 8), vec![TerrainKind::Sand]);
        h.insert((-6,-8), vec![TerrainKind::Stone]);
        h.insert(( 6, 6), vec![TerrainKind::Dirt]);
        h.insert((-6,-6), vec![TerrainKind::Dirt]);
        h.insert((0,1), vec![TerrainKind::Sand]);
        h.insert((1,0), vec![TerrainKind::Dirt]);
        h.insert((1,1), vec![TerrainKind::Ocean]);
        h.insert((-2,1), vec![TerrainKind::Ocean]);
        h.insert((-3,-3), vec![TerrainKind::Grass]);

        let m = Hexmap {
            size,
            hexes: h
        };
        m
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
                v.push((offset,self.hexes.get(&(q,r))));
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
                v.push((offset,self.hexes.get(&(q,r))));
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
                v.push((offset,self.hexes.get(&(q,r))));
            }
        }
        v
    }

}
