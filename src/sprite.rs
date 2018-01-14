//use sdl2::rect::Point;
use std::collections::HashMap;
use std::path;
use std::fs::File;
use std::io::prelude::*;

use sdl2::render;
use sdl2::video;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use toml;


use direction::Direction;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Sprite {
    pub id: String,
    atlas_y: i32,
    draw_offset_x: i32,
    draw_offset_y: i32,
    width: u32,
    height: u32,
}

impl Sprite {
    pub fn new(id: &str, atlas_y: i32, draw_offset_x: i32, draw_offset_y: i32, width: u32, height: u32) -> Sprite {
        Sprite {
            id: id.to_string(),
            atlas_y,
            draw_offset_x,
            draw_offset_y,
            width,
            height,
        }
    }

}

#[derive(Debug, Deserialize)]
struct SpriteAtlasConfig {
    sprites_png: Option<String>,
    atlas_format: Option<u64>,
    sprite: Option<Vec<Sprite>>,
}

pub struct SpriteAtlas<'a> {
    sprites: HashMap<String,Sprite>,
    sprite_sheet: render::Texture<'a>,
}

impl<'a> SpriteAtlas<'a> {
    // FIXME: instead of hard-coding all this stuff, 
    // read from a description file
    //pub fn new(texture_creator: &'a render::TextureCreator<render::Texture>) -> SpriteAtlas<'a> {
    pub fn new(texture_creator: &'a render::TextureCreator<video::WindowContext>) -> SpriteAtlas<'a> {
    
        
        let mut f = File::open("images/spritesheet.toml").expect("Sprite Sheet");

        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("Error reading sprite sheet");

        let atlas_config: SpriteAtlasConfig = toml::from_str(&buffer).unwrap();

        let mut a=SpriteAtlas {
            sprites: HashMap::new(),
            sprite_sheet: texture_creator.load_texture(path::Path::new("images/spritesheet.png")).unwrap(),
        };

        
        for s in atlas_config.sprite.unwrap() {
            println!("S {:#?}", s.id);
            //a.sprites.insert(&s.id[..],s);
        }
    
        a.sprites.insert(String::from("stone"),Sprite::new("stone",   0, 0, 0, 256, 160));
        a.sprites.insert(String::from("sand"),Sprite::new("sand",  160, 0, 0, 256, 160));
        a.sprites.insert(String::from("dirt"),Sprite::new("dirt",  320, 0, 0, 256, 160));
        a.sprites.insert(String::from("grass"),Sprite::new("grass", 480, 0, 0, 256, 160));
        a.sprites.insert(String::from("tree-palm"),Sprite::new("tree-palm", 640, 16, -160, 256, 256));
        a.sprites.insert(String::from("compass"),Sprite::new("compass", 1536, 0, 0, 256, 96));
        a
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
                    Rect::new(column*s.width as i32,s.atlas_y,s.width,s.height),
                    Rect::new(x+s.draw_offset_x/scale as i32,y+s.draw_offset_y/scale as i32,s.width/scale,s.height/scale)
                   ).expect("Render failed");
    }
}
    