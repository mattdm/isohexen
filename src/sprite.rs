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

#[derive(Debug, Deserialize)]
struct SpriteAtlasConfig {
    spritesheet_png: Option<String>,
    atlas_format: Option<u64>,
    sprite: Option<Vec<Sprite>>,
}

pub struct SpriteAtlas<'a> {
    sprites: HashMap<String,Sprite>,
    sprite_sheet: render::Texture<'a>,
}

impl<'a> SpriteAtlas<'a> {

    pub fn new(texture_creator: &'a render::TextureCreator<video::WindowContext>) -> SpriteAtlas<'a> {
    
        let mut f = File::open("images/spritesheet.toml").expect("Sprite Sheet");

        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("Error reading sprite sheet");

        let atlas_config: SpriteAtlasConfig = toml::from_str(&buffer).unwrap();

        let mut a=SpriteAtlas {
            sprites: HashMap::new(),
            // FIXME: error handling!
            sprite_sheet: texture_creator.load_texture(path::Path::new(&atlas_config.spritesheet_png.unwrap())).unwrap(),
        };

        for s in atlas_config.sprite.unwrap() {
            //println!("Read sprite {:#?}", s.id);
            a.sprites.insert(s.id.clone(),s);
        }
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
    