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
    size: i32,
    pub hexes: HashMap<(i32,i32),TerrainKind>,
}

impl Hexmap {

    // Note "rank" not row because we're going from
    // back to front taking offset rows
    // fixme -- do rotation here!
    
    // fixme: I am super hating this whole function.
    
    pub fn getrank(&self, rank: i32) -> Vec<&TerrainKind> {
        let mut v: Vec<&TerrainKind> = Vec::new();
        let mut sq = 0;
        let mut sr = 0;
        let mut er = 0;
        
        // watch the off-by-one errors here.
        // I made start and end EQUAL the numbers,
        // while the for loop leaves off the last one.
        
        if rank < self.size/2 {
            // pointy top of hexagon
            sq=0-rank;
            sr=(0-self.size/2)+rank;
            er=0-self.size/2;
        } else if rank >= (self.size*2-1)-(self.size/2) {
          // pointy bottom of hexagon
            sq=rank-(self.size*2-1);
            sr=self.size/2;
            er=rank-((self.size*2-1)-(self.size/2));
        } else {
          // the middle -- alternate even and odd
          if rank & 1 == 1 { // odd
              sq=0-self.size/2;
              sr=rank/2-1; // double check this
              er=sr+self.size/2+1;
          } else { //even
              sq=0-self.size/2+1;
              sr=rank/2-2; // FIXME this is almost certainly wrong
              er=sr+self.size/2; // ditto
          }
        }
        
        let mut q=sq;
        // rust can't count backwards, so this is inverted
        for r in er..sr+1 {
            let k=(q,-r);
            println!("{}: {},{}",rank,q,r);
            v.push(self.hexes.get(&k).expect("THIS WHOLE FUNCTION IS TERRIBLE"));
            q=q+2;
        }
        v
    }

    pub fn new(size: i32) -> Hexmap {
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
            size,
            hexes: h
        };
        m
    }
    
}
