//use sdl2::rect::Point;
use std::collections::HashMap;
use std::path;

use sdl2::render;
use sdl2::image::LoadTexture;

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
    sprite_sheet: render::Texture<'a>,
}

impl<'a> SpriteAtlas<'a> {
    // FIXME: instead of hard-coding all this stuff, 
    // read from a description file
    pub fn new(texture_creator: &'a render::TextureCreator<render::Texture>) -> SpriteAtlas<'a> {
        SpriteAtlas {
            sprite: HashMap::new(),
            sprite_sheet: texture_creator.load_texture(path::Path::new("images/spritesheet.png")).unwrap(),
        }
    }
}
