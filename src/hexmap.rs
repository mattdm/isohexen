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
        // hexagon!
        let lower =0-radius;
        let upper =radius+1;
        
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
            radius,
            hexes: h
        };
        m
    }
    
    pub fn get_ranked_map(&self) -> Vec<((i32,i32),&TerrainKind)> {
        let mut v: (Vec<((i32,i32),&TerrainKind)>) = Vec::new();
        
        for rank in (-self.radius+1)..(self.radius*2-3) {
           for i in 0..self.radius+1 {
              // hex is -self.radius+i*2,rank-i
              let offset=(i*2,rank*2);
              if let Some(hex) = self.hexes.get(&(-self.radius+i*2,rank-i)) {
                  v.push((offset,hex));
              } else {
                  v.push((offset,&TerrainKind::Ocean));
              }
           }
           for i in 0..self.radius {
               // hex is -self.radius+1+i*2,rank-i
               let offset=(i*2+1,rank*2+1);
               if let Some(hex) = self.hexes.get(&(-self.radius+1+i*2,rank-i)) {
                   v.push((offset,hex));
               } else {
                   v.push((offset,&TerrainKind::Ocean));
               }
           }
        }
        v
    }


}
