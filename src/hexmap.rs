use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TerrainKind {
    //Dirt,
    //Grass,
    //Sand,
    Stone,
    Ocean
}

pub struct Hexmap {
    pub hexes: HashMap<(i8,i8),TerrainKind>
}

impl Hexmap {
    pub fn new(size: i8) -> Hexmap {
        assert!(size & 1 == 1); // must be odd
        let mut h = HashMap::new();
        // hexagon!
        let lower =0-size/2;
        let upper =size/2+1;
        
        let mut s = 0;
        let mut e = upper;
        
        for q in lower..upper {
            for r in s..e {
                h.insert((q,r), TerrainKind::Stone);
            }
            if q<0 {
                s=s-1;
            } else {
                e=e-1;
            }
        }
        h.insert((0,0),TerrainKind::Ocean);
        
        let m = Hexmap {
            hexes: h
        };
        m

    }
}
