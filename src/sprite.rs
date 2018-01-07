//use sdl2::rect::Point;
use std::collections::HashMap;
use std::path;

use sdl2::render;

#[derive(Clone,Copy,Debug)]
pub struct Sprite<'a> {
    pub id: &'a str,
    //row: usize,
    //size: Point,
}

impl Sprite {
    pub fn new(id: &str) -> Sprite {
        id
    }
}


pub struct SpriteAtlas<'a> {
    pub sprite: HashMap<&'a str,Sprite>,
    sprite_sheet: &render::Texture,
}

impl SpriteAtlas {
    // FIXME: instead of hard-coding all this stuff, 
    // read from a description file
    pub fn new(canvas: &mut render::WindowCanvas) -> SpriteAtlas {
        SpriteAtlas {
            sprite: HashMap::new(),
            sprite_sheet: canvas.texture_creator().load_texture(path::Path::new("images/spritesheet.png")).unwrap(),
        }
    }
}