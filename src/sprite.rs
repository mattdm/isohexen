//use sdl2::rect::Point;
use std::collections::HashMap;
use std::path;

use sdl2::render;
use sdl2::image::LoadTexture;

use direction::Direction;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sprite<'a> {
    pub id: &'a str,
    y_offset: usize,
    width: usize,
    height: usize,
}

impl<'a> Sprite<'a> {
    pub fn new(id: &'a str, y_offset: usize, width: usize, height: usize) -> Sprite<'a> {
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
    pub fn new(texture_creator: &'a render::TextureCreator<render::Texture>) -> SpriteAtlas<'a> {
        let mut s=SpriteAtlas {
            sprites: HashMap::new(),
            sprite_sheet: texture_creator.load_texture(path::Path::new("images/spritesheet.png")).unwrap(),
        };
        s.sprites.insert("grass",Sprite::new("stone",   0, 256, 160));
        s.sprites.insert("grass",Sprite::new("sand",  160, 256, 160));
        s.sprites.insert("grass",Sprite::new("dirt",  320, 256, 160));
        s.sprites.insert("grass",Sprite::new("grass", 480, 256, 160));
        s
    }
    
    pub fn draw(&self, canvas: &mut render::WindowCanvas, sprite_id: &str) {
        //canvas.copy(&self.sprite_sheet, Rect::new(texturecol*256,texturerow.unwrap()*160,256,160), Rect::new(center_x+offset.0*32,center_y+offset.1*24-elevation*8,64,40)).expect("Render failed");
    }
}
