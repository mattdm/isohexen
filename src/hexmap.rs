use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TerrainKind {
    Dirt,
    Sand,
    Stone,
    Ocean
}

pub struct Hexmap {
    radius: i32,
    // FIXME: use point for location?
    // FIXME: make a Hexstack struct with piles of terrain
    // FIXME: put offset in the hexstack to pass around?
    pub hexes: HashMap<(i32,i32),TerrainKind>,
}

impl Hexmap {

    pub fn new(radius: i32) -> Hexmap {
        let mut h = HashMap::new();
        
        // FIXME: generate map in game engine rather than fake data below.

        for r in -radius+1..radius {
            for q in -radius+1..radius {
                let z = -q-r;
                match z.abs().max(r.abs().max(q.abs())) {
                    0 => h.insert((r,q),TerrainKind::Stone),
                    1 => h.insert((r,q),TerrainKind::Stone),
                    2 => h.insert((r,q),TerrainKind::Stone),
                    3 => h.insert((r,q),TerrainKind::Stone),                
                    4 => h.insert((r,q),TerrainKind::Dirt),                
                    5 => h.insert((r,q),TerrainKind::Dirt),                
                    6 => h.insert((r,q),TerrainKind::Sand),                
                    7 => h.insert((r,q),TerrainKind::Sand),                
                    8 => h.insert((r,q),TerrainKind::Sand),                
                    _ => None
                };
                
            }
        }
        

        let m = Hexmap {
            radius,
            hexes: h
        };
        m
    }
    
    pub fn get_ranked(&self) -> Vec<((i32,i32),&TerrainKind)> {
        let mut v: (Vec<((i32,i32),&TerrainKind)>) = Vec::new();

        // This looks super-comlicated but basically it's
        // https://www.redblobgames.com/grids/hexagons/#map-storage
        for r in -self.radius+1..self.radius {
            for q in -self.radius+1..self.radius {
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


}
