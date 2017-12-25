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
    radius: i32,
    pub hexes: HashMap<(i32,i32),TerrainKind>,
}

impl Hexmap {

    pub fn new(radius: i32) -> Hexmap {
        let mut h = HashMap::new();

        h.insert(( 0,-3),TerrainKind::Stone);
        h.insert(( 3,-3),TerrainKind::Stone);
        h.insert(( 3, 0),TerrainKind::Stone);
        h.insert(( 0, 3),TerrainKind::Stone);
        h.insert((-3,-3),TerrainKind::Stone);
        h.insert((-3, 0),TerrainKind::Stone);
        h.insert(( 0, 0),TerrainKind::Stone);
        
        let m = Hexmap {
            radius,
            hexes: h
        };
        m
    }
    
    pub fn get_ranked_map(&self) -> Vec<((i32,i32),&TerrainKind)> {
        let mut v: (Vec<((i32,i32),&TerrainKind)>) = Vec::new();
        
        let mut zigzag=0;
        for r in (-self.radius+1)..(self.radius) {
            if r & 1 == 1 {  // odd row
                zigzag=1;
            } else {
                zigzag=0;
            }
            for q in (-self.radius+1)..(self.radius) {
               let offset=(q*2+zigzag,r);
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
