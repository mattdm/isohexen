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

    pub fn generate(&mut self) {
        //self.size = 31; // FIXME: make this function scalable
        self.size= 32;
        self.map = Hexmap::new(self.size);
        
        let mut rng = rand::thread_rng();
        
        // center peak
        let center_tile = Hexpoint::new(0,0);
        let center_height = rng.gen::<isize>()%12+24; // FIXME: magic numbers!
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
                // change height -3 + random(0..5), max 1
                height = cmp::max(1,height + rng.gen::<isize>()%6 - 3);
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
        for ring in 1..15 { // FIXME: scale based on passed-in size parameter
            for tile in Hexpoint::new(ring,0).ring() {

                if self.map.hexes.get(&tile).is_none() {
                
                    let mut neighbor_height = 0;
                    let mut neighbor_count = 0;
                    let mut inner_sand = false;
                    let mut inner_ocean = 0;
                    
                    // get average height of inward neighbors
                    for neighbor in tile.inward_neighbors() {
                        match self.map.hexes.get(&neighbor) {
                            Some(neighbor_hex) => {
                                    neighbor_height += neighbor_hex.len();
                                    neighbor_count += 1;
                                    if neighbor_hex[0] == "sand" {
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
                                self.map.hexes.insert(tile, vec!["sand";rng.gen::<usize>()%2+1]);
                            }
                        } else if inner_ocean == 0 {
                            if ! rng.gen_weighted_bool(4) {
                                self.map.hexes.insert(tile, vec!["sand";rng.gen::<usize>()%2+1]);
                            }
                        }
                        
                    } else if ring  > 7 && (inner_sand || rng.gen_weighted_bool(16-ring as u32)) {
                        // chance of sand
                        self.map.hexes.insert(tile, vec!["sand";height+rng.gen::<usize>()%3]);
                    } else {
                        // inland: just dirt and grass
                        self.map.hexes.insert(tile, vec!["dirt";height]);
                    }
                    
                }
            }
        }
        
        // add grass and trees
        for ring in 1..15 { // FIXME: scale based on passed-in size parameter
            for tile in Hexpoint::new(ring,0).ring() {

                let hex = self.map.hexes.get(&tile);
                if hex.is_some() {		
                    let hex_id = hex.unwrap()[hex.unwrap().len()-1];
                    if hex_id=="dirt" {
                        self.map.decor.insert(tile, vec!["grass"]);
                    } else if hex_id=="sand" {
                        if rng.gen_weighted_bool(8) {
                            self.map.decor.insert(tile, vec!["tree-palm"]);
                        }
                    }
                }
            }
        }
        
    }
    
    pub fn get_ranked(&self, orientation: Direction) -> Vec<((i32,i32),Option<&Vec<&str>>,Option<&Vec<&str>>)> {
        self.map.get_ranked(orientation)
    }
    
}