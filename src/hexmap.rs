use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TerrainKind {
    Dirt,
    Grass,
    Sand,
    Stone,
    Ocean
}

pub struct Hexmap {
    radius: i32,
    pub hexes: HashMap<(i32,i32),TerrainKind>,
}

impl Hexmap {

    pub fn new(radius: i32) -> Hexmap {
        let mut h = HashMap::new();
        
        // FIXME: generate map rather than
        // fake data below.

        h.insert(( 0,-3),TerrainKind::Sand);
        h.insert(( 1,-3),TerrainKind::Sand);
        h.insert(( 2,-3),TerrainKind::Sand);
        h.insert(( 3,-3),TerrainKind::Sand);
        h.insert(( 3,-2),TerrainKind::Sand);
        h.insert(( 3,-1),TerrainKind::Sand);
        h.insert(( 3, 0),TerrainKind::Sand);
        h.insert(( 2, 1),TerrainKind::Sand);
        h.insert(( 1, 2),TerrainKind::Sand);
        h.insert(( 0, 3),TerrainKind::Sand);
        h.insert((-1, 3),TerrainKind::Sand);
        h.insert((-2, 3),TerrainKind::Sand);
        h.insert((-3, 3),TerrainKind::Sand);
        h.insert((-3, 2),TerrainKind::Sand);
        h.insert((-3, 1),TerrainKind::Sand);
        h.insert((-3, 0),TerrainKind::Sand);
        h.insert((-3, 0),TerrainKind::Sand);
        h.insert((-2,-1),TerrainKind::Sand);
        h.insert((-1,-2),TerrainKind::Sand);
        
        h.insert((0,0),TerrainKind::Stone);
        h.insert((-1,0),TerrainKind::Grass);
        h.insert((1,0),TerrainKind::Dirt);

        let m = Hexmap {
            radius,
            hexes: h
        };
        m
    }
    
    pub fn get_ranked(&self) -> Vec<((i32,i32),&TerrainKind)> {
        let mut v: (Vec<((i32,i32),&TerrainKind)>) = Vec::new();
        
        // FIXME: there's some way to simplify the
        // math with Q an S which I just haven't figured out
        let mut slant=self.radius-1;
        for r in -self.radius+1..self.radius {
            for q in  -self.radius+1..self.radius {
                let offset=(q*2+slant,r);
                if let Some(hex) = self.hexes.get(&(q,r)) {
                    v.push((offset,hex));
                } else {
                    v.push((offset,&TerrainKind::Ocean));
                }
            }
            slant=slant+1;
        }
        v
    }


}
