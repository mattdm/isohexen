//use sdl2::rect::Point;
use std::collections::HashMap;

use sdl2::render;

#[derive(Clone,Copy,Debug)]
pub struct Sprite<'a> {
    pub id: &'a str,
    //row: usize,
    //size: Point,
}

impl<'a> Sprite<'a> {
    pub fn new(id: &str) -> Sprite {
        Sprite {
            id
        }
    }
}


pub struct SpriteAtlas<'a> {
    pub sprite: HashMap<&'a str,Sprite<'a>>,
    sprite_sheet: &'a render::Texture<'a>,
}

impl<'a> SpriteAtlas<'a> {
    // FIXME: instead of hard-coding all this stuff, 
    // read from a description file
    pub fn new(sprite_sheet: &'a render::Texture) -> SpriteAtlas<'a> {
        SpriteAtlas {
            sprite: HashMap::new(),
            sprite_sheet,
        }
    }
}