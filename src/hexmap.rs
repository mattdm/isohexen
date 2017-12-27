use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TerrainKind {
    Dirt,
    Sand,
    Stone,
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
                h.insert((0,-8), TerrainKind::Dirt);
                h.insert(( 6, 8), TerrainKind::Sand);
                h.insert((-6,-8), TerrainKind::Stone);
                h.insert(( 6, 6), TerrainKind::Dirt);
                h.insert((-6,-6), TerrainKind::Dirt);
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
            Direction::E  => self.get_ranked_horizontal(1),
            Direction::SE => self.get_ranked_se(),
            Direction::SW => unreachable!(),
            Direction::W  => self.get_ranked_horizontal(-1),
            Direction::NW => unreachable!(),
            Direction::NE => unreachable!()
        }    
    }
    
    fn get_ranked_horizontal(&self,flip: i32) -> Vec<((i32,i32),&TerrainKind)> {
    
        let mut v: (Vec<((i32,i32),&TerrainKind)>) = Vec::new();

        // This looks super-complicated but basically it's
        // https://www.redblobgames.com/grids/hexagons/#map-storage
        // for orientation East (top-left corner to bottom-right corner)
        // or West (flip = -1)
        
        for y in 0..self.size {
            let r=flip*(y-(self.size/2));
            for x in 0..self.size {
                let q=flip*(x-(self.size/2));
                let offset=(flip*(q*2+r),r*flip);
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
            // start pointy, get broad, back to pointy
            let w=self.size-(y-self.size).abs()+1;
            for x in 0..w {
                let r=y-x-self.size/2;
                let q=(y-self.size/2)-r-self.size/2; 
                let offset=(x*2-w+1,y-self.size+1);
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
