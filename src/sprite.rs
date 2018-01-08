//use sdl2::rect::Point;
use std::collections::HashMap;
use std::path;

use sdl2::render;
use sdl2::video;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use direction::Direction;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sprite<'a> {
    pub id: &'a str,
    y_offset: i32,
    width: u32,
    height: u32,
}

impl<'a> Sprite<'a> {
    pub fn new(id: &'a str, y_offset: i32, width: u32, height: u32) -> Sprite<'a> {
        Sprite {
            id,
            y_offset,
            width,
            height,
        }
    }
}


pub struct SpriteAtlas<'a> {
    sprites: HashMap<&'a str,Sprite<'a>>,
    sprite_sheet: render::Texture<'a>,
}

impl<'a> SpriteAtlas<'a> {
    // FIXME: instead of hard-coding all this stuff, 
    // read from a description file
    //pub fn new(texture_creator: &'a render::TextureCreator<render::Texture>) -> SpriteAtlas<'a> {
    pub fn new(texture_creator: &'a render::TextureCreator<video::WindowContext>) -> SpriteAtlas<'a> {
        let mut s=SpriteAtlas {
            sprites: HashMap::new(),
            sprite_sheet: texture_creator.load_texture(path::Path::new("images/spritesheet.png")).unwrap(),
        };
        s.sprites.insert("stone",Sprite::new("stone",   0, 256, 160));
        s.sprites.insert("sand" ,Sprite::new("sand",  160, 256, 160));
        s.sprites.insert("dirt" ,Sprite::new("dirt",  320, 256, 160));
        s.sprites.insert("grass",Sprite::new("grass", 480, 256, 160));
        s
    }
    
    pub fn draw(&self, canvas: &mut render::WindowCanvas, sprite_id: &str, scale: u32, x: i32, y: i32, orientation: Direction) {
        let column = match orientation {
            Direction::E  => 0,
            Direction::SE => 1,
            Direction::SW => 2,
            Direction::W  => 3,
            Direction::NW => 4,
            Direction::NE => 5,
        };
        let s = self.sprites.get(sprite_id).unwrap(); //FIXME -- error handling if not found!
        canvas.copy(&self.sprite_sheet,
                    Rect::new(column*s.width as i32,s.y_offset,s.width,s.height),
                    Rect::new(x,y,s.width/scale,s.height/scale)
                   ).expect("Render failed");
    }
}
