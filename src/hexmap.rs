use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TerrainKind {
    Dirt,
    Sand,
    Stone,
    Ocean
}


// these are the pointy-topped-hexagon directions
#[derive(Copy, Clone)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE
}

pub struct Hexmap {
    size: i32,
    // FIXME: use point for location?
    // FIXME: make a Hexstack struct with piles of terrain
    // FIXME: put offset in the hexstack to pass around?
    pub hexes: HashMap<(i32,i32),TerrainKind>,
}

impl Hexmap {

    pub fn new(size: i32) -> Hexmap {
        let mut h = HashMap::new();
        
        // FIXME: generate map in game engine rather than fake data below.

        // FIXME: note that even-numbered maps are rouned up
        for r in -(size/2)..(size/2)+1 {
            for q in -(size/2)..(size/2)+1 {
                let z = -q-r;
                h.insert((r,q),
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
                });
                
            }
        }
        

        let m = Hexmap {
            size,
            hexes: h
        };
        m
    }


    pub fn get_ranked(&self, orientation: Direction) -> Vec<((i32,i32),&TerrainKind)> {
        match orientation {
            Direction::E  => self.get_ranked_e(),
            Direction::SE => self.get_ranked_se(),
            Direction::SW => unreachable!(),
            Direction::W  => unreachable!(),
            Direction::NW => unreachable!(),
            Direction::NE => unreachable!()
        }    
    
    }
    
    fn get_ranked_e(&self) -> Vec<((i32,i32),&TerrainKind)> {
    
        let mut v: (Vec<((i32,i32),&TerrainKind)>) = Vec::new();

        // This looks super-complicated but basically it's
        // https://www.redblobgames.com/grids/hexagons/#map-storage
        // for orientation East (top-left corner to bottom-right corner)
        for y in 0..self.size {
            let r=y-(self.size/2);
            for x in 0..self.size {
                let q=x-(self.size/2);
                let offset=(q*2+r,r);
                if let Some(hex) = self.hexes.get(&(q,r)) {
                    v.push((offset,hex));
                } else {
                    v.push((offset,&TerrainKind::Ocean));
                }
            }
        }
        v
    }

    fn get_ranked_se(&self) -> Vec<((i32,i32),&TerrainKind)> {
    
        let mut v: (Vec<((i32,i32),&TerrainKind)>) = Vec::new();


        // for orientation SouthEast, top row down
        for y in 0..self.size*2 {
            // FIXME: does rust have a trinary operator? that'd look
            // cleaner here
            // start pointy, get broad, back to pointy
            let w=self.size-(y-self.size).abs();
            for x in 0..w {
                let r=y-x;
                let q=x-r;
                let offset=(q*2+r,r);
                println!("{},{} : {},{} ({:?})",x,y,q,r,offset);
                if let Some(hex) = self.hexes.get(&(q,r)) {
                    v.push((offset,hex));
                } else {
                    v.push((offset,&TerrainKind::Ocean));
                }
            }
            println!("---");
        }
        v
    }

}
