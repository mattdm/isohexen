// todo: move basically all of this to an external script that lives
// next to the tiles. ideally, anything that knows what a terrain type 
// actually *is* lives in a scripting language.

extern crate rand;

use rand::Rng;

use std::cmp;

use direction::Direction;
use hexgeometry::Hexpoint;
use hexgeometry::Hexmap;



pub struct Island<'a> {
    size: i32,
    // FIXME: put offset in the hexstack to pass around?
    pub map: Hexmap<'a>,
}

impl<'a> Island<'a> {

    pub fn new() -> Island<'a> {
        Island {
            size: 0,
            map: Hexmap::new(0)
        }
    }

    pub fn generate(&mut self, size: i32) {
        self.size= size;
        self.map = Hexmap::new(self.size);
        
        let mut rng = rand::thread_rng();


        /* Mountain */
                
        // center peak
        let center_tile = Hexpoint::new(0,0);
        let mountain_height = (size as isize*3)/4; // FIXME: make configurable
        // note "isize  %" can be negative
        let center_height = rng.gen::<isize>()%(mountain_height)/2+mountain_height;
        self.map.hexes.insert(center_tile, vec!["stone";center_height as usize]);
        
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
                // change height -3 + random(-6..6), but make it at least 1
                height = cmp::max(1,height + rng.gen::<isize>()%6 -3);
                self.map.hexes.insert(tile, vec!["stone";height as usize]);
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
        for ring in 1..size/2 {
            for tile in Hexpoint::new(ring,0).ring() {

                if self.map.hexes.get(&tile).is_none() {
                
                    let mut neighbor_height = 0;
                    let mut neighbor_count = 0;
                    let mut inner_sand = 0;
                    let mut inner_stone = 0;
                    let mut inner_ocean = 0;
                    
                    // get average height of inward neighbors
                    for neighbor in tile.inward_neighbors() {
                        match self.map.hexes.get(&neighbor) {
                            Some(neighbor_hex) => {
                                    neighbor_height += neighbor_hex.len();
                                    neighbor_count += 1;
                                    if neighbor_hex[0] == "sand" {
                                        inner_sand += 1;
                                    } else if neighbor_hex[0] == "stone" {
                                        inner_stone +=1;
                                    }
                                }
                            None => inner_ocean += 1,
                        }
                    }
                    // if no neighbors (ocean), height 1
                    // if one neighbor, half that height
                    // if two neigbors, 3/4 their average
                    let mut height = match neighbor_count {
                        0 => 1,
                        1 => cmp::max(1,neighbor_height/2),
                        _ => cmp::max(1,((neighbor_height*3)/(cmp::max(1,neighbor_count)))/4),
                    };
                        
                    let terrain: Option<&str>;
                    
                    if ring > (size*3)/8 {
                        // outer ring: water and height 1 or 2 sand
                        // if both inner neighbors are water, leave this as water.
                        // otherwise, chance of sand
                        if rng.gen_weighted_bool(4) {
                            // ocean
                            terrain=None;
                        } else if inner_ocean > 0 {
                            // more ocean
                            if rng.gen_weighted_bool(3-inner_ocean) {
                                terrain=None;
                            } else {
                                terrain=Some("sand");
                            }
                        } else if inner_sand == 0 {
                            // inside dirt
                            if rng.gen_weighted_bool(3) {
                                terrain=Some("dirt");
                            } else {
                                terrain=Some("sand");
                            }
                            height = 1 + rng.gen::<usize>()%2;
                        } else {
                            terrain=Some("sand");
                            height = 1;
                        }

                    } else if ring > size/8 {
                        // middle band
                        if rng.gen_weighted_bool(size as u32*3) || (inner_stone > 0 && rng.gen_weighted_bool(2)) {
                            // throw some boulders around
                            terrain=Some("stone");
                            height += rng.gen::<usize>()%(size as usize/8) + rng.gen::<usize>()%(size as usize/8);
                        } else if rng.gen_weighted_bool(((size-ring) as u32)/3) {
                            // scatter sand piles, too
                            terrain=Some("sand");
                            height += rng.gen::<usize>()%3 + rng.gen::<usize>()%3;
                        } else if inner_sand > 0 {
                            if rng.gen_weighted_bool(3-inner_sand) {
                                terrain=Some("sand");
                            } else {
                                terrain=Some("dirt");
                            }
                        } else {
                            terrain=Some("dirt");
                            if ring<size/5 {
                                height += rng.gen::<usize>()%2 + rng.gen::<usize>()%2;
                            }
                        }
                        
                    } else {
                        // inland: just dirt/grass
                        terrain=Some("dirt");
                        // some varation
                        height += rng.gen::<usize>()%2 + rng.gen::<usize>()%2;
                    }
                    if terrain.is_some() {
                        self.map.hexes.insert(tile, vec![terrain.unwrap();height]);
                        
                    }
                }
            }
        }
        
        // add grass and trees
        for ring in 1..size {
            for tile in Hexpoint::new(ring,0).ring() {

                let hex = self.map.hexes.get(&tile);
                if hex.is_some() {		
                    let hex_id = hex.unwrap()[hex.unwrap().len()-1];
                    if hex_id=="dirt" {
                    
                        if ring<size/8 || hex.unwrap().len() > 1 || ! rng.gen_weighted_bool(5) {
                            self.map.decor.insert(tile, vec!["grass"]);
                        }
                    } else if hex_id=="sand" && hex.unwrap().len() < 4 {
                        if rng.gen_weighted_bool(8) {
                            self.map.decor.insert(tile, vec!["tree-palm"]);
                        }
                    }
                }
            }
        }
        
    }

    pub fn generate_debug(&mut self, size: i32) {
        self.size= size;
        self.map = Hexmap::new(self.size);
        
        for ring in 0..size/2 {
            for tile in Hexpoint::new(ring,0).ring() {
                let terrain;
                if ring==1 {
                    terrain=Some("stone");
                } else if ring==2 {
                    terrain=Some("dirt");                
                } else if ring==3 {
                    terrain=Some("sand");
                } else {
                    terrain = match (tile.x+tile.y).abs() % 3 {
                        0 => Some("stone"),
                        1 => Some("dirt"),
                        2 => Some("sand"),
                        _ => None,
                    };
                }
                if terrain.is_some() {
                    let height=1;
                    self.map.hexes.insert(tile, vec![terrain.unwrap();height]);
                }
            }
        }
        self.map.hexes.insert(Hexpoint::new(size/2-1,size/2-1), vec!["stone";1]);
        self.map.hexes.insert(Hexpoint::new(-(size/2-1),-(size/2-1)), vec!["sand";1]);

        self.map.hexes.insert(Hexpoint::new(size/2-1,-(size/2-1)), vec!["dirt";1]);
        self.map.hexes.insert(Hexpoint::new(-(size/2-1),size/2-1), vec!["dirt";1]);

    }
    
    pub fn get_ranked(&self, orientation: Direction) -> Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> {
        self.map.get_ranked(orientation)
    }
    
}
