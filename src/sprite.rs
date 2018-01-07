use sdl2::rect::Point;
use std::collections::HashMap;

#[derive(Clone,Copy,Debug)]
pub struct Sprite<'a> {
    pub id: &'a str,
    //row: usize,
    //size: Point,
}

impl Sprite {
    pub fn new(id: &str) -> Sprite {
        id,
    }
}


pub struct SpriteAtlas {
    pub sprite: HashMap<&str,Sprite>,
    sprite_sheet: &render::Texture,
}

impl SpriteAtlas {
    // FIXME: instead of hard-coding all this stuff, 
    // read from a description file
    pub fn new -> SpriteAtlas {
        let s = SpriteAtlas {
            sprite: HashMap::new();
            sprite_sheet: 
        }
    }
}