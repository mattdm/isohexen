extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::cmp;

use hexgeometry::Direction;
use hexgeometry::Hexpoint;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TerrainKind {
    Dirt,
    Sand,
    Stone,
    Grass, // FIXME: this should be a decoration rather than a terrain type
    //Ocean
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
        let center_tile = Hexpoint::new(0,0);
        let center_height = rng.gen::<isize>()%12+24; // FIXME: magic numbers!
        self.hexes.insert(center_tile, vec![TerrainKind::Stone;center_height as usize]);
        
        // mountain arms
        for arm in Hexpoint::new(1,0).ring() { // This might be better as recursive, but
                                               // rust and recursion don't seem to be friends.
            let mut height = center_height;     // maybe... something with pushing onto vectors?
            let mut parent = center_tile;
            let mut tile = arm; 

            // FIXME: this is spaghetti mess.
            // FIXME: recursive version had possibility of arms branching (fractal-style!)
            // and we lost that... add back.
            while height > 2 {
                // change height -3 + random(0..5), max 1
                height = cmp::max(1,height + rng.gen::<isize>()%6 - 3);
                self.hexes.insert(tile, vec![TerrainKind::Stone;height as usize]);
                // One-in-six chance of arm ending here.
                if rng.gen_weighted_bool(6) {
                    break;
                }
                let mut next=tile.neighbor(parent.direction_to(tile));
                // coin flip for direction to go
                if rng.gen_weighted_bool(2) {
                    if rng.gen_weighted_bool(2) {
                        next=tile.neighbor(parent.direction_to(tile).clockwise());
                    } else {
                        next=tile.neighbor(parent.direction_to(tile).counterclockwise())
                    }
                }
                parent = tile;
                tile = next;
            }
        }
        
        // fill in dirt and sand between arms of mountain
        for ring in 1..15 { // FIXME: scale based on passed-in size parameter
            for tile in Hexpoint::new(ring,0).ring() {

                if self.hexes.get(&tile).is_none() {
                
                    let mut neighbor_height = 0;
                    let mut neighbor_count = 0;
                    let mut inner_sand = false;
                    let mut inner_ocean = 0;
                    
                    // get average height of inward neighbors
                    for neighbor in tile.inward_neighbors() {
                        match self.hexes.get(&neighbor) {
                            Some(neighbor_hex) => {
                                    neighbor_height += neighbor_hex.len();
                                    neighbor_count += 1;
                                    if neighbor_hex[0] == TerrainKind::Sand {
                                        inner_sand = true;
                                    }
                                }
                            None => inner_ocean += 1,
                        }
                    }
                    // half of average of inward heights.
                    let mut height = cmp::max(1,neighbor_height/(cmp::max(1,neighbor_count*2)));
                    // add some variation
                    if height > 2 {
                        height += rng.gen::<usize>()%2 + rng.gen::<usize>()%2;
                    }
                    
                    
                    // FIXME: 7, 11, and 16 are magic numbers (scale to size parameter)
                    if ring > 11 {
                        // outer ring: water and height 1 or 2 sand
                        // if both inner neighbors are water, leave this as water.
                        // otherwise, chance of sand
                        if inner_ocean == 1 {
                            if rng.gen_weighted_bool(3) {
                                self.hexes.insert(tile, vec![TerrainKind::Sand;rng.gen::<usize>()%2+1]);
                            }
                        } else if inner_ocean == 0 {
                            if ! rng.gen_weighted_bool(4) {
                                self.hexes.insert(tile, vec![TerrainKind::Sand;rng.gen::<usize>()%2+1]);
                            }
                        }
                        
                    } else if ring > 7 && (inner_sand || rng.gen_weighted_bool(16-ring as u32)) {
                        // chance of sand
                        self.hexes.insert(tile, vec![TerrainKind::Sand;height+rng.gen::<usize>()%3]);
                    } else {
                        // inland: just dirt and grass
                        self.hexes.insert(tile, vec![TerrainKind::Dirt;height]);
                        self.hexes.get_mut(&tile).unwrap().push(TerrainKind::Grass);
                    }
                    
                }
            }
        }

        self.size = 31;
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
